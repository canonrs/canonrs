// FOUNDATION TOKENS — CanonRS
//
// Foundation tokens define **physical, non-semantic system values**.
// They NEVER encode meaning or context.
// They are stable across themes and brands.
//
// Layering:
// Primitive   -> raw physical value
// Foundation  -> normalized system contract
// Semantic    -> meaning / intent
// Family      -> component application

pub struct FoundationToken {
    pub token: &'static str,
    pub value: &'static str,
}

pub const FOUNDATION_TOKENS: &[FoundationToken] = &[

    // ======================================================================
    // Spacing (layout rhythm)
    // ======================================================================

    FoundationToken { token: "space-2xs", value: "var(--primitive-space-0-5)" },
    FoundationToken { token: "space-xs",  value: "var(--primitive-space-xs)" },
    FoundationToken { token: "space-sm",  value: "var(--primitive-space-sm)" },
    FoundationToken { token: "space-md",  value: "var(--primitive-space-md)" },
    FoundationToken { token: "space-lg",  value: "var(--primitive-space-lg)" },
    FoundationToken { token: "space-xl",  value: "var(--primitive-space-xl)" },
    FoundationToken { token: "space-2xl", value: "var(--primitive-space-2xl)" },
    FoundationToken { token: "space-3xl", value: "var(--primitive-space-3xl)" },

    // ======================================================================
    // Radius (shape system)
    // ======================================================================

    FoundationToken { token: "radius-sm",   value: "var(--primitive-radius-sm)" },
    FoundationToken { token: "radius-md",   value: "var(--primitive-radius-md)" },
    FoundationToken { token: "radius-lg",   value: "var(--primitive-radius-lg)" },
    FoundationToken { token: "radius-xl",   value: "var(--primitive-radius-xl)" },
    FoundationToken { token: "radius-2xl",  value: "var(--primitive-radius-2xl)" },
    FoundationToken { token: "radius-full", value: "var(--primitive-radius-full)" },

    // ======================================================================
    // Shadow (elevation mechanics)
    // ======================================================================

    FoundationToken { token: "shadow-sm", value: "var(--primitive-shadow-sm)" },
    FoundationToken { token: "shadow-md", value: "var(--primitive-shadow-md)" },
    FoundationToken { token: "shadow-lg", value: "var(--primitive-shadow-lg)" },
    FoundationToken { token: "shadow-xl", value: "var(--primitive-shadow-xl)" },

    // ======================================================================
    // Typography (mechanical, not semantic)
    // ======================================================================

    FoundationToken { token: "font-family-sans", value: "var(--primitive-font-sans)" },
    FoundationToken { token: "font-family-mono", value: "var(--primitive-font-mono)" },

    FoundationToken { token: "font-size-xs",  value: "var(--primitive-font-size-xs)" },
    FoundationToken { token: "font-size-sm",  value: "var(--primitive-font-size-sm)" },
    FoundationToken { token: "font-size-md",  value: "var(--primitive-font-size-md)" },
    FoundationToken { token: "font-size-lg",  value: "var(--primitive-font-size-lg)" },
    FoundationToken { token: "font-size-xl",  value: "var(--primitive-font-size-xl)" },
    FoundationToken { token: "font-size-2xl", value: "var(--primitive-font-size-2xl)" },
    FoundationToken { token: "font-size-3xl", value: "var(--primitive-font-size-3xl)" },

    FoundationToken { token: "font-weight-regular",  value: "var(--primitive-font-weight-normal)" },
    FoundationToken { token: "font-weight-medium",   value: "var(--primitive-font-weight-medium)" },
    FoundationToken { token: "font-weight-semibold", value: "var(--primitive-font-weight-semibold)" },
    FoundationToken { token: "font-weight-bold",     value: "var(--primitive-font-weight-bold)" },

    FoundationToken { token: "line-height-tight",   value: "var(--primitive-line-height-tight)" },
    FoundationToken { token: "line-height-normal",  value: "var(--primitive-line-height-normal)" },
    FoundationToken { token: "line-height-relaxed", value: "var(--primitive-line-height-relaxed)" },

    // ======================================================================
    // Motion (timing & easing only)
    // ======================================================================

    FoundationToken { token: "motion-duration-fast",   value: "var(--primitive-motion-duration-fast)" },
    FoundationToken { token: "motion-duration-normal", value: "var(--primitive-motion-duration-normal)" },
    FoundationToken { token: "motion-duration-slow",   value: "var(--primitive-motion-duration-slow)" },

    FoundationToken { token: "motion-ease-linear",      value: "var(--primitive-motion-ease-linear)" },
    FoundationToken { token: "motion-ease-standard",    value: "var(--primitive-motion-ease-out)" },
    FoundationToken { token: "motion-ease-emphasized",  value: "var(--primitive-motion-ease-emphasized)" },

];

/// Explicit list used by validators and build tooling
pub const FOUNDATION_TOKEN_NAMES: &[&str] = &[
    // Space
    "space-2xs","space-xs","space-sm","space-md","space-lg","space-xl","space-2xl","space-3xl",

    // Radius
    "radius-sm","radius-md","radius-lg","radius-xl","radius-2xl","radius-full",

    // Shadow
    "shadow-sm","shadow-md","shadow-lg","shadow-xl",

    // Font
    "font-family-sans","font-family-mono",
    "font-size-xs","font-size-sm","font-size-md","font-size-lg","font-size-xl","font-size-2xl","font-size-3xl",
    "font-weight-regular","font-weight-medium","font-weight-semibold","font-weight-bold",
    "line-height-tight","line-height-normal","line-height-relaxed",

    // Motion
    "motion-duration-fast","motion-duration-normal","motion-duration-slow",
    "motion-ease-linear","motion-ease-standard","motion-ease-emphasized",
];
