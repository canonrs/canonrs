//! CanonRS Dev Orchestrator
//! Responsabilidade única: watch + build + copy de todos os interaction crates + leptos watch

use std::process::{Command, Stdio};
use std::path::PathBuf;

const GROUPS: &[&str] = &["gesture", "overlay", "selection", "nav", "data", "content", "init"];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../..")
        .canonicalize()
        .unwrap()
}

fn build_group(root: &PathBuf, group: &str) {
    let crate_path = root.join(format!("packages-rust/rs-canonrs/canonrs-interactions-{}", group));
    let out_dir    = crate_path.join(format!("dist/{}", group));
    let dest       = root.join(format!("packages-rust/rs-canonrs/canonrs-server/assets/wasm/{}", group));

    std::fs::create_dir_all(&dest).ok();

    let status = Command::new("wasm-pack")
        .args(["build", crate_path.to_str().unwrap(), "--target", "web",
               "--out-dir", out_dir.to_str().unwrap(), "--dev"])
        .status();

    match status {
        Ok(s) if s.success() => {
            for entry in std::fs::read_dir(&out_dir).unwrap().filter_map(|e| e.ok()) {
                let name = entry.file_name();
                let name = name.to_str().unwrap();
                if name.ends_with(".wasm") || (name.ends_with(".js") && !name.ends_with(".d.ts")) {
                    std::fs::copy(entry.path(), dest.join(name)).ok();
                }
            }
            println!("[canon] built: {}", group);
        }
        _ => eprintln!("[canon] failed: {}", group),
    }
}

fn generate_bundle_zip(root: &PathBuf) {
    let version = env!("CARGO_PKG_VERSION");
    let bundle_dir = root.join(format!("dist/canonrs-{}", version));
    let wasm_dir = bundle_dir.join("wasm");
    std::fs::create_dir_all(&wasm_dir).ok();

    // copia wasm assets
    let assets = root.join("packages-rust/rs-canonrs/canonrs-server/assets/wasm");
    if let Ok(groups) = std::fs::read_dir(&assets) {
        for group in groups.filter_map(|e| e.ok()) {
            let group_dest = wasm_dir.join(group.file_name());
            std::fs::create_dir_all(&group_dest).ok();
            if let Ok(files) = std::fs::read_dir(group.path()) {
                for file in files.filter_map(|e| e.ok()) {
                    std::fs::copy(file.path(), group_dest.join(file.file_name())).ok();
                }
            }
        }
    }

    // copia bundle js e manifest
    let public_js = root.join("products/canonrs-site/public/js");
    std::fs::copy(public_js.join("canonrs.bundle.js"), bundle_dir.join("canonrs.bundle.js")).ok();
    std::fs::copy(
        root.join("products/canonrs-site/public/canonrs-manifest.json"),
        bundle_dir.join("manifest.json")
    ).ok();

    // gera tar.gz
    let archive = root.join(format!("dist/canonrs-{}.tar.gz", version));
    Command::new("tar")
        .args(["-czf", archive.to_str().unwrap(), "-C",
               root.join("dist").to_str().unwrap(),
               &format!("canonrs-{}", version)])
        .status()
        .ok();

    println!("[canon] bundle: dist/canonrs-{}.tar.gz", version);
}

fn generate_manifest(root: &PathBuf) {
    let version = env!("CARGO_PKG_VERSION");
    let mut groups = String::new();
    for (i, group) in GROUPS.iter().enumerate() {
        let comma = if i < GROUPS.len() - 1 { "," } else { "" };
        groups.push_str(&format!(
            r#"    "{}": {{
      "js": "/wasm/{}/canonrs_interactions_{}.js",
      "wasm": "/wasm/{}/canonrs_interactions_{}_bg.wasm"
    }}{}"#,
            group, group, group, group, group, comma
        ));
        groups.push('\n');
    }
    let manifest = format!(
        r#"{{
  "version": "{}",
  "groups": {{
{}}}
}}"#,
        version, groups
    );
    let dest = root.join("products/canonrs-site/public/canonrs-manifest.json");
    std::fs::write(&dest, manifest).ok();
    println!("[canon] manifest generated");
}

fn copy_loaders(root: &PathBuf) {
    let src_dir = root.join("packages-rust/rs-canonrs/canonrs-client/src/loader");
    let dest_dir = root.join("products/canonrs-site/public/js");
    std::fs::create_dir_all(&dest_dir).ok();
    let loaders = ["canon-loader.js", "canonrs.bundle.js"];
    let version = env!("CARGO_PKG_VERSION");
    for loader in &loaders {
        let src = src_dir.join(loader);
        let dst = dest_dir.join(loader);
        if src.exists() {
            let content = std::fs::read_to_string(&src).unwrap_or_default();
            let content = content.replace("__CANONRS_VERSION__", version);
            std::fs::write(&dst, content).ok();
            println!("[canon] copied loader: {}", loader);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // modo build <group> — chamado pelo watcher
    if args.len() == 3 && args[1] == "build" {
        let root = root();
        build_group(&root, &args[2]);
        return;
    }

    let root = root();
    println!("🚀 CanonRS Orchestrator");

    // copia loaders JS para public/js
    copy_loaders(&root);

    // build inicial de todos os grupos
    for group in GROUPS {
        build_group(&root, group);
    }

    // gera manifest.json
    generate_manifest(&root);

    // gera bundle zip se --release
    if args.iter().any(|a| a == "--release") {
        generate_bundle_zip(&root);
    }

    // spawn watcher para cada grupo
    for group in GROUPS {
        let watch_path = root
            .join(format!("packages-rust/rs-canonrs/canonrs-interactions-{}/src", group));

        Command::new("cargo")
            .args([
                "watch",
                "-w", watch_path.to_str().unwrap(),
                "-s", &format!("cargo run --manifest-path {}/packages-rust/rs-canonrs/Cargo.toml -p canonrs-orchestrator -- build {}", root.display(), group),
            ])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();
    }

    // rodar leptos watch (blocking)
    let site_dir = root.join("products/canonrs-site");
    Command::new("cargo")
        .args(["leptos", "watch", "--project", "canonrs-site"])
        .current_dir(&site_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
