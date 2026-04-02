//! Semantics — Surface
//! Maps color names to theme tokens. No HSL values.

use crate::design::tokens::FamilyToken;

pub const SEMANTICS_SURFACE: &[FamilyToken] = &[
    FamilyToken::new("color-background",          "var(--theme-surface-bg)"),
    FamilyToken::new("color-foreground",          "var(--theme-surface-fg)"),
    FamilyToken::new("color-muted",               "var(--theme-surface-muted)"),
    FamilyToken::new("color-muted-foreground",    "var(--theme-surface-fg-muted)"),
    FamilyToken::new("color-border",              "var(--theme-surface-border)"),
    FamilyToken::new("color-popover",             "var(--theme-overlay-bg)"),
    FamilyToken::new("color-popover-foreground",  "var(--theme-overlay-fg)"),
    FamilyToken::new("color-ring",                "var(--theme-action-focus-ring)"),
];
