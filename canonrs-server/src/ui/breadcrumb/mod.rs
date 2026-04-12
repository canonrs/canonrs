mod breadcrumb_ui;
pub mod preview;
mod breadcrumb_auto;
mod navigation_provider;
#[cfg(feature = "examples")]
pub mod examples;

// no types to re-export from breadcrumb_ui
pub use breadcrumb_auto::*;
pub use navigation_provider::*;
pub use breadcrumb_ui::BreadcrumbPreview;
pub use preview::BreadcrumbShowcasePreview;
pub mod breadcrumb_boundary;
pub use breadcrumb_boundary::*;
pub use breadcrumb_boundary::{Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbPage, BreadcrumbSeparator, BreadcrumbEllipsis};
