//! CanonRS UI - SSR+CSR Safe Components

// CANON RULE: UI nunca pode depender de runtime server-only
#[cfg(all(target_arch = "wasm32", feature = "ssr"))]
compile_error!("❌ CANON VIOLATION: feature 'ssr' cannot be enabled in WASM build");

// Re-export shared types (para `crate::shared`)
pub use canonrs_shared as shared;

pub mod blocks;
pub mod layouts;
pub mod primitives;
pub mod providers;
pub mod ui;
pub mod utils;
