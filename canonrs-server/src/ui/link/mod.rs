pub mod link_ui;
pub use link_ui::*;

pub mod boundary;
pub use boundary::Link;
pub use canonrs_core::primitives::LinkVariant;

pub const EXAMPLES: &str = include_str!("examples.rs");
#[cfg(feature = "examples")]
pub mod examples;
#[cfg(feature = "examples")]
pub use examples::*;

pub mod preview;
pub use preview::LinkShowcasePreview;
