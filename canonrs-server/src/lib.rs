#![recursion_limit = "512"]
//! CanonRS Server — SSR UI components + Axum integration

pub mod ui;
pub mod dev;
pub mod blocks;
pub mod layouts;
pub mod pages;
pub mod providers;
pub mod interactions;
pub use canonrs_core::primitives;
pub use canonrs_core::meta::VisibilityState;

// UI — re-exports explícitos
pub use ui::button::Button;
pub use ui::badge::Badge;
pub use ui::input::Input;
pub use ui::tabs::{TabsRoot, TabsTrigger, TabsContent};
pub use ui::icon_button::icon_button_ui::{IconButton, IconButtonVariant};
pub use ui::card::Card;
pub use ui::separator::Separator;
pub use ui::doc_progress::DocProgress;

// Layouts — re-exports explícitos
pub use layouts::MarketingLayout;
pub use layouts::DashboardLayout;
pub use layouts::{SplitViewLayout, SplitRatio};
pub use layouts::FullscreenLayout;
pub use layouts::WizardLayout;
pub use layouts::{PageLayout, PageLayoutVariant};
