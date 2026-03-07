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
    FamilyToken::new("composite-bg", "var(--color-background)"),
    FamilyToken::new("composite-border-color", "var(--color-border)"),
    FamilyToken::new("composite-border-width", "1px"),

    // Selection (focus ring removido, usa state-*)
    FamilyToken::new("composite-active-bg", "var(--color-accent)"),
    FamilyToken::new("composite-selected-bg", "var(--color-accent)"),

    // Complex interactions
    FamilyToken::new("composite-drag-preview-bg", "var(--color-background)"),
    FamilyToken::new("composite-dropzone-bg", "var(--color-accent)"),
    FamilyToken::new("composite-dropzone-border-width", "2px"),
    FamilyToken::new("composite-dropzone-border-style", "dashed"),
    FamilyToken::new("composite-dropzone-border-color", "var(--color-primary)"),

    // System motion
    FamilyToken::new("composite-transition-duration", "var(--motion-duration-normal)"),
    FamilyToken::new("composite-transition-ease", "var(--motion-ease-standard)"),

    // Resizable
    FamilyToken::new("resizable-bg", "var(--color-background)"),
    FamilyToken::new("resizable-gap", "var(--space-sm)"),
    FamilyToken::new("resizable-border", "1px solid var(--color-border)"),
    FamilyToken::new("resizable-panel-bg", "transparent"),
    FamilyToken::new("resizable-panel-padding", "var(--space-md)"),
    FamilyToken::new("resizable-panel-radius", "var(--radius-md)"),
    FamilyToken::new("resizable-handle-size", "var(--space-xs)"),
    FamilyToken::new("resizable-handle-cursor", "col-resize"),
    FamilyToken::new("resizable-handle-bg", "var(--color-border)"),
    FamilyToken::new("resizable-handle-hover-bg", "var(--color-primary)"),
    FamilyToken::new("resizable-handle-disabled-bg", "var(--color-muted)"),

    // Carousel
    FamilyToken::new("carousel-bg", "transparent"),
    FamilyToken::new("carousel-padding", "0"),
    FamilyToken::new("carousel-item-bg", "transparent"),
    FamilyToken::new("carousel-item-radius", "var(--radius-md)"),
    FamilyToken::new("carousel-item-shadow", "none"),
];
