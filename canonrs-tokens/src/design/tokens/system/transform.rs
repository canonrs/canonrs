//! System — Transform
//! Motion transforms for interactive states.

use crate::design::tokens::FamilyToken;

pub const SYSTEM_TRANSFORM: &[FamilyToken] = &[
    FamilyToken::new("transform-active-press", "translateY(1px)"),
    FamilyToken::new("transform-scale-sm",     "scale(0.98)"),
    FamilyToken::new("transform-scale-md",     "scale(1.02)"),
];
