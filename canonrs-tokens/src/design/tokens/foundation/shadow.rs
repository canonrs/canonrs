// Foundation — Shadow
// Elevation scale.

use crate::design::tokens::FamilyToken;

pub const FOUNDATION_SHADOW: &[FamilyToken] = &[
    FamilyToken::new("shadow-xs",  "0 1px 2px rgba(0,0,0,0.05)"),
    FamilyToken::new("shadow-sm",  "var(--primitive-shadow-sm)"),
    FamilyToken::new("shadow-md",  "var(--primitive-shadow-md)"),
    FamilyToken::new("shadow-lg",  "var(--primitive-shadow-lg)"),
    FamilyToken::new("shadow-xl",  "0 12px 24px rgba(0,0,0,0.12)"),
    FamilyToken::new("shadow-2xl", "0 20px 40px rgba(0,0,0,0.14)"),
];
