//! CanonRS Facade

#[cfg(all(feature = "ssr", feature = "hydrate"))]
compile_error!("Features `ssr` and `hydrate` are mutually exclusive.");

pub mod ui {
    pub use canonrs_ui::ui::*;
}

#[cfg(feature = "hydrate")]
pub mod ui_interactive {
    pub use canonrs_ui_interactive::ui::*;
}

pub mod layouts {
    pub use canonrs_ui::layouts::*;
}

pub mod blocks {
    pub use canonrs_ui::blocks::*;
}

pub use canonrs_providers as providers;


pub mod shared {
    pub use canonrs_shared::*;
}

#[cfg(feature = "hydrate")]
pub mod behaviors {
    pub use canonrs_behaviors::*;
}

#[cfg(feature = "ssr")]
pub fn canonrs_css() -> &'static str {
    include_str!(concat!(env!("OUT_DIR"), "/canonrs.css"))
}
