// ============================================================================
// CONTEXTUAL TOKEN FAMILIES (A-F)
// ============================================================================

// A) Overlay & Layering
pub const OVERLAY_BACKDROP_OPACITY: f32 = 0.8;
pub const OVERLAY_DISMISS_OUTSIDE: bool = true;
pub const OVERLAY_DISMISS_ESCAPE: bool = true;
pub const OVERLAY_FOCUS_TRAP: bool = true;
pub const OVERLAY_PORTAL: &str = "body";

// B) Selection & Lists
pub const LIST_ITEM_HEIGHT: &str = "var(--list-item-height, 2.5rem)";
pub const LIST_ITEM_PADDING: &str = "var(--list-item-padding, 0.5rem 1rem)";
pub const SELECTION_MODE_SINGLE: &str = "single";
pub const SELECTION_MODE_MULTI: &str = "multi";
pub const SELECTION_INDICATOR_CHECK: &str = "check";
pub const SELECTION_INDICATOR_RADIO: &str = "radio";
pub const EMPTY_STATE_STYLE: &str = "centered";

// C) Forms & Validation
pub const FIELD_HEIGHT: &str = "var(--field-height, 2.5rem)";
pub const FIELD_PADDING: &str = "var(--field-padding, 0.5rem 1rem)";
pub const FIELD_BORDER: &str = "var(--field-border, 1px solid hsl(var(--color-border)))";
pub const FIELD_PLACEHOLDER: &str = "hsl(var(--color-muted-foreground))";
pub const VALIDATION_ERROR: &str = "hsl(var(--color-destructive))";
pub const VALIDATION_SUCCESS: &str = "hsl(142 71% 45%)";
pub const VALIDATION_WARNING: &str = "hsl(45 93% 47%)";
pub const INPUT_MASKING: bool = false;

// D) Navigation & Structure
pub const NAV_ITEM_HEIGHT: &str = "var(--nav-item-height, 2.5rem)";
pub const NAV_INDICATOR_THICKNESS: &str = "2px";
pub const NAV_BREADCRUMB_SEPARATOR: &str = "/";
pub const SIDEBAR_WIDTH: &str = "var(--sidebar-width, 16rem)";
pub const PAGINATION_SIZE: &str = "var(--pagination-size, 2rem)";

// E) Feedback & Status
pub const TOAST_DURATION: u32 = 5000; // ms
pub const SPINNER_SIZE: &str = "var(--spinner-size, 2rem)";
pub const SKELETON_SHIMMER: bool = true;
pub const PROGRESS_HEIGHT: &str = "var(--progress-height, 0.5rem)";
pub const ALERT_EMPHASIS: &str = "subtle";

// F) Data & Media
pub const CHART_PALETTE: [&str; 5] = [
    "hsl(var(--chart-1))",
    "hsl(var(--chart-2))",
    "hsl(var(--chart-3))",
    "hsl(var(--chart-4))",
    "hsl(var(--chart-5))",
];
pub const CHART_GRID: bool = true;
pub const CHART_TOOLTIP: bool = true;
pub const CAROUSEL_GAP: &str = "var(--carousel-gap, 1rem)";
pub const CAROUSEL_SNAP: &str = "start";
pub const ASPECT_RATIO_1_1: f32 = 1.0;
pub const ASPECT_RATIO_16_9: f32 = 1.778;
pub const ASPECT_RATIO_4_3: f32 = 1.333;
