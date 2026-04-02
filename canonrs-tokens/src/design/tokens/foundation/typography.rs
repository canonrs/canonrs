//! Foundation — Typography
//! Mechanical type scale. References primitives only.

use crate::design::tokens::FamilyToken;

pub const FOUNDATION_TYPOGRAPHY: &[FamilyToken] = &[
    FamilyToken::new("font-family-sans",  "var(--primitive-font-sans)"),
    FamilyToken::new("font-family-mono",  "var(--primitive-font-mono)"),
    FamilyToken::new("font-family-serif", "Georgia, serif"),

    FamilyToken::new("font-size-2xs", "var(--primitive-font-size-1)"),
    FamilyToken::new("font-size-xs",  "var(--primitive-font-size-1)"),
    FamilyToken::new("font-size-sm",  "var(--primitive-font-size-2)"),
    FamilyToken::new("font-size-md",  "var(--primitive-font-size-3)"),
    FamilyToken::new("font-size-base","var(--primitive-font-size-3)"),
    FamilyToken::new("font-size-lg",  "var(--primitive-font-size-4)"),
    FamilyToken::new("font-size-xl",  "var(--primitive-font-size-5)"),
    FamilyToken::new("font-size-2xl", "var(--primitive-font-size-6)"),
    FamilyToken::new("font-size-3xl", "1.875rem"),
    FamilyToken::new("font-size-4xl", "2.25rem"),
    FamilyToken::new("font-size-5xl", "3rem"),

    FamilyToken::new("font-weight-normal",   "var(--primitive-font-weight-400)"),
    FamilyToken::new("font-weight-regular",  "var(--primitive-font-weight-400)"),
    FamilyToken::new("font-weight-medium",   "var(--primitive-font-weight-500)"),
    FamilyToken::new("font-weight-semibold", "var(--primitive-font-weight-600)"),
    FamilyToken::new("font-weight-bold",     "var(--primitive-font-weight-700)"),

    FamilyToken::new("line-height-tight",   "var(--primitive-line-height-1)"),
    FamilyToken::new("line-height-normal",  "var(--primitive-line-height-2)"),
    FamilyToken::new("line-height-relaxed", "var(--primitive-line-height-3)"),

    FamilyToken::new("shadow-sm",  "var(--primitive-shadow-sm)"),
    FamilyToken::new("shadow-md",  "var(--primitive-shadow-md)"),
    FamilyToken::new("shadow-lg",  "var(--primitive-shadow-lg)"),
    FamilyToken::new("shadow-xl",  "0 20px 25px rgb(0 0 0 / 0.15)"),
    FamilyToken::new("shadow-2xl", "0 25px 50px rgb(0 0 0 / 0.25)"),
];
