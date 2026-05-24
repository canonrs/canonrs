#![deny(warnings)]
//! canonrs-interactions-nav

pub mod sidebar;
pub mod menubar;
pub mod toolbar;
pub mod breadcrumb;
pub mod link_group;
pub mod pagination;
pub mod tabs;
pub mod accordion;


use canonrs_interactions_core::runtime::bootstrap;

use wasm_bindgen::prelude::*;

/// WASM entry point — initialize all nav components in document
#[wasm_bindgen]
pub fn init_nav_all() {
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        let Ok(nodes) = doc.query_selector_all("[data-rs-interaction=\"nav\"]") else { return };
        for i in 0..nodes.length() {
            let Some(raw) = nodes.item(i) else { continue };
            if let Ok(el) = wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(raw) {
                if !el.has_attribute("data-rs-initialized") {
                    init_nav(el);
                }
            }
        }
    }
}

/// WASM entry point — initialize nav subtree
#[wasm_bindgen]
pub fn init_nav_subtree(root: web_sys::Element) {
    let Ok(nodes) = root.query_selector_all("[data-rs-interaction=\"nav\"]") else { return };
    for i in 0..nodes.length() {
        let Some(raw) = nodes.item(i) else { continue };
        if let Ok(el) = wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(raw) {
            if !el.has_attribute("data-rs-initialized") {
                init_nav(el);
            }
        }
    }
    // also check root itself
    if root.get_attribute("data-rs-interaction").as_deref() == Some("nav") {
        if !root.has_attribute("data-rs-initialized") {
            init_nav(root);
        }
    }
}


/// Registra o grupo nav no bootstrap kernel.
pub fn register() {
    bootstrap::register("nav", init_nav);
}

/// Init subtree — replay-safe, delega para bootstrap kernel.
pub fn init_subtree(root: &web_sys::Element) {
    bootstrap::init_subtree(root);
}
pub fn init_nav(el: web_sys::Element) {
    if el.has_attribute("data-rs-sidebar")    { sidebar::init(el.clone()); }
    if el.has_attribute("data-rs-tabs")       { tabs::init(el.clone()); }
    if el.has_attribute("data-rs-accordion")  { accordion::init(el.clone()); }
    if el.has_attribute("data-rs-menubar")    { menubar::init(el.clone()); }
    if el.has_attribute("data-rs-toolbar")    { toolbar::init(el.clone()); }
    if el.has_attribute("data-rs-breadcrumb") { breadcrumb::init(el.clone()); }
    if el.has_attribute("data-rs-link-group") { link_group::init(el.clone()); }
    if el.has_attribute("data-rs-pagination") { pagination::init(el.clone()); }
}