// Family S: Interactive State Transformations
//
// Purpose: Visual transformations for states - NOT boolean flags
// State logic lives in data-attributes, not tokens

use crate::design::tokens::FamilyToken;

pub const STATE_TOKENS: &[FamilyToken] = &[
    // Opacity modifiers
    FamilyToken::new("state-opacity-hover",    "0.92"),
    FamilyToken::new("state-opacity-active",   "0.85"),
    FamilyToken::new("state-opacity-disabled", "0.5"),

    // Aliases usados por componentes
    FamilyToken::new("state-hover-opacity",    "0.92"),
    FamilyToken::new("state-disabled-opacity", "0.5"),

    // Focus system
    FamilyToken::new("state-focus-ring-width",  "2px"),
    FamilyToken::new("state-focus-ring-offset", "2px"),
    FamilyToken::new("state-focus-ring-style",  "solid"),
    FamilyToken::new("state-focus-ring-color",  "var(--theme-action-focus-ring)"),

    // Transition infrastructure
    FamilyToken::new("focus-ring-shadow",          "0 0 0 var(--state-focus-ring-width) var(--state-focus-ring-color)"),
    FamilyToken::new("state-transition-duration", "var(--motion-duration-fast)"),
    FamilyToken::new("state-transition-ease",     "var(--motion-ease-standard)"),
];
