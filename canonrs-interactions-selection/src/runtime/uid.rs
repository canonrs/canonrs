//! UID — geracao de identificadores unicos para componentes
use std::sync::atomic::{AtomicU64, Ordering};

static CTR: AtomicU64 = AtomicU64::new(0);

pub fn generate(prefix: &str) -> String {
    let ctr = CTR.fetch_add(1, Ordering::SeqCst);
    format!("{}-{:016x}-{:08x}", prefix, ctr << 16 | (ctr ^ 0xdeadbeef), ctr as u32)
}
