//! Sidebar block tokens
//! 
//! **Block:** Sidebar
//! **Purpose:** Navigation panel for dashboards/apps
//! **Token Family:** D (Navigation) + Core

// Structural tokens
pub const SIDEBAR_WIDTH: &str = "sidebar.width";
pub const SIDEBAR_WIDTH_COLLAPSED: &str = "sidebar.width.collapsed";
pub const SIDEBAR_PADDING: &str = "sidebar.padding";
pub const SIDEBAR_GAP: &str = "sidebar.gap";

// Item tokens (reuse nav.item.* from family D)
pub const SIDEBAR_ITEM_HEIGHT: &str = "nav.item.height";
pub const SIDEBAR_ITEM_PADDING: &str = "nav.item.padding";
pub const SIDEBAR_INDICATOR_THICKNESS: &str = "nav.indicator.thickness";

// Semantic colors
pub const SIDEBAR_BG: &str = "sidebar.bg";
pub const SIDEBAR_FG: &str = "sidebar.fg";
pub const SIDEBAR_BORDER_COLOR: &str = "sidebar.border.color";

// Active state
pub const SIDEBAR_ITEM_ACTIVE_BG: &str = "sidebar.item.active.bg";
pub const SIDEBAR_ITEM_ACTIVE_FG: &str = "sidebar.item.active.fg";
