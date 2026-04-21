//! UID — geração de identificadores únicos tier 1
//! Padrão: prefix-timestamp_hex-counter_hex
use std::sync::atomic::{AtomicU64, Ordering};

pub fn generate(prefix: &str) -> String {
    static CTR: AtomicU64 = AtomicU64::new(0);
    let ctr = CTR.fetch_add(1, Ordering::SeqCst);
    #[cfg(target_arch = "wasm32")]
    let ts: u64 = ctr;
    #[cfg(not(target_arch = "wasm32"))]
    let ts: u64 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(ctr);
    format!("{}-{:016x}-{:08x}", prefix, ts, ctr)
}
