use crate::design::tokens::FamilyToken;

/// FAMILY Z — Layers & Elevation
/// Z-index canonical values for stacking context
pub const FAMILY_Z_LAYERS: &[FamilyToken] = &[
    FamilyToken::new("layer-base", "0"),
    FamilyToken::new("layer-dropdown", "1000"),
    FamilyToken::new("layer-sticky", "1100"),
    FamilyToken::new("layer-overlay", "1200"),
    FamilyToken::new("layer-popover", "1300"),
    FamilyToken::new("layer-tooltip", "1400"),
    FamilyToken::new("layer-modal", "1500"),
    FamilyToken::new("layer-toast", "1600"),
];
