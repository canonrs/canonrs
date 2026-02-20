//! Enterprise Layouts
//!
//! Official CanonRS layout system (90% coverage)

pub mod marketing;
pub use marketing::MarketingLayout;


pub mod dashboard;
pub use dashboard::DashboardLayout;

pub mod split_view;
pub use split_view::{SplitViewLayout, SplitRatio};

pub mod fullscreen;
pub use fullscreen::FullscreenLayout;

pub mod wizard;
pub use wizard::WizardLayout;

pub mod page_header;
pub use page_header::PageHeader;

pub mod page_layout;
pub use page_layout::{PageLayout, PageLayoutVariant};

pub mod section;
pub use section::Section;

pub use page_layout::{MockSidebar, MockAside, MockMain};
