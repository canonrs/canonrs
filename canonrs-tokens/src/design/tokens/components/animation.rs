//! Component tokens — Animate
//! Motion infrastructure for the Animate component.
//! Duration and easing reference foundation/motion tokens.

use crate::design::tokens::FamilyToken;

pub const ANIMATION_TOKENS: &[FamilyToken] = &[
    // Animate component — references foundation/motion
    FamilyToken::new("animate-duration",       "var(--motion-duration-normal)"),
    FamilyToken::new("animate-ease",           "var(--motion-ease-standard)"),
    FamilyToken::new("animate-delay",          "0ms"),
    FamilyToken::new("animate-fade-from",      "0"),
    FamilyToken::new("animate-fade-to",        "1"),
    FamilyToken::new("animate-slide-distance", "var(--space-sm)"),
    FamilyToken::new("animate-scale-from",     "0.95"),
    FamilyToken::new("animate-scale-to",       "1"),
];
