use crate::design::tokens::FamilyToken;

/// DropdownMenu component tokens
/// Scope: floating menu, context menu

pub const DROPDOWN_TOKENS: &[FamilyToken] = &[
    FamilyToken::new("dropdown-menu-bg",                    "var(--theme-overlay-bg)"),
    FamilyToken::new("dropdown-menu-fg",                    "var(--theme-surface-fg)"),
    FamilyToken::new("dropdown-menu-radius",                "var(--radius-md)"),
    FamilyToken::new("dropdown-menu-shadow",                "var(--shadow-lg)"),
    FamilyToken::new("dropdown-menu-border-color",          "var(--theme-surface-border)"),
    FamilyToken::new("dropdown-menu-border-width",          "1px"),
    FamilyToken::new("dropdown-menu-min-width",             "var(--layout-width-sm)"),
    FamilyToken::new("dropdown-menu-z-index",               "var(--layer-overlay)"),
    FamilyToken::new("dropdown-menu-padding",               "var(--space-xs)"),
    FamilyToken::new("dropdown-menu-item-height",           "var(--space-xl)"),
    FamilyToken::new("dropdown-menu-item-padding",          "var(--space-sm)"),
    FamilyToken::new("dropdown-menu-item-hover-bg",         "var(--theme-action-accent-bg)"),
    FamilyToken::new("dropdown-menu-item-hover-fg",         "var(--theme-action-accent-fg)"),
    FamilyToken::new("dropdown-menu-item-selected-bg",      "var(--theme-action-accent-bg)"),
    FamilyToken::new("dropdown-menu-item-checked-bg",       "var(--theme-action-accent-bg)"),
    FamilyToken::new("dropdown-menu-item-checked-fg",       "var(--theme-action-accent-fg)"),
    FamilyToken::new("dropdown-menu-separator-color",       "var(--theme-surface-border)"),
    FamilyToken::new("dropdown-menu-separator-margin-y",    "var(--space-xs)"),
    FamilyToken::new("dropdown-menu-transition-duration",   "var(--motion-duration-fast)"),
    FamilyToken::new("dropdown-menu-transition-ease",       "var(--motion-ease-standard)"),
];
