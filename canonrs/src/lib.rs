//! CanonRS Facade

#[cfg(all(feature = "ssr", feature = "hydrate"))]
compile_error!("Features `ssr` and `hydrate` are mutually exclusive.");

pub use canonrs_core::*;

pub mod primitives {
    pub use canonrs_core::primitives::*;
}

pub mod shared {
    pub use canonrs_core::{TocItem, Orientation, NavigationState, HeadingHierarchy, HeadingNode};
}

pub mod providers {
    pub use canonrs_core::infra::theme::*;
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

#[cfg(feature = "hydrate")]
pub mod hooks {
    pub use canonrs_client::hooks::*;
}

#[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
pub mod runtime {
    pub use canonrs_client::runtime::init;
}

pub mod block_types {
    pub use canonrs_core::block_types::*;
}

pub mod catalog_types {
    pub use canonrs_core::catalog_types::*;
}

pub mod generated {
    pub mod block_definitions {
        pub use canonrs_core::generated::block_definitions::*;
    }
    pub mod component_definitions {
        pub use canonrs_core::generated::component_definitions::*;
    }
    pub mod catalog {
        pub use canonrs_core::generated::catalog::*;
    }
    pub mod component_meta {
        pub use canonrs_core::generated::component_meta::*;
    }
}

pub mod infra {
    pub mod constraint_engine {
        pub use canonrs_core::infra::constraint_engine::*;
    }
}
#[cfg(all(feature = "ssr", debug_assertions))]
pub use canonrs_server::dev::reload::with_dev_reload;

#[cfg(feature = "ssr")]
pub fn canonrs_css() -> &'static str {
    include_str!(concat!(env!("OUT_DIR"), "/canonrs.css"))
}
