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


pub mod page_layout;
pub use page_layout::{PageLayout, PageLayoutVariant};

pub mod section;
pub use section::Section;

pub use page_layout::{MockSidebar, MockAside, MockMain};

// Re-export from blocks for backwards compatibility
pub use crate::blocks::page_header::PageHeader;

// Mock exports
pub use dashboard::MockDashboardContent;
pub use marketing::{MockMarketingHeader, MockMarketingHero, MockMarketingMain, MockMarketingFooter};
pub use fullscreen::{MockFullscreenHeader, MockFullscreenContent};
pub use split_view::{MockSplitLeft, MockSplitRight};
pub use wizard::{MockWizardHeader, MockWizardStepper, MockWizardContent, MockWizardFooter};
pub use section::{MockSectionHeader, MockSectionContent, MockSectionFooter};
