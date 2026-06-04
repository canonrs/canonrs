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
    let release = std::env::var("CANON_RELEASE").is_ok();
    let wasm_opt = std::env::var("LEPTOS_WASM_OPT_VERSION")
        .unwrap_or_else(|_| "version_118".to_string());

    if release {
        println!("[canon][leptos] building release — project: {}", project);
        state.lock().unwrap().leptos = "BUILDING (release)".into();

        // build release com wasm-opt e gzip
        let status = Command::new("cargo")
            .args(["leptos", "build", "--release", "--project", project])
            .current_dir(root)
            .env("CANON_ROOT", root)
            .env("LEPTOS_WASM_OPT_VERSION", &wasm_opt)
            .status()
            .expect("cargo leptos not found");

        if status.success() {
            // gzip do wasm principal
            let pkg_dir = root.join("target/site/pkg");
            if pkg_dir.exists() {
                for entry in std::fs::read_dir(&pkg_dir).unwrap().filter_map(|e| e.ok()) {
                    let p = entry.path();
                    if p.extension().map(|e| e == "wasm" || e == "js").unwrap_or(false) {
                        Command::new("gzip")
                            .args(["-kf", p.to_str().unwrap()])
                            .status().ok();
                    }
                }
                println!("[canon][leptos] gzip done");
            }
            state.lock().unwrap().leptos = "OK (release)".into();
        } else {
            eprintln!("[canon][leptos] release build FAILED");
            state.lock().unwrap().leptos = "FAILED".into();
        }

        // serve em modo release
        Command::new("cargo")
            .args(["leptos", "serve", "--project", project])
            .current_dir(root)
            .env("CANON_ROOT", root)
            .spawn()
            .expect("cargo leptos not found")
    } else {
        println!("[canon][leptos] starting dev — project: {}", project);
        state.lock().unwrap().leptos = "RUNNING".into();
        let mut args = vec!["leptos", "watch", "--project", project];
        let extra_features = std::env::var("CANON_FEATURES").unwrap_or_default();
        if !extra_features.is_empty() {
            args.push("--lib-features");
            args.push(Box::leak(extra_features.into_boxed_str()));
        }
        let leptos_workspace = std::env::var("LEPTOS_WORKSPACE")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| root.clone());
        Command::new("cargo")
            .args(&args)
            .current_dir(&leptos_workspace)
            .env("CANON_ROOT", root)
            .spawn()
            .expect("cargo leptos not found")
    }
}
