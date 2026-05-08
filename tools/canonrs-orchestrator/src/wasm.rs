//! WASM build — wasm-pack, hash, inject

use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::sync::broadcast;
use crate::state::SystemState;

pub fn ensure_wasm_hash(root: &PathBuf) {
    let js_dir = root.join("packages-rust/rs-canonrs/canonrs-client/assets/js");
    std::fs::create_dir_all(&js_dir).ok();
}

pub fn wasm_hash(dest: &PathBuf) -> String {
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

pub fn inject_hash_in_html(root: &PathBuf, hash: &str) {
    let path = root.join("packages-rust/rs-canonrs/canonrs-client/assets/js/wasm_hash.js");
    std::fs::write(&path, format!("window.__CANON_WASM_HASH__ = '{}'; ", hash)).ok();
}

pub fn build_wasm(root: &PathBuf, state: &Arc<Mutex<SystemState>>, reload_tx: &broadcast::Sender<()>) {
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
            let hash    = wasm_hash(&dest);
            let elapsed = t.elapsed().as_millis();
            inject_hash_in_html(root, &hash);
            println!("[canon][wasm] done ({}ms) hash={}", elapsed, hash);
            {
                let mut s = state.lock().unwrap();
                s.wasm = format!("OK ({}ms) hash={}", elapsed, hash);
                s.print();
            }
            reload_tx.send(()).ok();
        }
        _ => {
            eprintln!("[canon][wasm] FAILED ({}ms)", t.elapsed().as_millis());
            state.lock().unwrap().wasm = "FAILED".into();
        }
    }
}
