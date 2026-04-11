//! State — open/close/toggle/is_open (crash-proof + auto-positioning)
use wasm_bindgen::JsValue;
use web_sys::Element;
use crate::runtime::positioning;

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

pub fn open(root: &Element) {
    if !is_valid(root) { return; }
    // auto-position: busca o primeiro filho com data-rs-side
    let content_selector = [
        "[data-rs-popover-content]",
        "[data-rs-dropdown-menu-content]",
        "[data-rs-hover-card-content]",
        "[data-rs-context-menu-content]",
        "[data-rs-tooltip-content]",
    ];
    for sel in content_selector {
        if root.query_selector(sel).ok().flatten().is_some() {
            positioning::auto_side(root, sel);
            break;
        }
    }
    remove_state(root, "closed");
    add_state(root, "open");
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

/// Add a state token to data-rs-state (idempotent)
pub fn add_state(el: &web_sys::Element, state: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    if !current.split_whitespace().any(|s| s == state) {
        let next = if current.is_empty() { state.to_string() } else { format!("{} {}", current, state) };
        el.set_attribute("data-rs-state", &next).ok();
    }
}

/// Remove a state token from data-rs-state (idempotent)
pub fn remove_state(el: &web_sys::Element, state: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    let next: Vec<&str> = current.split_whitespace().filter(|s| *s != state).collect();
    el.set_attribute("data-rs-state", &next.join(" ")).ok();
}
