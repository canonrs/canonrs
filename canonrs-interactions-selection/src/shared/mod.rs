//! Shared helpers for all interactions
//! DOM is source of truth — no internal state

use web_sys::Element;

/// Add a state token to data-rs-state (idempotent)
pub fn add_state(el: &Element, state: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    if !current.split_whitespace().any(|s| s == state) {
        let next = if current.is_empty() { state.to_string() } else { format!("{} {}", current, state) };
        el.set_attribute("data-rs-state", &next).ok();
    }
}

/// Remove a state token from data-rs-state (idempotent)
pub fn remove_state(el: &Element, state: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    let next: Vec<&str> = current.split_whitespace().filter(|s| *s != state).collect();
    el.set_attribute("data-rs-state", &next.join(" ")).ok();
}

/// Check if element is already initialized (idempotent init guard)
pub fn is_initialized(el: &Element) -> bool {
    el.get_attribute("data-rs-initialized").as_deref() == Some("true")
}

/// Mark element as initialized
pub fn mark_initialized(el: &Element) {
    let _ = el.set_attribute("data-rs-initialized", "true");
}
