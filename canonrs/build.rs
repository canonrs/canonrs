//! build.rs — CanonRS Build Pipeline
//! 1. Copia CSS
//! 2. Copia loader JS
//! 3. Compila 1 WASM (canonrs-interactions)

use std::process::Command;
use std::path::PathBuf;
use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CANON_SKIP_WASM");

    // Watch interaction crates
    let manifest_dir_watch = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let rs_canonrs_watch   = manifest_dir_watch.parent().unwrap();
    for crate_name in &[
        "canonrs-interactions",
        "canonrs-interactions-init",
        "canonrs-interactions-nav",
        "canonrs-interactions-data",
        "canonrs-interactions-gesture",
        "canonrs-interactions-overlay",
        "canonrs-interactions-selection",
        "canonrs-interactions-content",
    ] {
        let src = rs_canonrs_watch.join(crate_name).join("src");
        println!("cargo:rerun-if-changed={}", src.display());
    }

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

    // 3. WASM — gerenciado pelo canonrs-orchestrator
    // wasm-pack e chamado pelo orchestrator em runtime, nao no build script
    println!("cargo:warning=[canon] wasm managed by orchestrator");
}
