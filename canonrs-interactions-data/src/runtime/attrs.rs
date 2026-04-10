//! Attrs — DOM attribute helpers for data interactions
use web_sys::Element;
use wasm_bindgen::JsCast;

pub fn get_f64(el: &Element, attr: &str, default: f64) -> f64 {
    el.get_attribute(attr).and_then(|s| s.parse().ok()).unwrap_or(default)
}

pub fn get_usize(el: &Element, attr: &str, default: usize) -> usize {
    el.get_attribute(attr).and_then(|s| s.parse().ok()).unwrap_or(default)
}

pub fn get_str(el: &Element, attr: &str, default: &str) -> String {
    el.get_attribute(attr).unwrap_or_else(|| default.to_string())
}

pub fn get_bool(el: &Element, attr: &str) -> bool {
    el.has_attribute(attr)
}

pub fn is_state(el: &Element, token: &str) -> bool {
    el.get_attribute("data-rs-state")
        .unwrap_or_default()
        .split_whitespace()
        .any(|t| t == token)
}

pub fn query_one(root: &Element, selector: &str) -> Option<web_sys::HtmlElement> {
    root.query_selector(selector).ok().flatten()
        .and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok())
}

pub fn get_usize_html(el: &web_sys::HtmlElement, attr: &str, default: usize) -> usize {
    el.get_attribute(attr).and_then(|v| v.parse().ok()).unwrap_or(default)
}
