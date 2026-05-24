//! Watchers — wasm, core, loader

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use notify::{Watcher, RecursiveMode, recommended_watcher, Event};
use tokio::sync::broadcast;
use crate::config::{WASM_CRATES, CORE_WATCH_DIRS, WASM_DEBOUNCE_MS, CORE_DEBOUNCE_MS, INTERACTION_GROUPS};
use crate::state::SystemState;
use crate::wasm::{build_wasm, build_group};

pub fn spawn_wasm_watcher(
    root: &PathBuf,
    running: Arc<AtomicBool>,
    state: Arc<Mutex<SystemState>>,
    reload_tx: broadcast::Sender<()>,
) -> std::thread::JoinHandle<()> {
    let root = root.clone();
    let watch_dirs: Vec<PathBuf> = WASM_CRATES.iter()
        .map(|d| root.join("packages-rust/rs-canonrs").join(d).join("src"))
        .collect();

    std::thread::spawn(move || {
        let (tx, rx) = std::sync::mpsc::channel::<notify::Result<Event>>();
        let mut watcher = recommended_watcher(tx).expect("watcher failed");
        for dir in &watch_dirs {
            if dir.exists() { watcher.watch(dir, RecursiveMode::Recursive).ok(); }
        }
        println!("[canon][wasm-watcher] watching {} crates", watch_dirs.len());

        let mut last_build = Instant::now();
        while running.load(Ordering::Relaxed) {
            match rx.recv_timeout(std::time::Duration::from_millis(100)) {
                Ok(Ok(event)) => {
                    let is_rs = event.paths.iter()
                        .any(|p| p.extension().map(|e| e == "rs").unwrap_or(false));
                    if is_rs && last_build.elapsed().as_millis() > WASM_DEBOUNCE_MS {
                        for p in &event.paths {
                            println!("[canon][wasm-watcher] changed: {}",
                                p.file_name().unwrap_or_default().to_str().unwrap_or("?"));
                        }
                        last_build = Instant::now();
                        // Detect which group changed for targeted rebuild
                        let changed_group = event.paths.iter().find_map(|p| {
                            let path_str = p.to_string_lossy();
                            INTERACTION_GROUPS.iter().find(|g| {
                                path_str.contains(&format!("canonrs-interactions-{}", g))
                            }).map(|g| *g)
                        });
                        if let Some(group) = changed_group {
                            println!("[canon][wasm-watcher] targeted rebuild: {}", group);
                            if build_group(&root, group) {
                                reload_tx.send(()).ok();
                            }
                        } else {
                            build_wasm(&root, &state, &reload_tx);
                        }
                    }
                }
                _ => {}
            }
        }
    })
}

pub fn spawn_core_watcher(
    root: &PathBuf,
    running: Arc<AtomicBool>,
) -> std::thread::JoinHandle<()> {
    let root = root.clone();
    let watch_dirs: Vec<PathBuf> = CORE_WATCH_DIRS.iter()
        .map(|d| root.join(d))
        .collect();

    std::thread::spawn(move || {
        let (tx, rx) = std::sync::mpsc::channel::<notify::Result<Event>>();
        let mut watcher = recommended_watcher(tx).expect("watcher failed");
        for dir in &watch_dirs {
            if dir.exists() { watcher.watch(dir, RecursiveMode::Recursive).ok(); }
        }
        println!("[canon][core-watcher] watching {} dirs", watch_dirs.len());

        let mut last_build = Instant::now();
        while running.load(Ordering::Relaxed) {
            match rx.recv_timeout(std::time::Duration::from_millis(200)) {
                Ok(Ok(event)) => {
                    let is_yaml_or_rs = event.paths.iter().any(|p| {
                        p.extension().map(|e| e == "yaml" || e == "rs").unwrap_or(false)
                    });
                    if is_yaml_or_rs && last_build.elapsed().as_millis() > CORE_DEBOUNCE_MS {
                        for p in &event.paths {
                            println!("[canon][core-watcher] changed: {}",
                                p.file_name().unwrap_or_default().to_str().unwrap_or("?"));
                        }
                        last_build = Instant::now();
                        let build_rs = root.join("packages-rust/rs-canonrs/canonrs-core/build.rs");
                        if let Ok(c) = std::fs::read_to_string(&build_rs) {
                            std::fs::write(&build_rs, c).ok();
                        }
                        println!("[canon][core-watcher] build.rs touched — leptos will recompile");
                    }
                }
                _ => {}
            }
        }
    })
}

pub fn spawn_loader_watcher(
    root: &PathBuf,
    running: Arc<AtomicBool>,
) -> std::thread::JoinHandle<()> {
    let root     = root.clone();
    let src_dir  = root.join("packages-rust/rs-canonrs/canonrs-client/src/loader");
    let dest_dir = root.join("packages-rust/rs-canonrs/canonrs-client/assets/js");
    let version  = env!("CARGO_PKG_VERSION").to_string();

    std::thread::spawn(move || {
        let (tx, rx) = std::sync::mpsc::channel::<notify::Result<Event>>();
        let mut watcher = recommended_watcher(tx).expect("watcher failed");
        if src_dir.exists() { watcher.watch(&src_dir, RecursiveMode::NonRecursive).ok(); }
        println!("[canon][loader-watcher] watching loader dir");

        while running.load(Ordering::Relaxed) {
            match rx.recv_timeout(std::time::Duration::from_millis(200)) {
                Ok(Ok(event)) => {
                    let is_js = event.paths.iter()
                        .any(|p| p.extension().map(|e| e == "js").unwrap_or(false));
                    if is_js {
                        for p in &event.paths {
                            let name = p.file_name().unwrap_or_default().to_str().unwrap_or("?");
                            if name == "canon-loader.js" || name == "canonrs.bundle.js" {
                                let src = src_dir.join(name);
                                let dst = dest_dir.join(name);
                                if let Ok(c) = std::fs::read_to_string(&src) {
                                    let c = c.replace("__CANONRS_VERSION__", &version);
                                    if std::fs::write(&dst, c).is_ok() {
                                        println!("[canon][loader-watcher] recopied: {}", name);
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    })
}
