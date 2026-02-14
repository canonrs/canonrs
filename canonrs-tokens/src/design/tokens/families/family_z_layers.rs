//! Family Z: Layers & Stacking Context
//!
//! Purpose: Categorized z-index system by stacking context
//! Components never use literal z-index values

use crate::design::tokens::FamilyToken;

pub const FAMILY_Z_LAYERS: &[FamilyToken] = &[
    // Structural layout
    FamilyToken::new("layer-base", "0"),
    FamilyToken::new("layer-header", "100"),
    FamilyToken::new("layer-sticky", "200"),

    // Context overlays
    FamilyToken::new("layer-dropdown", "1000"),
    FamilyToken::new("layer-popover", "1100"),

    // Blocking layers
    FamilyToken::new("layer-overlay", "2000"),
    FamilyToken::new("layer-modal", "2100"),

    // Ephemeral UI
    FamilyToken::new("layer-tooltip", "3000"),
    FamilyToken::new("layer-toast", "4000"),
];
