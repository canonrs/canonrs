#![deny(warnings)]
//! canonrs-interactions-data

pub mod runtime;
pub mod engines;
pub mod data_table;
pub mod virtual_list;
pub mod list_item;
pub mod chart;

use canonrs_interactions_core::runtime::bootstrap;

use wasm_bindgen::prelude::*;

/// WASM entry point — initialize all data components in document
#[wasm_bindgen]
pub fn init_data_all() {
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        let Ok(nodes) = doc.query_selector_all("[data-rs-interaction=\"data\"]") else { return };
        for i in 0..nodes.length() {
            let Some(raw) = nodes.item(i) else { continue };
            if let Ok(el) = wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(raw) {
                if !el.has_attribute("data-rs-initialized") {
                    init_data(el);
                }
            }
        }
    }
}

/// WASM entry point — initialize data subtree
#[wasm_bindgen]
pub fn init_data_subtree(root: web_sys::Element) {
    let Ok(nodes) = root.query_selector_all("[data-rs-interaction=\"data\"]") else { return };
    for i in 0..nodes.length() {
        let Some(raw) = nodes.item(i) else { continue };
        if let Ok(el) = wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(raw) {
            if !el.has_attribute("data-rs-initialized") {
                init_data(el);
            }
        }
    }
    // also check root itself
    if root.get_attribute("data-rs-interaction").as_deref() == Some("data") {
        if !root.has_attribute("data-rs-initialized") {
            init_data(root);
        }
    }
}


/// Registra o grupo data no bootstrap kernel.
pub fn register() {
    bootstrap::register("data", init_data);
}

/// Init subtree — replay-safe, delega para bootstrap kernel.
pub fn init_subtree(root: &web_sys::Element) {
    use wasm_bindgen::JsCast;
    let js: &wasm_bindgen::JsValue = root.as_ref();
    if let Some(el) = js.dyn_ref() {
        bootstrap::init_subtree(el);
    }
}
pub fn init_data(el: web_sys::Element) {
    if el.has_attribute("data-rs-datatable")    { data_table::init(el.clone()); }
    if el.has_attribute("data-rs-virtual-list") { virtual_list::init(el.clone()); }
    if el.has_attribute("data-rs-list")         { list_item::init(el.clone()); }
    if el.has_attribute("data-rs-chart")        { chart::init(el.clone()); }
}