#![deny(warnings)]
//! canonrs-interactions-init
//! Tier S: registry-driven, per-element dispatch

pub mod runtime;
pub mod animate;
pub mod filter;
pub mod avatar;
pub mod table;
pub mod table_row_sheet_preview;
pub mod tooltip;
pub mod collapsible;
pub mod switch;
pub mod toggle;
pub mod markdown;
pub mod checkbox;
pub mod radio;
pub mod progress;
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
pub mod command;
pub mod field;
pub mod form;
pub mod nav_item;
pub mod loading_overlay;
pub mod input;

pub fn scan_all() {
    runtime::registry::scan_all();
}


use canonrs_interactions_core::runtime::bootstrap;

use wasm_bindgen::prelude::*;

/// WASM entry point — initialize all init components in document
#[wasm_bindgen]
pub fn init_init_all() {
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        let Ok(nodes) = doc.query_selector_all("[data-rs-interaction=\"init\"]") else { return };
        for i in 0..nodes.length() {
            let Some(raw) = nodes.item(i) else { continue };
            if let Ok(el) = wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(raw) {
                if !el.has_attribute("data-rs-initialized") {
                    init_init(el);
                }
            }
        }
    }
}

/// WASM entry point — initialize init subtree
#[wasm_bindgen]
pub fn init_init_subtree(root: web_sys::Element) {
    let Ok(nodes) = root.query_selector_all("[data-rs-interaction=\"init\"]") else { return };
    for i in 0..nodes.length() {
        let Some(raw) = nodes.item(i) else { continue };
        if let Ok(el) = wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(raw) {
            if !el.has_attribute("data-rs-initialized") {
                init_init(el);
            }
        }
    }
    // also check root itself
    if root.get_attribute("data-rs-interaction").as_deref() == Some("init") {
        if !root.has_attribute("data-rs-initialized") {
            init_init(root);
        }
    }
}


/// Registra o grupo init no bootstrap kernel.
pub fn register() {
    bootstrap::register("init", init_init);
}

/// Init subtree — replay-safe, delega para bootstrap kernel.
pub fn init_subtree(root: &web_sys::Element) {
    bootstrap::init_subtree(root);
}
pub fn init_init(el: web_sys::Element) {
    runtime::registry::dispatch(&el);
}