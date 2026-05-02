#![deny(warnings)]
pub mod runtime;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init_all() {
    runtime::scan_and_init();
}

#[wasm_bindgen]
pub fn gc() {
    runtime::registry::gc();
}

#[wasm_bindgen]
pub fn init_subtree(el: web_sys::Element) {
    runtime::init_element(&el);
}

/// Plugin registration — externos registram handlers sem depender do core
/// rs-canvas-runtime chama: canonrs_interactions::register_interaction("canvas", fn)
pub fn register_interaction(group: &str, handler: fn(web_sys::Element)) {
    runtime::dispatcher::register_external(group.to_string(), handler);
}
