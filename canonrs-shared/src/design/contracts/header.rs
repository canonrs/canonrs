//! Header block tokens
//! 
//! **Block:** Header
//! **Primary Family:** D (Navigation & Structure)
//! **Secondary Families:** Core (spacing, border), Color (semantic)

// 1. Structural tokens (layout)
pub const HEADER_HEIGHT: &str = "header.height";
pub const HEADER_PADDING_X: &str = "header.padding.x";
pub const HEADER_PADDING_Y: &str = "header.padding.y";
pub const HEADER_GAP: &str = "header.gap";

// 2. Slot tokens - Logo
pub const HEADER_LOGO_MAX_WIDTH: &str = "header.logo.max-width";
pub const HEADER_LOGO_ALIGN: &str = "header.logo.align";

// 3. Slot tokens - Primary Navigation
pub const HEADER_NAV_GAP: &str = "header.nav.gap";
pub const HEADER_NAV_ALIGN: &str = "header.nav.align";

// 4. Slot tokens - Actions
pub const HEADER_ACTIONS_GAP: &str = "header.actions.gap";
pub const HEADER_ACTIONS_ALIGN: &str = "header.actions.align";

// 5. Border / Separation tokens
pub const HEADER_BORDER_WIDTH: &str = "header.border.width";
pub const HEADER_BORDER_STYLE: &str = "header.border.style";
pub const HEADER_BORDER_COLOR: &str = "header.border.color";

// 6. Semantic color tokens
pub const HEADER_BG: &str = "header.bg";
pub const HEADER_FG: &str = "header.fg";

// 7. State tokens (sticky/scrolled)
pub const HEADER_STATE_STICKY_SHADOW: &str = "header.state.sticky.shadow";
pub const HEADER_STATE_SCROLLED_BG: &str = "header.state.scrolled.bg";
