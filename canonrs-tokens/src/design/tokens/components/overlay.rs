use crate::design::tokens::FamilyToken;

/// Overlay infrastructure tokens — shared foundation for all overlay components
/// Scope: z-index, backdrop, transitions

pub const OVERLAY_TOKENS: &[FamilyToken] = &[
    FamilyToken::new("overlay-z-index",            "var(--layer-overlay)"),
    FamilyToken::new("overlay-backdrop-bg",         "var(--color-overlay-50)"),
    FamilyToken::new("overlay-backdrop-blur",       "4px"),
    FamilyToken::new("overlay-transition-duration", "var(--motion-duration-normal)"),
    FamilyToken::new("overlay-transition-ease",     "var(--motion-ease-standard)"),
];
