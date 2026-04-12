pub mod link_ui;
// no types to re-export from link_ui

pub mod link_boundary;
pub use link_boundary::*;
pub use link_boundary::Link;
pub use canonrs_core::primitives::LinkVariant;

pub const EXAMPLES: &str = include_str!("examples.rs");
#[cfg(feature = "examples")]
pub mod examples;
#[cfg(feature = "examples")]
pub use examples::*;

pub mod preview;
pub use preview::LinkShowcasePreview;
