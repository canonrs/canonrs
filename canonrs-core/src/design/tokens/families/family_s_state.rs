use crate::design::tokens::FamilyToken;

/// FAMILY S — Interactive States
/// Cross-cutting state modifiers for hover, active, disabled, focus
/// Scope: State transformations applied across all interactive components

pub const FAMILY_S_STATE: &[FamilyToken] = &[
    // Opacity states (valores reais, não referências)
    FamilyToken::new("state-hover-opacity", "0.9"),
    FamilyToken::new("state-active-opacity", "0.85"),
    FamilyToken::new("state-disabled-opacity", "0.5"),

    // Focus ring (definição canônica)
    FamilyToken::new("state-focus-ring-width", "2px"),
    FamilyToken::new("state-focus-ring-color", "var(--color-ring)"),
    FamilyToken::new("state-focus-ring-offset", "2px"),
    FamilyToken::new("state-focus-ring-style", "solid"),

    // Transitions
    FamilyToken::new("state-transition-duration", "var(--motion-duration-fast)"),
    FamilyToken::new("state-transition-ease", "var(--motion-ease-standard)"),

    // State flags (para data attributes)
    FamilyToken::new("active", "1"),
    FamilyToken::new("disabled", "1"),
];
