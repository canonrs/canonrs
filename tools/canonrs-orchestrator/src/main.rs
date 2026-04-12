//! CanonRS Orchestrator — build system
//! Responsabilidade: compilar 1 WASM + pipeline de assets

use std::process::Command;
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../..")
        .canonicalize()
        .unwrap()
}

fn build_wasm(root: &PathBuf) {
    let crate_path = root.join("packages-rust/rs-canonrs/canonrs-interactions");
    let out_dir    = crate_path.join("dist");
    let dest       = root.join("packages-rust/rs-canonrs/canonrs-client/assets/wasm");

    std::fs::create_dir_all(&dest).ok();

    let release = std::env::var("CANON_RELEASE").is_ok();
    let mut args = vec![
        "build", crate_path.to_str().unwrap(),
        "--target", "web",
        "--out-dir", out_dir.to_str().unwrap(),
    ];
    if release { args.push("--release"); } else { args.push("--dev"); }

    let status = Command::new("wasm-pack").args(&args).status();

    match status {
        Ok(s) if s.success() => {
            // Copia .wasm e .js para assets/wasm/
            for entry in std::fs::read_dir(&out_dir).unwrap().filter_map(|e| e.ok()) {
                let name = entry.file_name();
                let name = name.to_str().unwrap();
                if name.ends_with(".d.ts") { continue; }
                if name.ends_with(".wasm") || name.ends_with(".js") {
                    let dest_file = dest.join(name);
                    std::fs::copy(entry.path(), &dest_file).ok();

                    // wasm-opt em release
                    if release && name.ends_with(".wasm") {
                        let opt_out = dest.join(format!("{}.opt.wasm", &name[..name.len()-5]));
                        let ok = Command::new("wasm-opt")
                            .args(["-Oz", "--enable-bulk-memory",
                                   dest_file.to_str().unwrap(),
                                   "-o", opt_out.to_str().unwrap()])
                            .status()
                            .map(|s| s.success())
                            .unwrap_or(false);
                        if ok {
                            std::fs::rename(&opt_out, &dest_file).ok();
                            println!("[canon] wasm-opt: {}", name);
                        }
                        // brotli + gzip
                        Command::new("gzip").args(["-9", "-k", "-f", dest_file.to_str().unwrap()]).status().ok();
                        Command::new("brotli").args(["-9", "-f", "-k", dest_file.to_str().unwrap()]).status().ok();
                    }
                }
            }
            println!("[canon] built: canonrs-interactions");
        }
        _ => eprintln!("[canon] failed: canonrs-interactions"),
    }
}

fn copy_loaders(root: &PathBuf) {
    let version = env!("CARGO_PKG_VERSION");
    let src_dir  = root.join("packages-rust/rs-canonrs/canonrs-client/src/loader");
    let dest_dir = root.join("packages-rust/rs-canonrs/canonrs-client/assets/js");
    std::fs::create_dir_all(&dest_dir).ok();

    for loader in &["canon-loader.js", "canonrs.bundle.js"] {
        let src = src_dir.join(loader);
        if src.exists() {
            let content = std::fs::read_to_string(&src).unwrap_or_default();
            let content = content.replace("__CANONRS_VERSION__", version);
            std::fs::write(dest_dir.join(loader), content).ok();
            println!("[canon] copied loader: {}", loader);
        }
    }
}

fn main() {
    let root = root();
    println!("🚀 CanonRS Orchestrator");

    copy_loaders(&root);
    build_wasm(&root);
}
