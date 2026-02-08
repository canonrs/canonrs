use crate::design::tokens::FamilyToken;

/// FOUNDATION TOKENS — System layer
/// Aliases over primitives
/// No color, no semantic, no component meaning

pub const CORE_TOKENS: &[FamilyToken] = &[

    // ======================================================================
    // Spacing system
    // ======================================================================

    FamilyToken::new("space-xs", "var(--primitive-space-1)"),
    FamilyToken::new("space-sm", "var(--primitive-space-2)"),
    FamilyToken::new("space-md", "var(--primitive-space-4)"),
    FamilyToken::new("space-lg", "var(--primitive-space-6)"),
    FamilyToken::new("space-xl", "var(--primitive-space-8)"),

    // ======================================================================
    // Radius system
    // ======================================================================

    FamilyToken::new("radius-sm", "var(--primitive-radius-1)"),
    FamilyToken::new("radius-md", "var(--primitive-radius-2)"),
    FamilyToken::new("radius-lg", "var(--primitive-radius-3)"),
    FamilyToken::new("radius-full", "var(--primitive-radius-full)"),

    // ======================================================================
    // Typography system
    // ======================================================================

    FamilyToken::new("font-family-sans", "var(--primitive-font-sans)"),
    FamilyToken::new("font-family-mono", "var(--primitive-font-mono)"),

    FamilyToken::new("font-size-sm", "var(--primitive-font-size-2)"),
    FamilyToken::new("font-size-md", "var(--primitive-font-size-3)"),
    FamilyToken::new("font-size-lg", "var(--primitive-font-size-4)"),

    FamilyToken::new("font-weight-regular", "var(--primitive-font-weight-400)"),
    FamilyToken::new("font-weight-medium", "var(--primitive-font-weight-500)"),
    FamilyToken::new("font-weight-semibold", "var(--primitive-font-weight-600)"),

    FamilyToken::new("line-height-tight", "var(--primitive-line-height-1)"),
    FamilyToken::new("line-height-normal", "var(--primitive-line-height-2)"),
    FamilyToken::new("line-height-relaxed", "var(--primitive-line-height-3)"),

    // ======================================================================
    // Shadows
    // ======================================================================

    FamilyToken::new("shadow-sm", "var(--primitive-shadow-sm)"),
    FamilyToken::new("shadow-md", "var(--primitive-shadow-md)"),
    FamilyToken::new("shadow-lg", "var(--primitive-shadow-lg)"),

    // ======================================================================
    // Motion
    // ======================================================================

    FamilyToken::new("motion-duration-fast", "var(--primitive-motion-duration-fast)"),
    FamilyToken::new("motion-duration-normal", "var(--primitive-motion-duration-normal)"),
    FamilyToken::new("motion-duration-slow", "var(--primitive-motion-duration-slow)"),

    FamilyToken::new("motion-ease-standard", "var(--primitive-motion-ease-standard)"),

    // ======================================================================
    // Z-index
    // ======================================================================

    FamilyToken::new("z-base", "var(--primitive-z-base)"),
    FamilyToken::new("z-dropdown", "var(--primitive-z-dropdown)"),
    FamilyToken::new("z-overlay", "var(--primitive-z-overlay)"),
    FamilyToken::new("z-modal", "var(--primitive-z-modal)"),
];
