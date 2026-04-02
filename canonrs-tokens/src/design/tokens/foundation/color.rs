// Foundation — Color
// Raw color values for use as CSS tokens.
// NO semantic meaning here — just physical values.

use crate::design::tokens::FamilyToken;

pub const FOUNDATION_COLOR: &[FamilyToken] = &[
    // Overlay alpha
    FamilyToken::new("color-overlay-50",          "rgba(0, 0, 0, 0.5)"),
    FamilyToken::new("color-overlay-30",          "rgba(0, 0, 0, 0.3)"),
    FamilyToken::new("color-overlay-80",          "rgba(0, 0, 0, 0.8)"),

    // Primary alpha scale
    FamilyToken::new("color-primary-alpha-05",    "rgba(99, 102, 241, 0.05)"),
    FamilyToken::new("color-primary-alpha-06",    "rgba(99, 102, 241, 0.06)"),
    FamilyToken::new("color-primary-alpha-08",    "rgba(99, 102, 241, 0.08)"),
    FamilyToken::new("color-primary-alpha-10",    "rgba(99, 102, 241, 0.10)"),
    FamilyToken::new("color-primary-alpha-12",    "rgba(99, 102, 241, 0.12)"),
    FamilyToken::new("color-primary-alpha-18",    "rgba(99, 102, 241, 0.18)"),
    FamilyToken::new("color-primary-alpha-20",    "rgba(99, 102, 241, 0.20)"),

    // Primary border soft
    FamilyToken::new("color-primary-border-soft", "#a5b4fc"),

    // Success alpha
    FamilyToken::new("color-success-glow",        "hsl(142 71% 45% / 0.4)"),

    // Layout divider
    FamilyToken::new("color-divider-subtle",      "rgba(0, 0, 0, 0.08)"),
];
