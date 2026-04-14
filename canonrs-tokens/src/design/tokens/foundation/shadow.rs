// Foundation — Shadow
// Elevation scale — all values from primitives.

use crate::design::tokens::FamilyToken;

pub const FOUNDATION_SHADOW: &[FamilyToken] = &[
    FamilyToken::new("shadow-xs",  "var(--primitive-shadow-xs)"),
    FamilyToken::new("shadow-sm",  "var(--primitive-shadow-sm)"),
    FamilyToken::new("shadow-md",  "var(--primitive-shadow-md)"),
    FamilyToken::new("shadow-lg",  "var(--primitive-shadow-lg)"),
    FamilyToken::new("shadow-xl",  "var(--primitive-shadow-xl)"),
    FamilyToken::new("shadow-2xl", "var(--primitive-shadow-2xl)"),
];
