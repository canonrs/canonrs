use crate::design::tokens::FamilyToken;

// Components: Overlay — Dialog, Sheet, Popover, Drawer, HoverCard, Tooltip, DropdownMenu, Modal
/// Components: Dialog, Sheet, Popover, Drawer, HoverCard, Tooltip, DropdownMenu, ContextMenu, Modal
/// Scope: Overlays, modals, popovers, floating elements

pub const OVERLAY_TOKENS: &[FamilyToken] = &[
    // Overlay foundation
    FamilyToken::new("overlay-z-index", "var(--layer-overlay)"),
    FamilyToken::new("overlay-backdrop-bg", "var(--color-overlay-50)"),
    FamilyToken::new("overlay-backdrop-blur", "4px"),
    FamilyToken::new("overlay-transition-duration", "var(--motion-duration-normal)"),
    FamilyToken::new("overlay-transition-ease", "var(--motion-ease-standard)"),

    // Dialog
    FamilyToken::new("dialog-overlay-bg", "var(--color-overlay-50)"),
    FamilyToken::new("dialog-overlay-z-index", "var(--layer-overlay)"),
    FamilyToken::new("dialog-content-bg", "var(--theme-surface-bg)"),
    FamilyToken::new("dialog-content-fg", "var(--theme-surface-fg)"),
    FamilyToken::new("dialog-content-width", "var(--layout-width-lg)"),
    FamilyToken::new("dialog-content-max-width", "90vw"),
    FamilyToken::new("dialog-content-padding", "var(--space-lg)"),
    FamilyToken::new("dialog-content-radius", "var(--radius-md)"),
    FamilyToken::new("dialog-content-shadow", "var(--shadow-2xl)"),
    FamilyToken::new("dialog-header-gap", "var(--space-sm)"),
    FamilyToken::new("dialog-title-fg", "var(--theme-surface-fg)"),
    FamilyToken::new("dialog-title-font-size", "var(--font-size-lg)"),
    FamilyToken::new("dialog-title-font-weight", "var(--font-weight-semibold)"),
    FamilyToken::new("dialog-description-fg", "var(--theme-surface-fg-muted)"),
    FamilyToken::new("dialog-description-font-size", "var(--font-size-sm)"),
    FamilyToken::new("dialog-footer-gap", "var(--space-sm)"),
    FamilyToken::new("dialog-close-fg", "var(--theme-surface-fg-muted)"),
    FamilyToken::new("dialog-close-fg-hover", "var(--theme-surface-fg)"),
    FamilyToken::new("dialog-bg", "var(--theme-surface-bg)"),
    FamilyToken::new("dialog-width", "var(--layout-width-lg)"),
    FamilyToken::new("dialog-max-width", "90vw"),
    FamilyToken::new("dialog-padding", "var(--space-lg)"),
    FamilyToken::new("dialog-radius", "var(--radius-md)"),
    FamilyToken::new("dialog-shadow", "var(--shadow-2xl)"),

    // Modal (alias for Dialog with specific sizing)
    FamilyToken::new("modal-popup-width", "32rem"),
    FamilyToken::new("modal-popup-padding", "var(--space-lg)"),
    FamilyToken::new("modal-popup-radius", "var(--radius-lg)"),
    FamilyToken::new("modal-popup-bg", "var(--theme-surface-bg)"),
    FamilyToken::new("modal-backdrop-bg", "var(--color-overlay-50)"),

    // DropdownMenu
    FamilyToken::new("dropdown-menu-bg", "var(--theme-overlay-bg)"),
    FamilyToken::new("dropdown-menu-fg", "var(--color-popover-foreground)"),
    FamilyToken::new("dropdown-menu-padding-y", "var(--space-xs)"),
    FamilyToken::new("dropdown-menu-radius", "var(--radius-md)"),
    FamilyToken::new("dropdown-menu-shadow", "var(--shadow-lg)"),
    FamilyToken::new("dropdown-menu-border-color", "var(--theme-surface-border)"),
    FamilyToken::new("dropdown-menu-border-width", "1px"),
    FamilyToken::new("dropdown-menu-min-width", "var(--layout-width-sm)"),
    FamilyToken::new("dropdown-menu-z-index", "var(--layer-overlay)"),
    FamilyToken::new("dropdown-menu-item-height", "var(--space-xl)"),
    FamilyToken::new("dropdown-menu-item-padding", "var(--space-sm)"),
    FamilyToken::new("dropdown-menu-item-hover-bg", "var(--theme-action-accent-bg)"),
    FamilyToken::new("dropdown-menu-item-selected-bg", "var(--theme-action-accent-bg)"),
    FamilyToken::new("dropdown-menu-separator-color", "var(--theme-surface-border)"),
    FamilyToken::new("dropdown-menu-separator-margin-y", "var(--space-xs)"),
    FamilyToken::new("dropdown-menu-padding", "var(--space-xs)"),
    FamilyToken::new("dropdown-menu-transition-duration", "var(--motion-duration-fast)"),
    FamilyToken::new("dropdown-menu-transition-ease", "var(--motion-ease-standard)"),
    FamilyToken::new("dropdown-menu-item-hover-fg", "var(--theme-action-accent-fg)"),
    FamilyToken::new("dropdown-menu-item-checked-bg", "var(--theme-action-accent-bg)"),
    FamilyToken::new("dropdown-menu-item-checked-fg", "var(--theme-action-accent-fg)"),

];
