#![deny(warnings)]
//! canonrs-interactions-gesture
//! Grupo gesture: resizable, slider, carousel, scroll_area.

pub mod runtime;
pub mod resizable;
pub mod slider;
pub mod carousel;
pub mod scroll_area;


use canonrs_interactions_core::runtime::bootstrap;

/// Registra o grupo gesture no bootstrap kernel.
pub fn register() {
    bootstrap::register("gesture", init_gesture);
}

/// Init subtree — replay-safe, delega para bootstrap kernel.
pub fn init_subtree(root: &web_sys::Element) {
    bootstrap::init_subtree(root);
}
pub fn init_gesture(el: web_sys::Element) {
    if el.has_attribute("data-rs-resizable") { resizable::init(el.clone()); }
    if el.has_attribute("data-rs-slider") { slider::init(el.clone()); }
    if el.has_attribute("data-rs-carousel") { carousel::init(el.clone()); }
    if el.has_attribute("data-rs-scroll-area") { scroll_area::init(el.clone()); }
}
