// Foundation — Breakpoints
// Responsive breakpoint scale

use crate::design::tokens::FamilyToken;

pub const FOUNDATION_BREAKPOINTS: &[FamilyToken] = &[
    FamilyToken::new("breakpoint-xs",  "320px"),
    FamilyToken::new("breakpoint-sm",  "480px"),
    FamilyToken::new("breakpoint-md",  "768px"),
    FamilyToken::new("breakpoint-lg",  "1024px"),
    FamilyToken::new("breakpoint-xl",  "1280px"),
    FamilyToken::new("breakpoint-2xl", "1536px"),
];
