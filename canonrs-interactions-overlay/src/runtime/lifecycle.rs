//! Lifecycle — init guard e mark
use web_sys::Element;
use crate::shared::{is_initialized, mark_initialized};

pub fn init_guard(el: &Element) -> bool {
    if is_initialized(el) { return false; }
    mark_initialized(el);
    true
}
