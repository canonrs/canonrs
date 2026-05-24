#![deny(warnings)]
//! canonrs-interactions-gesture
//! Grupo gesture: resizable, slider, carousel, scroll_area.

pub mod runtime;
pub mod resizable;
pub mod slider;
pub mod carousel;
pub mod scroll_area;


use canonrs_interactions_core::runtime::bootstrap;

use wasm_bindgen::prelude::*;

/// WASM entry point — initialize all gesture components in document
#[wasm_bindgen]
pub fn init_gesture_all() {
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        let Ok(nodes) = doc.query_selector_all("[data-rs-interaction=\"gesture\"]") else { return };
        for i in 0..nodes.length() {
            let Some(raw) = nodes.item(i) else { continue };
            if let Ok(el) = wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(raw) {
                if !el.has_attribute("data-rs-initialized") {
                    init_gesture(el);
                }
            }
        }
    }
}

/// WASM entry point — initialize gesture subtree
#[wasm_bindgen]
pub fn init_gesture_subtree(root: web_sys::Element) {
    let Ok(nodes) = root.query_selector_all("[data-rs-interaction=\"gesture\"]") else { return };
    for i in 0..nodes.length() {
        let Some(raw) = nodes.item(i) else { continue };
        if let Ok(el) = wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(raw) {
            if !el.has_attribute("data-rs-initialized") {
                init_gesture(el);
            }
        }
    }
    // also check root itself
    if root.get_attribute("data-rs-interaction").as_deref() == Some("gesture") {
        if !root.has_attribute("data-rs-initialized") {
            init_gesture(root);
        }
    }
}


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