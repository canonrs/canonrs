pub mod table_of_contents;
pub use table_of_contents::*;
pub use canonrs_core::primitives::table_of_contents::TocMode;
pub use navigation_provider::{NavigationProvider, NavigationContext, use_navigation};
#[cfg(feature = "examples")]
pub mod examples;
#[cfg(feature = "examples")]
pub use examples::*;
pub mod navigation_provider;

pub use table_of_contents::TableOfContentsPreview;
