//! Foundation — Radius
//! Shape scale. References primitives only.

use crate::design::tokens::FamilyToken;

pub const FOUNDATION_RADIUS: &[FamilyToken] = &[
    FamilyToken::new("radius-xs",   "var(--primitive-radius-1)"),
    FamilyToken::new("radius-sm",   "var(--primitive-radius-1)"),
    FamilyToken::new("radius-md",   "var(--primitive-radius-2)"),
    FamilyToken::new("radius-lg",   "var(--primitive-radius-3)"),
    FamilyToken::new("radius-xl",   "var(--primitive-radius-3)"),
    FamilyToken::new("radius-2xl",  "var(--primitive-radius-3)"),
    FamilyToken::new("radius-full", "var(--primitive-radius-full)"),
];
