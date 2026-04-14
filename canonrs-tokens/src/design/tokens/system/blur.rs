//! System — Blur
//! Backdrop and element blur scale.

use crate::design::tokens::FamilyToken;

pub const SYSTEM_BLUR: &[FamilyToken] = &[
    FamilyToken::new("blur-sm", "4px"),
    FamilyToken::new("blur-md", "8px"),
    FamilyToken::new("blur-lg", "16px"),
];
