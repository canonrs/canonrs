//! Attrs — DOM attribute helpers
use web_sys::Element;

pub fn get_str(el: &Element, attr: &str, default: &str) -> String {
    el.get_attribute(attr).unwrap_or_else(|| default.to_string())
}

pub fn get_bool(el: &Element, attr: &str) -> bool {
    el.has_attribute(attr)
}

pub fn get_usize(el: &Element, attr: &str, default: usize) -> usize {
    el.get_attribute(attr).and_then(|s| s.parse().ok()).unwrap_or(default)
}

pub fn get_f64(el: &Element, attr: &str, default: f64) -> f64 {
    el.get_attribute(attr).and_then(|s| s.parse().ok()).unwrap_or(default)
}

pub fn get_i32(el: &Element, attr: &str, default: i32) -> i32 {
    el.get_attribute(attr).and_then(|s| s.parse().ok()).unwrap_or(default)
}

pub fn query_one(root: &Element, selector: &str) -> Option<web_sys::HtmlElement> {
    root.query_selector(selector).ok().flatten()
        .and_then(|n| wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>(n).ok())
}
