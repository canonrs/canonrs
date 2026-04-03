// Foundation — Typography
// Mechanical type scale only.

use crate::design::tokens::FamilyToken;

pub const FOUNDATION_TYPOGRAPHY: &[FamilyToken] = &[
    FamilyToken::new("font-family-sans",  "var(--primitive-font-sans)"),
    FamilyToken::new("font-family-mono",  "var(--primitive-font-mono)"),
    FamilyToken::new("font-family-serif", "var(--primitive-font-serif)"),

    FamilyToken::new("font-size-2xs", "0.625rem"),
    FamilyToken::new("font-size-xs",  "0.75rem"),
    FamilyToken::new("font-size-sm",  "0.875rem"),
    FamilyToken::new("font-size-md",  "1rem"),
    FamilyToken::new("font-size-base","1rem"),
    FamilyToken::new("font-size-lg",  "1.125rem"),
    FamilyToken::new("font-size-xl",  "1.25rem"),
    FamilyToken::new("font-size-2xl", "1.5rem"),
    FamilyToken::new("font-size-3xl", "1.875rem"),
    FamilyToken::new("font-size-4xl", "2.25rem"),
    FamilyToken::new("font-size-5xl", "3rem"),

    FamilyToken::new("font-weight-normal",   "400"),
    FamilyToken::new("font-weight-medium",   "500"),
    FamilyToken::new("font-weight-semibold", "600"),
    FamilyToken::new("font-weight-bold",     "700"),

    FamilyToken::new("line-height-tight",   "1.2"),
    FamilyToken::new("line-height-normal",  "1.5"),
    FamilyToken::new("line-height-relaxed", "1.7"),
];
