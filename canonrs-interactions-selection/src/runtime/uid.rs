//! UID — geração de identificadores únicos para componentes
use std::sync::atomic::{AtomicU64, Ordering};

pub fn generate(prefix: &str) -> String {
    static CTR: AtomicU64 = AtomicU64::new(0);
    let ctr = CTR.fetch_add(1, Ordering::SeqCst);
    format!("{}-{:016x}-{:08x}",
        prefix,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(ctr),
        ctr)
}
