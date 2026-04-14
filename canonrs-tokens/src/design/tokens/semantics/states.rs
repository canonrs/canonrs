//! Semantics — States
//! Feedback and status color contracts.

use crate::design::tokens::FamilyToken;

pub const SEMANTICS_STATES: &[FamilyToken] = &[
    FamilyToken::new("color-success",             "var(--theme-state-success-bg)"),
    FamilyToken::new("color-success-foreground",  "var(--theme-state-success-fg)"),
    FamilyToken::new("color-warning",             "var(--theme-state-warning-bg)"),
    FamilyToken::new("color-warning-foreground",  "var(--theme-state-warning-fg)"),
    FamilyToken::new("color-error",               "var(--theme-state-error-bg)"),
    FamilyToken::new("color-error-foreground",    "var(--theme-state-error-fg)"),
    FamilyToken::new("color-info",                "var(--theme-state-info-bg)"),
    FamilyToken::new("color-info-foreground",     "var(--theme-state-info-fg)"),
    FamilyToken::new("color-neutral-900",         "var(--theme-surface-fg)"),
    FamilyToken::new("color-neutral-50",          "var(--theme-surface-bg)"),
];
