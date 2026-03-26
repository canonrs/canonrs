//! StyleContract — declarative styling layer
//! Sits between Block/Layout structure and CSS render
//! Zero Leptos dependency — pure Rust contracts

pub mod spacing;
pub mod layout_style;
pub mod typography;
pub mod color;
pub mod props;

pub use spacing::{SpaceScale, Spacing};
pub use layout_style::{Align, Width, LayoutStyle};
pub use typography::{TextSize, TextWeight, TextAlign, Typography};
pub use color::{Variant, ColorStyle};
pub use props::StyleProps;
