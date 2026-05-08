//! Pipeline — sequência de inicialização do CanonRS

use std::path::PathBuf;
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use crate::state::SystemState;

pub fn spawn_tokens(root: &PathBuf, state: &Arc<Mutex<SystemState>>) {
    let tokens_dir = root.join("packages-rust/rs-canonrs/canonrs-tokens");
    let t = Instant::now();
    println!("[canon][tokens] building...");
    Command::new("cargo")
        .args(["run", "--bin", "tokens-engine"])
        .current_dir(&tokens_dir)
        .env("CARGO_TARGET_DIR", "/tmp/tokens-build")
        .status().ok();
    let elapsed = t.elapsed().as_millis();
    println!("[canon][tokens] done ({}ms)", elapsed);
    state.lock().unwrap().tokens = format!("OK ({}ms)", elapsed);
}

pub fn copy_loaders(root: &PathBuf) {
    let version  = env!("CARGO_PKG_VERSION");
    let src_dir  = root.join("packages-rust/rs-canonrs/canonrs-client/src/loader");
    let dest_dir = root.join("packages-rust/rs-canonrs/canonrs-client/assets/js");
    std::fs::create_dir_all(&dest_dir).ok();
    for loader in &["canon-loader.js", "canonrs.bundle.js"] {
        let src = src_dir.join(loader);
        if src.exists() {
            let content = std::fs::read_to_string(&src).unwrap_or_default();
            let content = content.replace("__CANONRS_VERSION__", version);
            std::fs::write(dest_dir.join(loader), content).ok();
        }
    }
    println!("[canon][loaders] ready");
}

pub fn build_css(root: &PathBuf) {
    let site_dir = root.join("products/canonrs-site");
    if !site_dir.exists() { return; }
    let t = Instant::now();
    println!("[canon][css] building...");
    let status = Command::new("npm")
        .args(["run", "build:css"])
        .current_dir(&site_dir)
        .status();
    match status {
        Ok(s) if s.success() => println!("[canon][css] done ({}ms)", t.elapsed().as_millis()),
        _ => eprintln!("[canon][css] FAILED"),
    }
}

pub fn spawn_leptos(root: &PathBuf, project: &str, state: &Arc<Mutex<SystemState>>) -> Child {
    println!("[canon][leptos] starting — project: {}", project);
    state.lock().unwrap().leptos = "RUNNING".into();
    let mut args = vec!["leptos", "watch", "--project", project];
    let extra_features = std::env::var("CANON_FEATURES").unwrap_or_default();
    if !extra_features.is_empty() {
        args.push("--lib-features");
        args.push(Box::leak(extra_features.into_boxed_str()));
    }
    Command::new("cargo")
        .args(&args)
        .current_dir(root)
        .env("CANON_ROOT", root)
        .spawn()
        .expect("cargo leptos not found")
}
