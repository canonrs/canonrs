//! canonrs-interactions-gesture
//! Grupo gesture: resizable, slider, carousel, scroll_area.

pub mod shared;
pub mod resizable;
pub mod slider;
pub mod carousel;
pub mod scroll_area;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn init_gesture(el: web_sys::Element) {
    if el.has_attribute("data-rs-resizable") { resizable::init(el.clone()); }
    if el.has_attribute("data-rs-slider") { slider::init(el.clone()); }
    if el.has_attribute("data-rs-carousel") { carousel::init(el.clone()); }
    if el.has_attribute("data-rs-scroll-area") { scroll_area::init(el.clone()); }
}
