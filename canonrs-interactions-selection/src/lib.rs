#![deny(warnings)]
//! canonrs-interactions-selection

pub mod runtime;
pub mod select;
pub mod combobox;
pub mod color_picker;
pub mod radio_group;
pub mod toggle_group;
pub mod tree;


use canonrs_interactions_core::runtime::bootstrap;

use wasm_bindgen::prelude::*;

/// WASM entry point — initialize all selection components in document
#[wasm_bindgen]
pub fn init_selection_all() {
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        let Ok(nodes) = doc.query_selector_all("[data-rs-interaction=\"selection\"]") else { return };
        for i in 0..nodes.length() {
            let Some(raw) = nodes.item(i) else { continue };
            if let Ok(el) = wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(raw) {
                if !el.has_attribute("data-rs-initialized") {
                    init_selection(el);
                }
            }
        }
    }
}

/// WASM entry point — initialize selection subtree
#[wasm_bindgen]
pub fn init_selection_subtree(root: web_sys::Element) {
    let Ok(nodes) = root.query_selector_all("[data-rs-interaction=\"selection\"]") else { return };
    for i in 0..nodes.length() {
        let Some(raw) = nodes.item(i) else { continue };
        if let Ok(el) = wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(raw) {
            if !el.has_attribute("data-rs-initialized") {
                init_selection(el);
            }
        }
    }
    // also check root itself
    if root.get_attribute("data-rs-interaction").as_deref() == Some("selection") {
        if !root.has_attribute("data-rs-initialized") {
            init_selection(root);
        }
    }
}


/// Registra o grupo selection no bootstrap kernel.
pub fn register() {
    bootstrap::register("selection", init_selection);
}

/// Init subtree — replay-safe, delega para bootstrap kernel.
pub fn init_subtree(root: &web_sys::Element) {
    bootstrap::init_subtree(root);
}
pub fn init_selection(el: web_sys::Element) {
    if el.has_attribute("data-rs-select") { select::init(el.clone()); }
    if el.has_attribute("data-rs-combobox") { combobox::init(el.clone()); }
    if el.has_attribute("data-rs-color-picker") { color_picker::init(el.clone()); }
    if el.has_attribute("data-rs-radio-group") { radio_group::init(el.clone()); }
    if el.has_attribute("data-rs-toggle-group") { toggle_group::init(el.clone()); }
    if el.has_attribute("data-rs-tree") { tree::init(el.clone()); }
}