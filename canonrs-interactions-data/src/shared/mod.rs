//! Shared — re-exports runtime for backwards compatibility
pub use crate::runtime::state::{add as add_state, remove as remove_state};
pub use crate::runtime::lifecycle::{init_guard as is_initialized_check};

use web_sys::Element;

pub fn is_initialized(el: &Element) -> bool {
    el.get_attribute("data-rs-initialized").as_deref() == Some("true")
}

pub fn mark_initialized(el: &Element) {
    let _ = el.set_attribute("data-rs-initialized", "true");
}
