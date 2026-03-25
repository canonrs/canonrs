//! CanonRS Client — WASM facade (ui-interactive + behaviors)

pub use canonrs_core::primitives;
#[allow(ambiguous_glob_reexports)]
pub use canonrs_core::primitives::*;
pub use canonrs_core::utils;
pub use canonrs_core::theme as providers;
pub use canonrs_core::theme::ThemeMode;

#[cfg(target_arch = "wasm32")]
pub mod behaviors;

#[cfg(target_arch = "wasm32")]
pub use behaviors::*;

pub mod themes;
pub mod ui;
