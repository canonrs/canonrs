//! Detecta quais grupos de interação estão presentes no DOM.

use wasm_bindgen::JsCast;
use web_sys::{window, Element};
use std::collections::BTreeSet;

pub fn detect_groups() -> Vec<String> {
    let doc = match window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return vec![],
    };

    let nodes = match doc.query_selector_all("[data-rs-interaction]") {
        Ok(n) => n,
        Err(_) => return vec![],
    };

    let mut set = BTreeSet::new();

    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() {
                if let Some(group) = el.get_attribute("data-rs-interaction") {
                    set.insert(group);
                }
            }
        }
    }

    set.into_iter().collect()
}
