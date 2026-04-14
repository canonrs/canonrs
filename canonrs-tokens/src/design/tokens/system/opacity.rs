//! System — Opacity
//! Visual opacity modifiers for interactive states.

use crate::design::tokens::FamilyToken;

pub const SYSTEM_OPACITY: &[FamilyToken] = &[
    FamilyToken::new("opacity-disabled", "0.4"),
    FamilyToken::new("opacity-hover",    "0.92"),
    FamilyToken::new("opacity-active",   "0.85"),
    FamilyToken::new("opacity-overlay",  "0.6"),
];
