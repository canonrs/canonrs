// Foundation — Color
// Raw color values only. No opacity, transform, focus, blur.

use crate::design::tokens::FamilyToken;

pub const FOUNDATION_COLOR: &[FamilyToken] = &[
    FamilyToken::new("color-overlay-50",          "rgba(0, 0, 0, 0.5)"),
    FamilyToken::new("color-overlay-30",          "rgba(0, 0, 0, 0.3)"),
    FamilyToken::new("color-overlay-80",          "rgba(0, 0, 0, 0.8)"),
    FamilyToken::new("color-primary-alpha-05",    "rgba(99, 102, 241, 0.05)"),
    FamilyToken::new("color-primary-alpha-06",    "rgba(99, 102, 241, 0.06)"),
    FamilyToken::new("color-primary-alpha-08",    "rgba(99, 102, 241, 0.08)"),
    FamilyToken::new("color-primary-alpha-10",    "rgba(99, 102, 241, 0.10)"),
    FamilyToken::new("color-primary-alpha-12",    "rgba(99, 102, 241, 0.12)"),
    FamilyToken::new("color-primary-alpha-18",    "rgba(99, 102, 241, 0.18)"),
    FamilyToken::new("color-primary-alpha-20",    "rgba(99, 102, 241, 0.20)"),
    FamilyToken::new("color-primary-border-soft", "#a5b4fc"),
    FamilyToken::new("color-success-glow",        "hsl(142 71% 45% / 0.4)"),
    FamilyToken::new("color-divider-subtle",      "rgba(0, 0, 0, 0.08)"),
];
