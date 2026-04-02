//! Foundation — Motion
//! Duration and easing scale. References primitives only.

use crate::design::tokens::FamilyToken;

pub const FOUNDATION_MOTION: &[FamilyToken] = &[
    FamilyToken::new("motion-duration-instant",   "0ms"),
    FamilyToken::new("motion-duration-fast",      "var(--primitive-motion-duration-fast)"),
    FamilyToken::new("motion-duration-normal",    "var(--primitive-motion-duration-normal)"),
    FamilyToken::new("motion-duration-slow",      "var(--primitive-motion-duration-slow)"),
    FamilyToken::new("motion-duration-deliberate","500ms"),
    FamilyToken::new("motion-ease-linear",        "var(--primitive-motion-ease-linear)"),
    FamilyToken::new("motion-ease-standard",      "var(--primitive-motion-ease-standard)"),
    FamilyToken::new("motion-ease-decelerate",    "cubic-bezier(0, 0, 0.2, 1)"),
    FamilyToken::new("motion-ease-accelerate",    "cubic-bezier(0.4, 0, 1, 1)"),
    FamilyToken::new("motion-reduced-duration",   "0ms"),
];
