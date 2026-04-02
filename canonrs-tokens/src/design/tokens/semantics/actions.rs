//! Semantics — Actions
//! Interactive element color contracts.

use crate::design::tokens::FamilyToken;

pub const SEMANTICS_ACTIONS: &[FamilyToken] = &[
    FamilyToken::new("color-primary",              "var(--theme-action-primary-bg)"),
    FamilyToken::new("color-primary-foreground",   "var(--theme-action-primary-fg)"),
    FamilyToken::new("color-primary-hover",        "var(--theme-action-primary-hover)"),
    FamilyToken::new("color-primary-active",       "var(--theme-action-primary-active)"),
    FamilyToken::new("color-secondary",            "var(--theme-action-secondary-bg)"),
    FamilyToken::new("color-secondary-foreground", "var(--theme-action-secondary-fg)"),
    FamilyToken::new("color-accent",               "var(--theme-action-accent-bg)"),
    FamilyToken::new("color-accent-foreground",    "var(--theme-action-accent-fg)"),
    // Legacy aliases
    FamilyToken::new("color-destructive",             "var(--theme-state-error-bg)"),
    FamilyToken::new("color-destructive-foreground",  "var(--theme-state-error-fg)"),
];
