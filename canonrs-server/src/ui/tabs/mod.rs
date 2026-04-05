pub mod tabs_ui;
pub mod tabs_island;
pub mod preview;

pub use tabs_ui::*;
pub use tabs_island::{TabsIsland, TabItem, TabsRootIsland, TabsTriggerIsland, TabsContentIsland, TabsContext};
pub const EXAMPLES: &str = include_str!("examples.rs");
#[cfg(feature = "examples")]
pub mod examples;
#[cfg(feature = "examples")]
pub use examples::*;

pub use tabs_ui::TabsPreview;
pub use preview::TabsShowcasePreview;
