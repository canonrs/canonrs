//! CanonRS Interaction Engine

#[cfg(target_arch = "wasm32")]
pub mod resizable;
#[cfg(target_arch = "wasm32")]
pub mod scroll_area;
#[cfg(target_arch = "wasm32")]
pub mod chart;
#[cfg(target_arch = "wasm32")]
pub mod carousel;
#[cfg(target_arch = "wasm32")]
pub mod toggle_group;
#[cfg(target_arch = "wasm32")]
pub mod avatar;
