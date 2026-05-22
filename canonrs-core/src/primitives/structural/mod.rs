//! Structural primitives — semantic DOM nodes without visual CSS
//! These have DOM presence and semantic meaning but no design tokens.

pub mod hidden_input;
pub mod null_view;
pub mod orientation;

pub use hidden_input::HiddenInputPrimitive;
pub use null_view::NullViewPrimitive;
pub use orientation::Orientation;
pub mod toc_item;
pub use toc_item::TocItem;
