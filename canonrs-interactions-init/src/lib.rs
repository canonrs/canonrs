//! canonrs-interactions-init
//! Tier S: registry-driven, per-element dispatch

use wasm_bindgen::prelude::*;

pub mod runtime;
pub mod table;
pub mod tooltip;
pub mod collapsible;
pub mod switch;
pub mod toggle;
pub mod checkbox;

#[wasm_bindgen]
pub fn init_all() {
    runtime::registry::scan_all();
}

#[wasm_bindgen]
pub fn init_init(el: web_sys::Element) {
    runtime::registry::dispatch(&el);
}
