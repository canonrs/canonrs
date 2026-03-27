//! CanonRS Client — WASM facade (ui-interactive + behaviors)

pub use canonrs_core::primitives;
#[allow(ambiguous_glob_reexports)]
pub use canonrs_core::primitives::*;
pub use canonrs_core::infra::theme as providers;
pub use canonrs_core::infra::theme::ThemeMode;

#[cfg(target_arch = "wasm32")]
pub mod behaviors;

#[cfg(target_arch = "wasm32")]
pub use behaviors::*;

pub mod themes;
pub mod ui;
