pub mod collapsible_ui;
pub mod boundary;
pub mod preview;
#[cfg(feature = "examples")]
pub mod examples;

pub use collapsible_ui::*;
pub use boundary::{Collapsible, CollapsibleTrigger, CollapsibleContent};
#[cfg(feature = "examples")]
pub use examples::*;

pub use collapsible_ui::CollapsiblePreview;
pub use preview::CollapsibleShowcasePreview;
