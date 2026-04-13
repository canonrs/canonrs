//! Enterprise Layouts — CanonRS Layout System

pub mod marketing;
pub use marketing::MarketingLayout;

pub mod dashboard;
pub use dashboard::DashboardLayout;

pub mod split_view;
pub use split_view::{SplitViewLayout, SplitRatio};

pub mod three_pane;
pub use three_pane::ThreePaneLayout;

pub mod fullscreen;
pub use fullscreen::FullscreenLayout;

pub mod wizard;
pub use wizard::WizardLayout;

pub mod page_layout;
pub use page_layout::{PageLayout, PageLayoutVariant};

// Re-export from blocks for backwards compatibility
pub use crate::blocks::page_header::PageHeader;
