//! Breadcrumb Interaction Engine — active state sync via data-rs-current (DOM-driven)

use wasm_bindgen::JsCast;
use crate::runtime::{lifecycle, state};
use web_sys::Element;



pub fn init(root: Element) {
    if lifecycle::is_initialized(&root) { return; }
    lifecycle::mark_initialized(&root);
    let Ok(links) = root.query_selector_all("[data-rs-breadcrumb-link]") else { return };
    for i in 0..links.length() {
        if let Some(node) = links.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() {
                let is_active = el.get_attribute("data-rs-current").as_deref() == Some("true");
                state::remove_state(&el, "active");
                state::remove_state(&el, "inactive");
                if is_active {
                    state::add_state(&el, "active");
                } else {
                    state::add_state(&el, "inactive");
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
