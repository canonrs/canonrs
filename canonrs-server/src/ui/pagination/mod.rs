pub mod pagination_ui;
pub mod preview;
pub use pagination_ui::*;
pub const EXAMPLES: &str = include_str!("examples.rs");
#[cfg(feature = "examples")]
pub mod examples;
#[cfg(feature = "examples")]
pub use examples::*;

pub use pagination_ui::PaginationPreview;
pub use preview::PaginationShowcasePreview;
pub mod pagination_island;
pub use pagination_island::{PaginationIsland, PaginationContentIsland, PaginationItemIsland, PaginationLinkIsland, PaginationPreviousIsland, PaginationNextIsland, PaginationEllipsisIsland};
