//! build.rs — CanonRS WASM Bundle Generator
//! Gera os WASM bundles de interação durante `cargo build`.
//! OUTPUT: OUT_DIR/wasm/<group>/

use std::process::Command;
use std::path::PathBuf;
use std::fs;

fn main() {
    // declarar antes do return para o Cargo usar cache corretamente
    println!("cargo:rerun-if-changed=build.rs");

    // copiar canonrs.bundle.css para OUT_DIR/canonrs.css
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let rs_canonrs   = manifest_dir.parent().unwrap();
    let css_src      = rs_canonrs.join("canonrs-server/styles/canonrs.bundle.css");
    let out_dir      = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    if css_src.exists() {
        fs::copy(&css_src, out_dir.join("canonrs.css")).expect("failed to copy canonrs.css");
        println!("cargo:warning=[canon] css copied");
    }
    println!("cargo:rerun-if-changed={}", css_src.display());

    // copiar canon-loader.js sempre (mesmo em dev)
    let manifest_dir_early = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let rs_canonrs_early   = manifest_dir_early.parent().unwrap();
    let loader_src_early   = rs_canonrs_early.join("canonrs-client/src/loader/canon-loader.js");
    let site_js_early      = rs_canonrs_early.parent().unwrap().parent().unwrap()
        .join("packages-rust/rs-canonrs/canonrs-client/assets/js");
    fs::create_dir_all(&site_js_early).ok();
    fs::copy(&loader_src_early, site_js_early.join("canon-loader.js")).ok();
    // copia canonrs.bundle.js
    let bundle_src_early = rs_canonrs_early.join("canonrs-client/src/loader/canonrs.bundle.js");
    if bundle_src_early.exists() {
        let bundle_content = fs::read_to_string(&bundle_src_early).unwrap_or_default();
        let version = env!("CARGO_PKG_VERSION");
        let bundle_content = bundle_content.replace("__CANONRS_VERSION__", version);
        fs::write(site_js_early.join("canonrs.bundle.js"), bundle_content).ok();
    }
    println!("cargo:rerun-if-changed={}", loader_src_early.display());
    let bundle_src_changed = rs_canonrs_early.join("canonrs-client/src/loader/canonrs.bundle.js");
    println!("cargo:rerun-if-changed={}", bundle_src_changed.display());

    // CANON_SKIP_WASM pode ser usado para pular explicitamente
    if std::env::var("CANON_SKIP_WASM").is_ok() {
        println!("cargo:warning=[canon] skipping wasm-pack (CANON_SKIP_WASM set)");
        return;
    }
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let rs_canonrs   = manifest_dir.parent().unwrap();
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("wasm");

    fs::create_dir_all(&out_dir).expect("failed to create OUT_DIR/wasm");

    let groups = ["gesture", "overlay", "selection", "nav", "data", "content", "init"];

    for group in &groups {
        let crate_path = rs_canonrs.join(format!("canonrs-interactions-{}", group));


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

    // gera manifest.json
    generate_manifest(rs_canonrs, &groups[..]);

    // gera bundle zip em release
    if std::env::var("PROFILE").unwrap_or_default() == "release" {
        generate_bundle_zip(rs_canonrs, &groups[..]);
    }

    // copiar canon-loader.js para public/js do site
    let loader_src = rs_canonrs.join("canonrs-client/src/loader/canon-loader.js");
    let site_js    = rs_canonrs.parent().unwrap().parent().unwrap()
        .join("packages-rust/rs-canonrs/canonrs-client/assets/js");
    fs::create_dir_all(&site_js).expect("failed to create public/js");
    fs::copy(&loader_src, site_js.join("canon-loader.js"))
        .expect("failed to copy canon-loader.js");
    println!("cargo:warning=[canon] loader copied");
    println!("cargo:rerun-if-changed={}", loader_src.display());

}

fn generate_bundle_zip(rs_canonrs: &std::path::Path, groups: &[&str]) {
    let version = env!("CARGO_PKG_VERSION");
    let dist_root = rs_canonrs.parent().unwrap().parent().unwrap().join("dist");
    let bundle_dir = dist_root.join(format!("canonrs-{}", version));
    let wasm_dir = bundle_dir.join("wasm");
    fs::create_dir_all(&wasm_dir).ok();

    // copia wasm assets
    let assets = rs_canonrs.parent().unwrap().parent().unwrap()
        .join("packages-rust/rs-canonrs/canonrs-client/assets/wasm");
    for group in groups {
        let src = assets.join(group);
        let dst = wasm_dir.join(group);
        fs::create_dir_all(&dst).ok();
        if let Ok(entries) = fs::read_dir(&src) {
            for entry in entries.filter_map(|e| e.ok()) {
                fs::copy(entry.path(), dst.join(entry.file_name())).ok();
            }
        }
    }

    // copia bundle js e manifest
    let site = rs_canonrs.parent().unwrap().parent().unwrap()
        .join("products/canonrs-site/public");
    fs::copy(site.join("js/canonrs.bundle.js"), bundle_dir.join("canonrs.bundle.js")).ok();
    fs::copy(site.join("canonrs-manifest.json"), bundle_dir.join("manifest.json")).ok();

    // gera tar.gz
    let archive = dist_root.join(format!("canonrs-{}.tar.gz", version));
    std::process::Command::new("tar")
        .args(["-czf", archive.to_str().unwrap(), "-C",
               dist_root.to_str().unwrap(),
               &format!("canonrs-{}", version)])
        .status().ok();

    println!("cargo:warning=[canon] bundle: dist/canonrs-{}.tar.gz", version);
}

fn generate_manifest(rs_canonrs: &std::path::Path, groups: &[&str]) {
    let version = env!("CARGO_PKG_VERSION");
    let mut entries = String::new();
    for (i, group) in groups.iter().enumerate() {
        let comma = if i < groups.len() - 1 { "," } else { "" };
        entries.push_str(&format!(
            "    \"{g}\": {{\n      \"js\": \"/wasm/{g}/canonrs_interactions_{g}.js\",\n      \"wasm\": \"/wasm/{g}/canonrs_interactions_{g}_bg.wasm\"\n    }}{c}\n",
            g = group, c = comma
        ));
    }
    let manifest = format!(
        "{{\n  \"version\": \"{}\",\n  \"groups\": {{\n{}  }}\n}}",
        version, entries
    );
    let dest = rs_canonrs.parent().unwrap().parent().unwrap()
        .join("products/canonrs-site/public/js/canonrs-manifest.json");
    std::fs::write(&dest, manifest).ok();
    println!("cargo:warning=[canon] manifest generated");
}
