//! State — open/close/toggle/is_open (crash-proof)
use wasm_bindgen::JsValue;
use web_sys::Element;
use crate::shared::{add_state, remove_state};

fn is_valid(el: &Element) -> bool {
    let v: &JsValue = el.as_ref();
    !v.is_null() && !v.is_undefined() && el.is_connected()
}

pub fn is_open(el: &Element) -> bool {
    if !is_valid(el) { return false; }
    el.get_attribute("data-rs-state")
        .map(|s| s.split_whitespace().any(|t| t == "open"))
        .unwrap_or(false)
}

pub fn open(el: &Element) {
    if !is_valid(el) { return; }
    remove_state(el, "closed");
    add_state(el, "open");
}

pub fn close(el: &Element) {
    if !is_valid(el) { return; }
    remove_state(el, "open");
    add_state(el, "closed");
}

pub fn toggle(el: &Element) {
    if is_open(el) { close(el); } else { open(el); }
}

pub fn set_scroll_lock(locked: bool) {
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        if let Some(body) = doc.body() {
            if locked { let _ = body.set_attribute("data-rs-scroll-lock", "true"); }
            else      { let _ = body.remove_attribute("data-rs-scroll-lock"); }
        }
    }
}
