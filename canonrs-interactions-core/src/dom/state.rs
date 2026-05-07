//! State — helpers para mutar data-rs-state no DOM
use wasm_bindgen::JsValue;
use web_sys::Element;

pub fn is_valid(el: &Element) -> bool {
    let v: &JsValue = el.as_ref();
    !v.is_null() && !v.is_undefined() && el.is_connected()
}

pub fn add(el: &Element, token: &str) {
    if !is_valid(el) { return; }
    let cur = el.get_attribute("data-rs-state").unwrap_or_default();
    if cur.split_whitespace().any(|t| t == token) { return; }
    let next = if cur.is_empty() { token.to_string() } else { format!("{} {}", cur, token) };
    let _ = el.set_attribute("data-rs-state", &next);
}

pub fn remove(el: &Element, token: &str) {
    if !is_valid(el) { return; }
    let cur = el.get_attribute("data-rs-state").unwrap_or_default();
    if !cur.split_whitespace().any(|t| t == token) { return; }
    let next = cur.split_whitespace().filter(|t| *t != token).collect::<Vec<_>>().join(" ");
    let _ = el.set_attribute("data-rs-state", &next);
}

pub fn has(el: &Element, token: &str) -> bool {
    if !is_valid(el) { return false; }
    el.get_attribute("data-rs-state")
        .unwrap_or_default()
        .split_whitespace()
        .any(|t| t == token)
}

pub fn open(el: &Element) {
    remove(el, "closed");
    add(el, "open");
}

pub fn close(el: &Element) {
    remove(el, "open");
    add(el, "closed");
}

pub fn toggle(el: &Element) {
    if is_open(el) { close(el); } else { open(el); }
}

pub fn is_open(el: &Element) -> bool {
    has(el, "open")
}

pub fn expand(el: &Element) {
    remove(el, "collapsed");
    add(el, "expanded");
}

pub fn collapse(el: &Element) {
    remove(el, "expanded");
    add(el, "collapsed");
}

pub fn is_expanded(el: &Element) -> bool {
    has(el, "expanded")
}

pub fn set_scroll_lock(locked: bool) {
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        if let Some(body) = doc.body() {
            if locked { let _ = body.set_attribute("data-rs-scroll-lock", "true"); }
            else      { let _ = body.remove_attribute("data-rs-scroll-lock"); }
        }
    }
}

// Aliases para compatibilidade
pub fn add_state(el: &Element, token: &str) { add(el, token); }
pub fn remove_state(el: &Element, token: &str) { remove(el, token); }
