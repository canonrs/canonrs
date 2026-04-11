//! State — helpers para mutar data-rs-state no DOM
use web_sys::Element;

pub fn add_state(el: &Element, state: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    if !current.split_whitespace().any(|s| s == state) {
        let next = if current.is_empty() { state.to_string() } else { format!("{} {}", current, state) };
        let _ = el.set_attribute("data-rs-state", &next);
    }
}

pub fn remove_state(el: &Element, state: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    let next: Vec<&str> = current.split_whitespace().filter(|s| *s != state).collect();
    let _ = el.set_attribute("data-rs-state", &next.join(" "));
}

pub fn open(el: &Element) {
    remove_state(el, "closed");
    add_state(el, "open");
}

pub fn close(el: &Element) {
    remove_state(el, "open");
    add_state(el, "closed");
}

pub fn is_open(el: &Element) -> bool {
    el.get_attribute("data-rs-state")
        .map(|s| s.split_whitespace().any(|t| t == "open"))
        .unwrap_or(false)
}
