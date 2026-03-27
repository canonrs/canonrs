//! CanonRS Style — declarative styling contracts
//! Pure Rust, zero Leptos dependency

pub mod style_contract;

pub use style_contract::{
    SpaceScale, Spacing,
    Align, Width, LayoutStyle,
    TextSize, TextWeight, TextAlign, Typography,
    Variant, ColorStyle,
    StyleProps,
    validate_style, style_from_json, StyleValidationError,
};
