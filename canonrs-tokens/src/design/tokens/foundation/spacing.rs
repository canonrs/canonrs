//! Foundation — Spacing
//! Physical space scale. References primitives only.

use crate::design::tokens::FamilyToken;

pub const FOUNDATION_SPACING: &[FamilyToken] = &[
    FamilyToken::new("space-2xs", "var(--primitive-space-1)"),
    FamilyToken::new("space-xs",  "var(--primitive-space-2)"),
    FamilyToken::new("space-sm",  "var(--primitive-space-3)"),
    FamilyToken::new("space-md",  "var(--primitive-space-4)"),
    FamilyToken::new("space-lg",  "var(--primitive-space-6)"),
    FamilyToken::new("space-xl",  "var(--primitive-space-8)"),
    FamilyToken::new("space-2xl", "var(--primitive-space-12)"),
    FamilyToken::new("space-3xl", "var(--primitive-space-16)"),
    FamilyToken::new("space-4xl", "var(--primitive-space-24)"),
    FamilyToken::new("space-xxs", "var(--primitive-space-1)"),
];
