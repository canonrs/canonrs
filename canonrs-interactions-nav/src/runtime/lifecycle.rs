//! Lifecycle — init guard
use web_sys::Element;

pub fn is_initialized(el: &Element) -> bool {
    el.get_attribute("data-rs-initialized").as_deref() == Some("true")
}

pub fn mark_initialized(el: &Element) {
    let _ = el.set_attribute("data-rs-initialized", "true");
}

pub fn init_guard(el: &Element) -> bool {
    if is_initialized(el) { return false; }
    mark_initialized(el);
    true
}
