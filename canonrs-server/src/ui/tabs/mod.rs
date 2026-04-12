pub mod tabs_ui;
pub mod boundary;
pub mod preview;

pub use tabs_ui::*;
pub use boundary::{TabsRoot, TabsTrigger, TabsContent};
pub const EXAMPLES: &str = include_str!("examples.rs");
#[cfg(feature = "examples")]
pub mod examples;
#[cfg(feature = "examples")]
pub use examples::*;

pub use tabs_ui::TabsPreview;
pub use preview::TabsShowcasePreview;
