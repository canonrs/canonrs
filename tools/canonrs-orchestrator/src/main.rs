//! CanonRS Orchestrator — Tier S
//! Bootstrap: tokens → loaders → wasm → watchers → ws → css → leptos

mod config;
mod state;
mod wasm;
mod pipeline;
mod watchers;
mod ws;

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::broadcast;

use config::root;
use state::SystemState;
use pipeline::{spawn_tokens, copy_loaders, build_css, spawn_leptos};
use wasm::{ensure_wasm_hash, build_wasm};
use watchers::{spawn_wasm_watcher, spawn_core_watcher, spawn_loader_watcher};
use ws::ws_reload_server;

#[tokio::main]
async fn main() {
    let root    = root();
    let project = std::env::args().nth(1).unwrap_or_else(|| "canonrs-site".to_string());
    let state   = Arc::new(Mutex::new(SystemState::default()));
    let (reload_tx, reload_rx) = broadcast::channel::<()>(16);

    println!("🚀 CanonRS Orchestrator — Tier S");
    println!("   project : {}", project);
    println!("   root    : {}", root.display());

    // 1. tokens
    spawn_tokens(&root, &state);

    // 2. loaders
    copy_loaders(&root);

    // 3. wasm (inicial)
    ensure_wasm_hash(&root);
    build_wasm(&root, &state, &reload_tx);
    // 3b. capability groups — build each interaction group as standalone wasm
    wasm::build_all_groups(&root);

    // 4. watchers
    let running = Arc::new(AtomicBool::new(true));
    let _wasm_watcher   = spawn_wasm_watcher(&root, running.clone(), state.clone(), reload_tx.clone());
    let _loader_watcher = spawn_loader_watcher(&root, running.clone());
    let _core_watcher   = spawn_core_watcher(&root, running.clone());

    // 5. WS reload server
    tokio::spawn(async move {
        ws_reload_server(reload_rx).await;
        eprintln!("[canon][ws] server exited");
    });
    println!("[canon][ws] spawned");

    // 6. CSS
    build_css(&root);

    // 7. leptos
    let mut leptos = spawn_leptos(&root, &project, &state);
    state.lock().unwrap().print();
    println!("[canon] all systems running");

    // ctrlc
    let running_ctrlc = running.clone();
    ctrlc::set_handler(move || {
        println!("\n[canon] shutting down...");
        running_ctrlc.store(false, Ordering::Relaxed);
        std::process::exit(0);
    }).ok();

    // leptos em thread separada
    let running_leptos = running.clone();
    std::thread::spawn(move || {
        leptos.wait().ok();
        running_leptos.store(false, Ordering::Relaxed);
    });

    // mantém tokio runtime vivo
    while running.load(Ordering::Relaxed) {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
