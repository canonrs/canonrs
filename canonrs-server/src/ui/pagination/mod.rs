pub mod pagination_ui;
pub mod pagination_island;
pub mod preview;
pub use pagination_ui::*;
pub use pagination_island::PaginationIsland;
pub const EXAMPLES: &str = include_str!("examples.rs");
#[cfg(feature = "examples")]
pub mod examples;
#[cfg(feature = "examples")]
pub use examples::*;

pub use pagination_ui::PaginationPreview;
pub use preview::PaginationShowcasePreview;
