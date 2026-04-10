//! Attrs — DOM attribute helpers compartilhados

use web_sys::Element;

/// Lê atributo f64 com fallback
pub fn get_f64(el: &Element, attr: &str, default: f64) -> f64 {
    el.get_attribute(attr).and_then(|s| s.parse().ok()).unwrap_or(default)
}

/// Lê atributo usize com fallback
pub fn get_usize(el: &Element, attr: &str, default: usize) -> usize {
    el.get_attribute(attr).and_then(|s| s.parse().ok()).unwrap_or(default)
}

/// Lê atributo &str como String com fallback
pub fn get_str(el: &Element, attr: &str, default: &str) -> String {
    el.get_attribute(attr).unwrap_or_else(|| default.to_string())
}
