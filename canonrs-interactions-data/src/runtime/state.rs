//! State — add/remove/has (crash-proof)
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
    el.get_attribute("data-rs-state").unwrap_or_default().split_whitespace().any(|t| t == token)
}
