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
pub mod alert;
pub mod banner;
pub mod button;
pub mod doc_progress;
pub mod icon_button;
pub mod input_group;
pub mod input_otp;
pub mod menu;
pub mod navigation_menu;
pub mod toast;
pub mod table_of_contents;

#[wasm_bindgen]
pub fn init_all() {
    runtime::registry::scan_all();
}

#[wasm_bindgen]
pub fn init_init(el: web_sys::Element) {
    runtime::registry::dispatch(&el);
}
