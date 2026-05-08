//! Configuração central do orchestrator — paths e constantes

use std::path::PathBuf;

pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../..")
        .canonicalize()
        .unwrap()
}

/// Crates WASM observados pelo wasm_watcher.
/// Adicionar aqui quando um novo crate de interação for criado.
pub const WASM_CRATES: &[&str] = &[
    "canonrs-interactions",
    "canonrs-interactions-core",
    "canonrs-interactions-init",
    "canonrs-interactions-nav",
    "canonrs-interactions-data",
    "canonrs-interactions-gesture",
    "canonrs-interactions-overlay",
    "canonrs-interactions-selection",
    "canonrs-interactions-content",
];

/// Diretórios observados pelo core_watcher (blocks/layouts/ui).
pub const CORE_WATCH_DIRS: &[&str] = &[
    "packages-rust/rs-canonrs/canonrs-server/src/blocks",
    "packages-rust/rs-canonrs/canonrs-server/src/layouts",
    "packages-rust/rs-canonrs/canonrs-server/src/ui",
    "packages-rust/rs-canonrs/canonrs-core/build",
];

pub const WS_PORT: u16 = 9099;
pub const WASM_DEBOUNCE_MS: u128 = 500;
pub const CORE_DEBOUNCE_MS: u128 = 1000;
