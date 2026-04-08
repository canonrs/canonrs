//! Breadcrumb Interaction Engine — active state sync via data-rs-current (DOM-driven)

use wasm_bindgen::JsCast;
use web_sys::Element;

fn add_state(el: &Element, state: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    if !current.split_whitespace().any(|s| s == state) {
        let next = if current.is_empty() { state.to_string() } else { format!("{} {}", current, state) };
        el.set_attribute("data-rs-state", &next).ok();
    }
}

fn remove_state(el: &Element, state: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    let next: Vec<&str> = current.split_whitespace().filter(|s| *s != state).collect();
    el.set_attribute("data-rs-state", &next.join(" ")).ok();
}

pub fn init(root: Element) {
    let Ok(links) = root.query_selector_all("[data-rs-breadcrumb-link]") else { return };
    for i in 0..links.length() {
        if let Some(node) = links.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() {
                let is_active = el.get_attribute("data-rs-current").as_deref() == Some("true");
                remove_state(&el, "active");
                remove_state(&el, "inactive");
                if is_active {
                    add_state(&el, "active");
                } else {
                    add_state(&el, "inactive");
                }
            }
        }
    }
}

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-breadcrumb-link]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init(el); }
        }
    }
}
