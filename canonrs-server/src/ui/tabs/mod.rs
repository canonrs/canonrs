pub mod tabs_ui;
pub mod preview;

pub use tabs_ui::*;
pub const EXAMPLES: &str = include_str!("examples.rs");
#[cfg(feature = "examples")]
pub mod examples;
#[cfg(feature = "examples")]
pub use examples::*;

pub use tabs_ui::TabsPreview;
pub use preview::TabsShowcasePreview;
