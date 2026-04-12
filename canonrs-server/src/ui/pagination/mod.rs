pub mod pagination_ui;
pub mod preview;
// no types to re-export from pagination_ui
pub const EXAMPLES: &str = include_str!("examples.rs");
#[cfg(feature = "examples")]
pub mod examples;
#[cfg(feature = "examples")]
pub use examples::*;

pub use pagination_ui::PaginationPreview;
pub use preview::PaginationShowcasePreview;
pub mod pagination_boundary;
pub use pagination_boundary::*;
pub use pagination_boundary::{Pagination, PaginationContent, PaginationItem, PaginationLink, PaginationPrevious, PaginationNext, PaginationEllipsis};
