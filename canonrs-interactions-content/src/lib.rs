#![deny(warnings)]
//! canonrs-interactions-content

pub mod markdown;
pub mod copy_button;


use canonrs_interactions_core::runtime::bootstrap;

use wasm_bindgen::prelude::*;

/// WASM entry point — initialize all content components in document
#[wasm_bindgen]
pub fn init_content_all() {
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        let Ok(nodes) = doc.query_selector_all("[data-rs-interaction=\"content\"]") else { return };
        for i in 0..nodes.length() {
            let Some(raw) = nodes.item(i) else { continue };
            if let Ok(el) = wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(raw) {
                if !el.has_attribute("data-rs-initialized") {
                    init_content(el);
                }
            }
        }
    }
}

/// WASM entry point — initialize content subtree
#[wasm_bindgen]
pub fn init_content_subtree(root: web_sys::Element) {
    let Ok(nodes) = root.query_selector_all("[data-rs-interaction=\"content\"]") else { return };
    for i in 0..nodes.length() {
        let Some(raw) = nodes.item(i) else { continue };
        if let Ok(el) = wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(raw) {
            if !el.has_attribute("data-rs-initialized") {
                init_content(el);
            }
        }
    }
    // also check root itself
    if root.get_attribute("data-rs-interaction").as_deref() == Some("content") {
        if !root.has_attribute("data-rs-initialized") {
            init_content(root);
        }
    }
}


/// Registra o grupo content no bootstrap kernel.
pub fn register() {
    bootstrap::register("content", init_content);
}

/// Init subtree — replay-safe, delega para bootstrap kernel.
pub fn init_subtree(root: &web_sys::Element) {
    bootstrap::init_subtree(root);
}
pub fn init_content(el: web_sys::Element) {
    if el.has_attribute("data-rs-markdown") { markdown::init(el.clone()); }
    if el.has_attribute("data-rs-copy-button") { copy_button::init(el.clone()); }
}