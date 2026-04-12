pub mod collapsible_ui;
pub mod collapsible_boundary;
pub mod preview;
#[cfg(feature = "examples")]
pub mod examples;

pub use collapsible_ui::*;
pub use collapsible_boundary::{Collapsible, CollapsibleTrigger, CollapsibleContent};
#[cfg(feature = "examples")]
pub use examples::*;

pub use collapsible_ui::CollapsiblePreview;
pub use preview::CollapsibleShowcasePreview;
