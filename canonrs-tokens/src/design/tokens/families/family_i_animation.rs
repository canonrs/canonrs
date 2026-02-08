//! Family I: Animation & Transitions
//!
//! Components: animate
//!
//! Purpose: Animation primitives, transitions, and motion controls

use crate::design::tokens::FamilyToken;

pub const FAMILY_I_ANIMATION: &[FamilyToken] = &[
    // Animate - Core animation tokens
    FamilyToken::new("animate-duration-fast", "150ms"),
    FamilyToken::new("animate-duration-normal", "300ms"),
    FamilyToken::new("animate-duration-slow", "500ms"),
    
    FamilyToken::new("animate-easing-ease-in", "cubic-bezier(0.4, 0, 1, 1)"),
    FamilyToken::new("animate-easing-ease-out", "cubic-bezier(0, 0, 0.2, 1)"),
    FamilyToken::new("animate-easing-ease-in-out", "cubic-bezier(0.4, 0, 0.2, 1)"),
    FamilyToken::new("animate-easing-linear", "linear"),
    FamilyToken::new("animate-easing-spring", "cubic-bezier(0.34, 1.56, 0.64, 1)"),
    
    FamilyToken::new("animate-scale-enter", "0.95"),
    FamilyToken::new("animate-scale-exit", "0.95"),
    
    FamilyToken::new("animate-opacity-enter", "0"),
    FamilyToken::new("animate-opacity-exit", "0"),
    
    FamilyToken::new("animate-translate-enter", "-10px"),
    FamilyToken::new("animate-translate-exit", "10px"),
];
