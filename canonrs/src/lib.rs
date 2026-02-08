//! CanonRS Facade
//!
//! Pure orchestration layer.
//! Defines stable public contracts and runtime boundaries.

#[cfg(all(feature = "ssr", feature = "hydrate"))]
compile_error!("Features `ssr` and `hydrate` are mutually exclusive.");

/// =======================================================
/// UI — SSR-safe
/// =======================================================
///
/// Declarative, deterministic components.
/// No callbacks, no DOM access, no client state.
pub mod ui {
    pub use canonrs_ui::ui::*;
}

/// =======================================================
/// UI Interactive — CSR-only by feature
/// =======================================================
///
/// Interactive components with callbacks and client state.
///
/// - The MODULE always exists (contract stability)
/// - The CONTENT only exists when `with-interactive` is enabled
/// - No stubs, no mocks, no fake implementations
pub mod ui_interactive {
    #[cfg(feature = "with-interactive")]
    pub use canonrs_ui_interactive::ui::*;
}

/// =======================================================
/// Layouts
/// =======================================================
pub mod layouts {
    pub use canonrs_ui::layouts::*;
}

/// =======================================================
/// Blocks
/// =======================================================
pub mod blocks {
    pub use canonrs_ui::blocks::*;
}

/// =======================================================
/// Providers
/// =======================================================
pub mod providers {
    pub use canonrs_providers::prelude::*;
}

/// =======================================================
/// Shared
/// =======================================================
pub mod shared {
    pub use canonrs_shared::*;
}

/// =======================================================
/// CSR Runtime
/// =======================================================
#[cfg(feature = "with-behaviors")]
pub mod behaviors {
    pub use canonrs_behaviors::*;
}

/// Serve CanonRS CSS bundle
/// 
/// Mount this at `/canonrs.css` in your Axum router
#[cfg(feature = "ssr")]
pub fn canonrs_css() -> &'static str {
    include_str!(concat!(env!("OUT_DIR"), "/canonrs.css"))
}
