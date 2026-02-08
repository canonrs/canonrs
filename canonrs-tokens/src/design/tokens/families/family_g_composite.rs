use crate::design::tokens::FamilyToken;

/// FAMILY G — Composite Systems
/// Components: Workflow, DragDrop, FormBuilder, Kanban, VirtualTable, TreeView, Resizable, Carousel
/// Scope: Multi-state, interactive, composed components with complex interactions

pub const FAMILY_G_COMPOSITE: &[FamilyToken] = &[
    // Composite foundation
    FamilyToken::new("composite-gap", "var(--space-md)"),
    FamilyToken::new("composite-padding", "var(--space-md)"),
    FamilyToken::new("composite-radius", "var(--radius-md)"),

    // System states
    FamilyToken::new("composite-bg", "var(--theme-surface-bg)"),
    FamilyToken::new("composite-border-color", "var(--theme-surface-border)"),
    FamilyToken::new("composite-border-width", "1px"),

    // Selection (focus ring removido, usa state-*)
    FamilyToken::new("composite-active-bg", "var(--theme-action-accent-bg)"),
    FamilyToken::new("composite-selected-bg", "var(--theme-action-accent-bg)"),

    // Complex interactions
    FamilyToken::new("composite-drag-preview-bg", "var(--theme-surface-bg)"),
    FamilyToken::new("composite-dropzone-bg", "var(--theme-action-accent-bg)"),
    FamilyToken::new("composite-dropzone-border-width", "2px"),
    FamilyToken::new("composite-dropzone-border-style", "dashed"),
    FamilyToken::new("composite-dropzone-border-color", "var(--theme-action-primary-bg)"),

    // System motion
    FamilyToken::new("composite-transition-duration", "var(--motion-duration-normal)"),
    FamilyToken::new("composite-transition-ease", "var(--motion-ease-standard)"),

    // Resizable
    FamilyToken::new("resizable-bg", "var(--theme-surface-bg)"),
    FamilyToken::new("resizable-gap", "var(--space-sm)"),
    FamilyToken::new("resizable-border", "1px solid var(--theme-surface-border)"),
    FamilyToken::new("resizable-panel-bg", "transparent"),
    FamilyToken::new("resizable-panel-padding", "var(--space-md)"),
    FamilyToken::new("resizable-panel-radius", "var(--radius-md)"),
    FamilyToken::new("resizable-handle-size", "var(--space-xs)"),
    FamilyToken::new("resizable-handle-cursor", "col-resize"),
    FamilyToken::new("resizable-handle-bg", "var(--theme-surface-border)"),
    FamilyToken::new("resizable-handle-hover-bg", "var(--theme-action-primary-bg)"),
    FamilyToken::new("resizable-handle-disabled-bg", "var(--theme-surface-muted)"),

    // Carousel
    FamilyToken::new("carousel-bg", "transparent"),
    FamilyToken::new("carousel-padding", "0"),
    FamilyToken::new("carousel-content-gap", "var(--space-sm)"),
    FamilyToken::new("carousel-content-direction", "row"),
    FamilyToken::new("carousel-content-scroll-snap", "x mandatory"),
    FamilyToken::new("carousel-item-width", "auto"),
    FamilyToken::new("carousel-item-bg", "transparent"),
    FamilyToken::new("carousel-item-radius", "var(--radius-md)"),
    FamilyToken::new("carousel-item-shadow", "none"),
    FamilyToken::new("carousel-button-bg", "var(--theme-surface-bg)"),
    FamilyToken::new("carousel-button-fg", "var(--theme-surface-fg)"),
    FamilyToken::new("carousel-button-size", "var(--space-2xl)"),
    FamilyToken::new("carousel-button-radius", "var(--radius-full)"),
    FamilyToken::new("carousel-button-shadow", "var(--shadow-md)"),
    FamilyToken::new("carousel-indicator-size", "var(--space-sm)"),
    FamilyToken::new("carousel-indicator-gap", "var(--space-xs)"),
    FamilyToken::new("carousel-indicator-bg", "var(--theme-surface-muted)"),
    FamilyToken::new("carousel-indicator-active-bg", "var(--theme-action-primary-bg)"),
];
