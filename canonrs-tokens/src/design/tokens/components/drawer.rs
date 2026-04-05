use crate::design::tokens::FamilyToken;

/// Drawer component tokens
/// Scope: slide-in panel, directional overlay

pub const DRAWER_TOKENS: &[FamilyToken] = &[
    FamilyToken::new("drawer-overlay-bg",           "var(--color-overlay-50)"),
    FamilyToken::new("drawer-overlay-z-index",      "var(--layer-overlay)"),
    FamilyToken::new("drawer-bg",                   "var(--theme-surface-bg)"),
    FamilyToken::new("drawer-fg",                   "var(--theme-surface-fg)"),
    FamilyToken::new("drawer-width",                "var(--layout-width-md)"),
    FamilyToken::new("drawer-height",               "40vh"),
    FamilyToken::new("drawer-padding",              "var(--space-lg)"),
    FamilyToken::new("drawer-shadow",               "var(--shadow-2xl)"),
    FamilyToken::new("drawer-header-gap",           "var(--space-sm)"),
    FamilyToken::new("drawer-footer-gap",           "var(--space-sm)"),
    FamilyToken::new("drawer-title-fg",             "var(--theme-surface-fg)"),
    FamilyToken::new("drawer-title-font-size",      "var(--font-size-lg)"),
    FamilyToken::new("drawer-title-font-weight",    "var(--font-weight-semibold)"),
    FamilyToken::new("drawer-description-fg",       "var(--theme-surface-fg-muted)"),
    FamilyToken::new("drawer-description-font-size","var(--font-size-sm)"),
    FamilyToken::new("drawer-close-fg",             "var(--theme-surface-fg-muted)"),
    FamilyToken::new("drawer-close-fg-hover",       "var(--theme-surface-fg)"),
    FamilyToken::new("drawer-transition-duration",  "var(--motion-duration-normal)"),
    FamilyToken::new("drawer-transition-ease",      "var(--motion-ease-standard)"),
];
