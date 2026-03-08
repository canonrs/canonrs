//! CanonRS Facade

#[cfg(all(feature = "ssr", feature = "hydrate"))]
compile_error!("Features `ssr` and `hydrate` are mutually exclusive.");

pub use canonrs_core::*;

pub mod primitives {
    pub use canonrs_core::primitives::*;
}

pub mod shared {
    pub use canonrs_core::shared::*;
}

pub mod providers {
    pub use canonrs_core::theme::*;
    pub mod prelude {
        pub use canonrs_core::prelude::*;
    }
}

#[cfg(any(feature = "ssr", feature = "hydrate"))]
pub mod ui {
    pub use canonrs_server::ui::*;
}

#[cfg(any(feature = "ssr", feature = "hydrate"))]
pub mod layouts {
    pub use canonrs_server::layouts::*;
}

#[cfg(any(feature = "ssr", feature = "hydrate"))]
pub mod blocks {
    pub use canonrs_server::blocks::*;
}

#[cfg(feature = "hydrate")]
pub mod ui_interactive {
    pub use canonrs_client::ui::*;
}

#[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
pub mod behaviors {
    pub use canonrs_client::behaviors::*;
}

#[cfg(feature = "ssr")]
pub fn canonrs_css() -> &'static str {
    include_str!(concat!(env!("OUT_DIR"), "/canonrs.css"))
}
