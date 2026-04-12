//! build.rs — CanonRS Build Pipeline
//! 1. Copia CSS
//! 2. Copia loader JS
//! 3. Compila 1 WASM (canonrs-interactions)

use std::process::Command;
use std::path::PathBuf;
use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let rs_canonrs   = manifest_dir.parent().unwrap();
    let out_dir      = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let assets_js    = rs_canonrs.join("canonrs-client/assets/js");
    let assets_wasm  = rs_canonrs.join("canonrs-client/assets/wasm");
    let version      = env!("CARGO_PKG_VERSION");

    fs::create_dir_all(&assets_js).ok();
    fs::create_dir_all(&assets_wasm).ok();

    // 1. CSS
    let css_src = rs_canonrs.join("canonrs-server/styles/canonrs.bundle.css");
    if css_src.exists() {
        fs::copy(&css_src, out_dir.join("canonrs.css")).expect("failed to copy css");
        println!("cargo:warning=[canon] css copied");
    }
    println!("cargo:rerun-if-changed={}", css_src.display());

    // 2. Loader JS
    let loader_dir = rs_canonrs.join("canonrs-client/src/loader");
    for loader in &["canon-loader.js", "canonrs.bundle.js"] {
        let src = loader_dir.join(loader);
        if src.exists() {
            let content = fs::read_to_string(&src).unwrap_or_default();
            let content = content.replace("__CANONRS_VERSION__", version);
            fs::write(assets_js.join(loader), content).ok();
        }
        println!("cargo:rerun-if-changed={}", loader_dir.join(loader).display());
    }
    println!("cargo:warning=[canon] loader copied");

    // 3. WASM — skip se CANON_SKIP_WASM
    if std::env::var("CANON_SKIP_WASM").is_ok() {
        println!("cargo:warning=[canon] skipping wasm (CANON_SKIP_WASM set)");
        return;
    }

    let crate_path = rs_canonrs.join("canonrs-interactions");
    let dist       = crate_path.join("dist");
    let wasm_file  = assets_wasm.join("canonrs_interactions_bg.wasm");
    let src_dir    = crate_path.join("src");

    // skip se up-to-date
    if wasm_file.exists() {
        let wasm_mtime = fs::metadata(&wasm_file).unwrap().modified().unwrap();
        let src_mtime  = fs::read_dir(&src_dir).unwrap()
            .filter_map(|e| e.ok())
            .filter_map(|e| e.metadata().ok()?.modified().ok())
            .max();
        if let Some(src_mtime) = src_mtime {
            if wasm_mtime >= src_mtime {
                println!("cargo:warning=[canon] wasm up-to-date");
                return;
            }
        }
    }

    let release = std::env::var("PROFILE").unwrap_or_default() == "release";
    let mut args = vec![
        "build", crate_path.to_str().unwrap(),
        "--target", "web",
        "--out-dir", dist.to_str().unwrap(),
    ];
    if release { args.push("--release"); } else { args.push("--dev"); }

    let status = Command::new("wasm-pack").args(&args)
        .status()
        .expect("wasm-pack not found");

    if !status.success() {
        println!("cargo:warning=[canon] wasm build failed");
        return;
    }

    // Copia .wasm e .js para assets/wasm/
    for entry in fs::read_dir(&dist).unwrap().filter_map(|e| e.ok()) {
        let name = entry.file_name();
        let name = name.to_str().unwrap();
        if name.ends_with(".d.ts") { continue; }
        if name.ends_with(".wasm") || name.ends_with(".js") {
            fs::copy(entry.path(), assets_wasm.join(name)).ok();
        }
    }

    println!("cargo:warning=[canon] wasm built");
}
