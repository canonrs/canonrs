// Foundation — Interaction
// Opacity, focus ring, transform, blur.

use crate::design::tokens::FamilyToken;

pub const FOUNDATION_INTERACTION: &[FamilyToken] = &[
    // Opacity
    FamilyToken::new("opacity-disabled", "0.4"),
    FamilyToken::new("opacity-hover",    "0.92"),
    FamilyToken::new("opacity-active",   "0.85"),
    FamilyToken::new("opacity-overlay",  "0.6"),

    // Focus ring
    FamilyToken::new("focus-ring-width",  "2px"),
    FamilyToken::new("focus-ring-offset", "2px"),

    // Transforms
    FamilyToken::new("transform-active-press", "translateY(1px)"),
    FamilyToken::new("transform-scale-sm",     "scale(0.98)"),
    FamilyToken::new("transform-scale-md",     "scale(1.02)"),

    // Blur
    FamilyToken::new("blur-sm", "4px"),
    FamilyToken::new("blur-md", "8px"),
    FamilyToken::new("blur-lg", "16px"),
];
