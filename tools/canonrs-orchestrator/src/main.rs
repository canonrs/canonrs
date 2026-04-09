//! CanonRS Dev Orchestrator
//! Responsabilidade única: watch + build + copy de todos os interaction crates + leptos watch

use std::process::{Command, Stdio};
use std::path::PathBuf;

const GROUPS: &[&str] = &["gesture", "overlay", "selection", "nav", "data", "content"];

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

    // build inicial de todos os grupos
    for group in GROUPS {
        build_group(&root, group);
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
