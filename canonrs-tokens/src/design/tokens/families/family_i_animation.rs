//! Family I: Animation & Motion Infrastructure
//!
//! Purpose: Global motion system + Animate component
//! Components: Animate
//! Scope: Motion tokens, easing, duration scale, animation wrapper

use crate::design::tokens::FamilyToken;

pub const FAMILY_I_ANIMATION: &[FamilyToken] = &[
    // Duration scale (global motion system)
    FamilyToken::new("motion-duration-instant", "0ms"),
    FamilyToken::new("motion-duration-fast", "150ms"),
    FamilyToken::new("motion-duration-normal", "200ms"),
    FamilyToken::new("motion-duration-slow", "300ms"),
    FamilyToken::new("motion-duration-deliberate", "500ms"),

    // Easing system
    FamilyToken::new("motion-ease-standard", "cubic-bezier(0.4, 0, 0.2, 1)"),
    FamilyToken::new("motion-ease-decelerate", "cubic-bezier(0, 0, 0.2, 1)"),
    FamilyToken::new("motion-ease-accelerate", "cubic-bezier(0.4, 0, 1, 1)"),
    FamilyToken::new("motion-ease-linear", "linear"),

    // Motion accessibility
    FamilyToken::new("motion-reduced-duration", "0ms"),

    // Animate — animation wrapper component
    FamilyToken::new("animate-duration", "var(--motion-duration-normal)"),
    FamilyToken::new("animate-ease", "var(--motion-ease-standard)"),
    FamilyToken::new("animate-delay", "0ms"),
    FamilyToken::new("animate-fade-from", "0"),
    FamilyToken::new("animate-fade-to", "1"),
    FamilyToken::new("animate-slide-distance", "var(--space-sm)"),
    FamilyToken::new("animate-scale-from", "0.95"),
];
