//! build.rs — CanonRS WASM Bundle Generator
//! Gera os WASM bundles de interação durante `cargo build`.
//! OUTPUT: OUT_DIR/wasm/<group>/

use std::process::Command;
use std::path::PathBuf;
use std::fs;

fn main() {
    // declarar antes do return para o Cargo usar cache corretamente
    println!("cargo:rerun-if-changed=build.rs");

    // Em modo dev, pular wasm-pack para acelerar o build
    if std::env::var("LEPTOS_WATCH").is_ok() || std::env::var("CANON_SKIP_WASM").is_ok() {
        println!("cargo:warning=[canon] skipping wasm-pack in dev/watch mode");
        return;
    }
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let rs_canonrs   = manifest_dir.parent().unwrap();
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("wasm");

    fs::create_dir_all(&out_dir).expect("failed to create OUT_DIR/wasm");

    let groups = ["gesture", "overlay", "selection", "nav", "data", "content"];

    for group in &groups {
        let crate_path = rs_canonrs.join(format!("canonrs-interactions-{}", group));

        println!("cargo:rerun-if-changed={}", crate_path.join("src").display());

        let dist = crate_path.join(format!("dist/{}", group));
        let dest = out_dir.join(group);
        fs::create_dir_all(&dest).expect("failed to create dest dir");

        // skip se wasm já existe e é mais novo que o src
        let wasm_file = dest.join(format!("canonrs_interactions_{}_bg.wasm", group.replace('-', "_")));
        let src_dir = crate_path.join("src");
        if wasm_file.exists() {
            let wasm_mtime = fs::metadata(&wasm_file).unwrap().modified().unwrap();
            let src_mtime = fs::read_dir(&src_dir).unwrap()
                .filter_map(|e| e.ok())
                .filter_map(|e| e.metadata().ok()?.modified().ok())
                .max();
            if let Some(src_mtime) = src_mtime {
                if wasm_mtime >= src_mtime {
                    println!("cargo:warning=[canon] wasm up-to-date: {}", group);
                    continue;
                }
            }
        }

        let status = Command::new("wasm-pack")
            .args([
                "build",
                crate_path.to_str().unwrap(),
                "--target", "web",
                "--out-dir", dist.to_str().unwrap(),
                "--release",
            ])
            .status()
            .expect("wasm-pack not found — install with: cargo install wasm-pack");

        if !status.success() {
            println!("cargo:warning=[canon] wasm-pack failed for {}", group);
            continue;
        }

        for entry in fs::read_dir(&dist).unwrap() {
            let entry = entry.unwrap();
            let name  = entry.file_name();
            let name  = name.to_str().unwrap();
            if name.ends_with(".wasm") || (name.ends_with(".js") && !name.ends_with(".d.ts")) {
                fs::copy(entry.path(), dest.join(name)).unwrap();
            }
        }

        println!("cargo:warning=[canon] wasm built: {}", group);
    }

}
