use crate::design::tokens::FamilyToken;

/// FAMILY A — Overlay & Portals
/// Components: Dialog, Sheet, Popover, Drawer, HoverCard, Tooltip, DropdownMenu
/// Scope: Overlays, modals, popovers, floating elements

pub const FAMILY_A_OVERLAY: &[FamilyToken] = &[
    // Overlay foundation
    FamilyToken::new("overlay-z-index", "var(--layer-overlay)"),
    FamilyToken::new("overlay-backdrop-bg", "rgba(0, 0, 0, 0.5)"),
    FamilyToken::new("overlay-backdrop-blur", "4px"),
    FamilyToken::new("overlay-transition-duration", "var(--motion-duration-normal)"),
    FamilyToken::new("overlay-transition-ease", "var(--motion-ease-standard)"),

    // Dialog
    FamilyToken::new("dialog-overlay-bg", "rgba(0, 0, 0, 0.5)"),
    FamilyToken::new("dialog-overlay-z-index", "var(--layer-overlay)"),
    FamilyToken::new("dialog-content-bg", "var(--color-background)"),
    FamilyToken::new("dialog-content-fg", "var(--color-foreground)"),
    FamilyToken::new("dialog-content-width", "var(--layout-width-lg)"),
    FamilyToken::new("dialog-content-max-width", "90vw"),
    FamilyToken::new("dialog-content-padding", "var(--space-lg)"),
    FamilyToken::new("dialog-content-radius", "var(--radius-md)"),
    FamilyToken::new("dialog-content-shadow", "var(--shadow-2xl)"),
    FamilyToken::new("dialog-header-gap", "var(--space-sm)"),
    FamilyToken::new("dialog-title-fg", "var(--color-foreground)"),
    FamilyToken::new("dialog-title-font-size", "var(--font-size-lg)"),
    FamilyToken::new("dialog-title-font-weight", "var(--font-weight-semibold)"),
    FamilyToken::new("dialog-description-fg", "var(--color-muted-foreground)"),
    FamilyToken::new("dialog-description-font-size", "var(--font-size-sm)"),
    FamilyToken::new("dialog-footer-gap", "var(--space-sm)"),
    FamilyToken::new("dialog-close-fg", "var(--color-muted-foreground)"),
    FamilyToken::new("dialog-close-fg-hover", "var(--color-foreground)"),

    // Sheet
    FamilyToken::new("sheet-overlay-bg", "rgba(0, 0, 0, 0.5)"),
    FamilyToken::new("sheet-overlay-z-index", "var(--layer-overlay)"),
    FamilyToken::new("sheet-content-bg", "var(--color-background)"),
    FamilyToken::new("sheet-content-fg", "var(--color-foreground)"),
    FamilyToken::new("sheet-content-width", "var(--layout-width-md)"),
    FamilyToken::new("sheet-content-padding", "var(--space-lg)"),
    FamilyToken::new("sheet-content-shadow", "var(--shadow-2xl)"),
    FamilyToken::new("sheet-header-gap", "var(--space-sm)"),
    FamilyToken::new("sheet-title-fg", "var(--color-foreground)"),
    FamilyToken::new("sheet-title-font-size", "var(--font-size-lg)"),
    FamilyToken::new("sheet-title-font-weight", "var(--font-weight-semibold)"),
    FamilyToken::new("sheet-description-fg", "var(--color-muted-foreground)"),
    FamilyToken::new("sheet-description-font-size", "var(--font-size-sm)"),
    FamilyToken::new("sheet-footer-gap", "var(--space-sm)"),
    FamilyToken::new("sheet-close-fg", "var(--color-muted-foreground)"),

    // Popover
    FamilyToken::new("popover-bg", "var(--semantic-overlay-bg)"),
    FamilyToken::new("popover-fg", "var(--semantic-overlay-fg)"),
    FamilyToken::new("popover-padding", "var(--space-md)"),
    FamilyToken::new("popover-radius", "var(--radius-md)"),
    FamilyToken::new("popover-shadow", "var(--shadow-lg)"),
    FamilyToken::new("popover-border-color", "var(--color-border)"),
    FamilyToken::new("popover-border-width", "1px"),
    FamilyToken::new("popover-z-index", "var(--layer-overlay)"),
    FamilyToken::new("popover-arrow-size", "var(--space-sm)"),

    // Drawer
    FamilyToken::new("drawer-overlay-bg", "rgba(0, 0, 0, 0.5)"),
    FamilyToken::new("drawer-overlay-z-index", "var(--layer-overlay)"),
    FamilyToken::new("drawer-content-bg", "var(--color-background)"),
    FamilyToken::new("drawer-content-fg", "var(--color-foreground)"),
    FamilyToken::new("drawer-content-width", "var(--layout-width-md)"),
    FamilyToken::new("drawer-content-padding", "var(--space-lg)"),
    FamilyToken::new("drawer-content-shadow", "var(--shadow-2xl)"),
    FamilyToken::new("drawer-header-gap", "var(--space-sm)"),
    FamilyToken::new("drawer-title-fg", "var(--color-foreground)"),
    FamilyToken::new("drawer-title-font-size", "var(--font-size-lg)"),
    FamilyToken::new("drawer-title-font-weight", "var(--font-weight-semibold)"),
    FamilyToken::new("drawer-description-fg", "var(--color-muted-foreground)"),
    FamilyToken::new("drawer-footer-gap", "var(--space-sm)"),
    FamilyToken::new("drawer-handle-bg", "var(--color-muted)"),
    FamilyToken::new("drawer-handle-width", "var(--space-2xl)"),
    FamilyToken::new("drawer-handle-height", "var(--space-xs)"),
    FamilyToken::new("drawer-handle-radius", "var(--radius-full)"),

    // HoverCard
    FamilyToken::new("hover-card-bg", "var(--semantic-overlay-bg)"),
    FamilyToken::new("hover-card-fg", "var(--semantic-overlay-fg)"),
    FamilyToken::new("hover-card-padding", "var(--space-md)"),
    FamilyToken::new("hover-card-radius", "var(--radius-md)"),
    FamilyToken::new("hover-card-shadow", "var(--shadow-lg)"),
    FamilyToken::new("hover-card-border-color", "var(--color-border)"),
    FamilyToken::new("hover-card-border-width", "1px"),
    FamilyToken::new("hover-card-z-index", "var(--layer-overlay)"),
    FamilyToken::new("hover-card-arrow-size", "var(--space-sm)"),

    // Tooltip
    FamilyToken::new("tooltip-bg", "var(--semantic-overlay-bg)"),
    FamilyToken::new("tooltip-fg", "var(--semantic-overlay-fg)"),
    FamilyToken::new("tooltip-padding-x", "var(--space-sm)"),
    FamilyToken::new("tooltip-padding-y", "var(--space-sm)"),
    FamilyToken::new("tooltip-radius", "var(--radius-sm)"),
    FamilyToken::new("tooltip-font-size", "var(--font-size-sm)"),
    FamilyToken::new("tooltip-shadow", "var(--shadow-md)"),
    FamilyToken::new("tooltip-z-index", "var(--layer-overlay)"),
    FamilyToken::new("tooltip-arrow-size", "var(--space-xs)"),
    FamilyToken::new("tooltip-max-width", "var(--layout-width-md)"),

    // DropdownMenu
    FamilyToken::new("dropdown-menu-bg", "var(--semantic-overlay-bg)"),
    FamilyToken::new("dropdown-menu-fg", "var(--semantic-overlay-fg)"),
    FamilyToken::new("dropdown-menu-padding-y", "var(--space-xs)"),
    FamilyToken::new("dropdown-menu-radius", "var(--radius-md)"),
    FamilyToken::new("dropdown-menu-shadow", "var(--shadow-lg)"),
    FamilyToken::new("dropdown-menu-border-color", "var(--color-border)"),
    FamilyToken::new("dropdown-menu-border-width", "1px"),
    FamilyToken::new("dropdown-menu-min-width", "var(--layout-width-sm)"),
    FamilyToken::new("dropdown-menu-z-index", "var(--layer-overlay)"),
    FamilyToken::new("dropdown-menu-item-height", "var(--space-xl)"),
    FamilyToken::new("dropdown-menu-item-padding", "var(--space-sm)"),
    FamilyToken::new("dropdown-menu-item-hover-bg", "var(--color-accent)"),
    FamilyToken::new("dropdown-menu-item-selected-bg", "var(--color-accent)"),
    FamilyToken::new("dropdown-menu-separator-color", "var(--color-border)"),
    FamilyToken::new("dropdown-menu-separator-margin-y", "var(--space-xs)"),
    // Confirm Dialog (has interactive version)
    FamilyToken::new("confirm-dialog-bg", "var(--dialog-content-bg)"),
    FamilyToken::new("confirm-dialog-border", "var(--dialog-content-border)"),
    FamilyToken::new("confirm-dialog-shadow", "var(--dialog-content-shadow)"),

    // Modal
    FamilyToken::new("modal-bg", "var(--dialog-content-bg)"),
    FamilyToken::new("modal-max-width", "90vw"),
    FamilyToken::new("modal-padding", "var(--space-xl)"),

];
