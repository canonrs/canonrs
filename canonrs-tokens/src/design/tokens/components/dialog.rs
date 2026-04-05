use crate::design::tokens::FamilyToken;

/// Dialog component tokens
/// Scope: modal dialog, centered overlay

pub const DIALOG_TOKENS: &[FamilyToken] = &[
    FamilyToken::new("dialog-overlay-bg",               "var(--color-overlay-50)"),
    FamilyToken::new("dialog-overlay-z-index",          "var(--layer-overlay)"),
    FamilyToken::new("dialog-content-bg",               "var(--theme-surface-bg)"),
    FamilyToken::new("dialog-content-fg",               "var(--theme-surface-fg)"),
    FamilyToken::new("dialog-content-width",            "var(--layout-width-lg)"),
    FamilyToken::new("dialog-content-max-width",        "90vw"),
    FamilyToken::new("dialog-content-padding",          "var(--space-lg)"),
    FamilyToken::new("dialog-content-radius",           "var(--radius-md)"),
    FamilyToken::new("dialog-content-shadow",           "var(--shadow-2xl)"),
    FamilyToken::new("dialog-header-gap",               "var(--space-sm)"),
    FamilyToken::new("dialog-footer-gap",               "var(--space-sm)"),
    FamilyToken::new("dialog-title-fg",                 "var(--theme-surface-fg)"),
    FamilyToken::new("dialog-title-font-size",          "var(--font-size-lg)"),
    FamilyToken::new("dialog-title-font-weight",        "var(--font-weight-semibold)"),
    FamilyToken::new("dialog-description-fg",           "var(--theme-surface-fg-muted)"),
    FamilyToken::new("dialog-description-font-size",    "var(--font-size-sm)"),
    FamilyToken::new("dialog-close-fg",                 "var(--theme-surface-fg-muted)"),
    FamilyToken::new("dialog-close-fg-hover",           "var(--theme-surface-fg)"),
    FamilyToken::new("dialog-transition-duration",      "var(--motion-duration-normal)"),
];
