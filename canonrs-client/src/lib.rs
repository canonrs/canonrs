//! CanonRS Client — WASM facade (ui-interactive + islands)
pub mod engines;
pub mod shared;
#[allow(unused_imports)]
#[cfg(feature = "hydrate")]
pub use canonrs_core::primitives;
#[allow(ambiguous_glob_reexports)]
pub use canonrs_core::primitives::*;
pub use canonrs_core::infra::theme as providers;
pub use canonrs_core::infra::theme::ThemeMode;

pub mod runtime;
pub mod behaviors;
pub mod interactions;

pub mod themes;
pub mod ui;
