//! CanonRS Orchestrator — Tier S
//! tokens + wasm (notify FS) + leptos + WS reload + system state

use std::process::{Command, Child};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use notify::{Watcher, RecursiveMode, recommended_watcher, Event};
use tokio::sync::broadcast;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../..")
        .canonicalize()
        .unwrap()
}

// --- System State ---
#[derive(Default, Clone)]
struct SystemState {
    tokens: String,
    wasm:   String,
    leptos: String,
}

impl SystemState {
    fn print(&self) {
        println!("\n┌─ CANON SYSTEM STATE ─────────────────");
        println!("│  tokens : {}", self.tokens);
        println!("│  wasm   : {}", self.wasm);
        println!("│  leptos : {}", self.leptos);
        println!("└──────────────────────────────────────\n");
    }
}

// --- FNV hash ---
fn wasm_hash(dest: &PathBuf) -> String {
    let wasm_path = dest.join("canonrs_interactions_bg.wasm");
    if let Ok(bytes) = std::fs::read(&wasm_path) {
        let mut h: u64 = 0xcbf29ce484222325;
        for b in bytes { h ^= b as u64; h = h.wrapping_mul(0x100000001b3); }
        format!("{:x}", h & 0xffffffff)
    } else {
        format!("{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs())
    }
}

fn inject_hash_in_html(root: &PathBuf, hash: &str) {
    let path = root.join("packages-rust/rs-canonrs/canonrs-client/assets/js/wasm_hash.js");
    std::fs::write(&path, format!("window.__CANON_WASM_HASH__ = '{}'; ", hash)).ok();
}

fn ensure_wasm_hash(root: &PathBuf) {
    let js_dir = root.join("packages-rust/rs-canonrs/canonrs-client/assets/js");
    std::fs::create_dir_all(&js_dir).ok();
    // sem placeholder — hash só existe após build_wasm()
}

fn build_wasm(root: &PathBuf, state: &Arc<Mutex<SystemState>>, reload_tx: &broadcast::Sender<()>) {
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

    let t = Instant::now();
    println!("[canon][wasm] building...");
    { state.lock().unwrap().wasm = "building...".into(); }

    let status = Command::new("wasm-pack").args(&args).status();

    match status {
        Ok(s) if s.success() => {
            for entry in std::fs::read_dir(&out_dir).unwrap().filter_map(|e| e.ok()) {
                let name = entry.file_name();
                let name = name.to_str().unwrap();
                if name.ends_with(".d.ts") { continue; }
                if name.ends_with(".wasm") || name.ends_with(".js") {
                    std::fs::copy(entry.path(), dest.join(name)).ok();
                }
            }
            let hash = wasm_hash(&dest);
            inject_hash_in_html(root, &hash);
            let elapsed = t.elapsed().as_millis();
            println!("[canon][wasm] done ({}ms) hash={}", elapsed, hash);
            {
                let mut s = state.lock().unwrap();
                s.wasm = format!("OK ({}ms) hash={}", elapsed, hash);
                s.print();
            }
            // notifica browser via WS
            reload_tx.send(()).ok();
        }
        _ => {
            eprintln!("[canon][wasm] FAILED ({}ms)", t.elapsed().as_millis());
            state.lock().unwrap().wasm = "FAILED".into();
        }
    }
}

fn copy_loaders(root: &PathBuf) {
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

fn spawn_tokens(root: &PathBuf, state: &Arc<Mutex<SystemState>>) {
    let tokens = root.join("packages-rust/rs-canonrs/canonrs-tokens");
    let t = Instant::now();
    println!("[canon][tokens] building...");
    Command::new("cargo")
        .args(["run", "--bin", "tokens-engine"])
        .current_dir(&tokens)
        .env("CARGO_TARGET_DIR", "/tmp/tokens-build")
        .status().ok();
    let elapsed = t.elapsed().as_millis();
    println!("[canon][tokens] done ({}ms)", elapsed);


}

fn spawn_core_watcher(root: &PathBuf, running: Arc<AtomicBool>) -> std::thread::JoinHandle<()> {
    let root = root.clone();
    let watch_dirs: Vec<PathBuf> = [
        "packages-rust/rs-canonrs/canonrs-server/src/blocks",
        "packages-rust/rs-canonrs/canonrs-server/src/layouts",
        "packages-rust/rs-canonrs/canonrs-server/src/ui",
        "packages-rust/rs-canonrs/canonrs-core/build",
    ].iter().map(|d| root.join(d)).collect();

    std::thread::spawn(move || {
        let (tx, rx) = std::sync::mpsc::channel::<notify::Result<Event>>();
        let mut watcher = recommended_watcher(tx).expect("watcher failed");
        for dir in &watch_dirs {
            if dir.exists() { watcher.watch(dir, RecursiveMode::Recursive).ok(); }
        }
        println!("[canon][core-watcher] watching blocks/layouts/ui/build ({} dirs)", watch_dirs.len());

        let mut last_build = Instant::now();
        while running.load(Ordering::Relaxed) {
            match rx.recv_timeout(std::time::Duration::from_millis(200)) {
                Ok(Ok(event)) => {
                    let is_yaml_or_rs = event.paths.iter().any(|p| {
                        p.extension().map(|e| e == "yaml" || e == "rs").unwrap_or(false)
                    });
                    if is_yaml_or_rs && last_build.elapsed().as_millis() > 1000 {
                        for p in &event.paths {
                            println!("[canon][core-watcher] changed: {}", p.file_name().unwrap_or_default().to_str().unwrap_or("?"));
                        }
                        last_build = Instant::now();
                        println!("[canon][core-watcher] touching build.rs to force recompile...");
                        let build_rs = root.join("packages-rust/rs-canonrs/canonrs-core/build.rs");
                        let meta = std::fs::metadata(&build_rs).ok();
                        if let Some(m) = meta {
                            let t = std::time::SystemTime::now();
                            // read + rewrite to update mtime
                            if let Ok(c) = std::fs::read_to_string(&build_rs) {
                                std::fs::write(&build_rs, c).ok();
                            }
                        }
                        println!("[canon][core-watcher] build.rs touched — leptos will recompile");
                    }
                }
                _ => {}
            }
        }
    })
}

fn build_css(root: &PathBuf) {
    let site_dir = root.join("products/canonrs-site");
    if !site_dir.exists() { return; }
    let t = std::time::Instant::now();
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

fn spawn_leptos(root: &PathBuf, project: &str, state: &Arc<Mutex<SystemState>>) -> Child {
    println!("[canon][leptos] starting — project: {}", project);
    state.lock().unwrap().leptos = "RUNNING".into();
    Command::new("cargo")
        .args(["leptos", "watch", "--project", project])
        .current_dir(root)
        .env("CANON_ROOT", root)
        .spawn()
        .expect("cargo leptos not found")
}

// WebSocket reload server na porta 9099
async fn ws_reload_server(reload_rx: broadcast::Receiver<()>) {
    use tokio::net::TcpListener;
    use tokio_tungstenite::accept_async;
    use futures_util::SinkExt;
    use futures_util::StreamExt;

    let listener = match TcpListener::bind("0.0.0.0:9099").await {
        Ok(l) => { println!("[canon][ws] reload server on ws://localhost:9099"); l }
        Err(e) => { eprintln!("[canon][ws] FAILED to bind 9099: {}", e); return; }
    };

    loop {
        let (stream, _) = match listener.accept().await {
            Ok(s) => s,
            Err(e) => { eprintln!("[canon][ws] accept error: {}", e); continue; }
        };

        let mut rx = reload_rx.resubscribe();

        tokio::spawn(async move {
            let ws = match accept_async(stream).await {
                Ok(ws) => { println!("[canon][ws] client connected"); ws }
                Err(e) => { eprintln!("[canon][ws] handshake failed: {}", e); return; }
            };
            let (mut write, mut read) = ws.split();
            loop {
                tokio::select! {
                    msg = read.next() => {
                        match msg {
                            Some(Ok(tokio_tungstenite::tungstenite::Message::Close(_))) => break,
                            Some(Ok(tokio_tungstenite::tungstenite::Message::Ping(_))) => {}
                            Some(Ok(_)) => {}
                            Some(Err(_)) => break,
                            None => break,
                        }
                    }
                    evt = rx.recv() => {
                        match evt {
                            Ok(_) => {
                                let _ = write.send(
                                    tokio_tungstenite::tungstenite::Message::Text("reload".into())
                                ).await;
                            }
                            Err(broadcast::error::RecvError::Lagged(_)) => continue,
                            Err(_) => break,
                        }
                    }
                }
            }
        });
    }
}

fn spawn_loader_watcher(root: &PathBuf, running: Arc<AtomicBool>) -> std::thread::JoinHandle<()> {
    let root = root.clone();
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
                    let is_js = event.paths.iter().any(|p| p.extension().map(|e| e == "js").unwrap_or(false));
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

fn spawn_wasm_watcher(root: &PathBuf, running: Arc<AtomicBool>, state: Arc<Mutex<SystemState>>, reload_tx: broadcast::Sender<()>) -> std::thread::JoinHandle<()> {
    let root = root.clone();
    let watch_dirs: Vec<PathBuf> = [
        "canonrs-interactions",
        "canonrs-interactions-init",
        "canonrs-interactions-nav",
        "canonrs-interactions-data",
        "canonrs-interactions-gesture",
        "canonrs-interactions-overlay",
        "canonrs-interactions-selection",
        "canonrs-interactions-content",
    ].iter().map(|d| root.join("packages-rust/rs-canonrs").join(d).join("src")).collect();

    std::thread::spawn(move || {
        let (tx, rx) = std::sync::mpsc::channel::<notify::Result<Event>>();
        let mut watcher = recommended_watcher(tx).expect("watcher failed");
        for dir in &watch_dirs {
            if dir.exists() { watcher.watch(dir, RecursiveMode::Recursive).ok(); }
        }
        println!("[canon][watcher] watching {} dirs (event-driven)", watch_dirs.len());

        let mut last_build = Instant::now();
        while running.load(Ordering::Relaxed) {
            match rx.recv_timeout(std::time::Duration::from_millis(100)) {
                Ok(Ok(event)) => {
                    let is_rs = event.paths.iter().any(|p| p.extension().map(|e| e == "rs").unwrap_or(false));
                    if is_rs && last_build.elapsed().as_millis() > 500 {
                        for p in &event.paths {
                            println!("[canon][watcher] changed: {}", p.file_name().unwrap_or_default().to_str().unwrap_or("?"));
                        }
                        last_build = Instant::now();
                        build_wasm(&root, &state, &reload_tx);
                    }
                }
                _ => {}
            }
        }
    })
}

#[tokio::main]
async fn main() {
    let root    = root();
    let project = std::env::args().nth(1).unwrap_or_else(|| "canonrs-site".to_string());
    let state   = Arc::new(Mutex::new(SystemState::default()));
    let (reload_tx, reload_rx) = broadcast::channel::<()>(16);

    println!("🚀 CanonRS Orchestrator — Tier S");
    println!("   project: {}", project);

    spawn_tokens(&root, &state);
    copy_loaders(&root);
    // garante wasm_hash.js existe antes do leptos
    ensure_wasm_hash(&root);
    build_wasm(&root, &state, &reload_tx);

    let running = Arc::new(AtomicBool::new(true));
    let _watcher = spawn_wasm_watcher(&root, running.clone(), state.clone(), reload_tx.clone());
    let _loader_watcher = spawn_loader_watcher(&root, running.clone());
    let _core_watcher = spawn_core_watcher(&root, running.clone());

    // WS reload server em background
    println!("[canon][ws] starting...");
    tokio::spawn(async move {
        ws_reload_server(reload_rx).await;
        eprintln!("[canon][ws] server exited");
    });
    println!("[canon][ws] spawned");

    build_css(&root);
    let mut leptos = spawn_leptos(&root, &project, &state);
    state.lock().unwrap().print();
    println!("[canon] leptos spawned, continuing...");

    let running_ctrlc = running.clone();
    ctrlc::set_handler(move || {
        println!("\n[canon] shutting down...");
        running_ctrlc.store(false, Ordering::Relaxed);
        std::process::exit(0);
    }).ok();

    // leptos.wait() e bloqueante — rodar em thread separada
    let running_leptos = running.clone();
    std::thread::spawn(move || {
        leptos.wait().ok();
        running_leptos.store(false, Ordering::Relaxed);
    });

    // mantém tokio runtime vivo para o WS server
    while running.load(Ordering::Relaxed) {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
