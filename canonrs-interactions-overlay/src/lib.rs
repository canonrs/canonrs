#![deny(warnings)]
//! canonrs-interactions-overlay
//! Grupo overlay: dialog, modal, drawer, sheet, alert_dialog,
//! confirm_dialog, popover, hover_card, context_menu, dropdown_menu.
//!
//! Padrão oficial:
//!   register() — registra init_overlay no bootstrap kernel
//!   init_subtree(root) — delega para bootstrap::init_subtree
//!   init_overlay(el) — dispatcher interno por atributo

pub mod runtime;

pub mod modal;
pub mod drawer;
pub mod sheet;
pub mod alert_dialog;
pub mod dialog;
pub mod confirm_dialog;
pub mod popover;
pub mod hover_card;
pub mod context_menu;
pub mod dropdown_menu;

use canonrs_interactions_core::runtime::bootstrap;

use wasm_bindgen::prelude::*;

/// WASM entry point — initialize all overlay components in document
#[wasm_bindgen]
pub fn init_overlay_all() {
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        let Ok(nodes) = doc.query_selector_all("[data-rs-interaction=\"overlay\"]") else { return };
        for i in 0..nodes.length() {
            let Some(raw) = nodes.item(i) else { continue };
            if let Ok(el) = wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(raw) {
                if !el.has_attribute("data-rs-initialized") {
                    init_overlay(el);
                }
            }
        }
    }
}

/// WASM entry point — initialize overlay subtree
#[wasm_bindgen]
pub fn init_overlay_subtree(root: web_sys::Element) {
    let Ok(nodes) = root.query_selector_all("[data-rs-interaction=\"overlay\"]") else { return };
    for i in 0..nodes.length() {
        let Some(raw) = nodes.item(i) else { continue };
        if let Ok(el) = wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(raw) {
            if !el.has_attribute("data-rs-initialized") {
                init_overlay(el);
            }
        }
    }
    // also check root itself
    if root.get_attribute("data-rs-interaction").as_deref() == Some("overlay") {
        if !root.has_attribute("data-rs-initialized") {
            init_overlay(root);
        }
    }
}


/// Registra o grupo overlay no bootstrap kernel.
/// Deve ser chamado uma vez no bootstrap da aplicação.
pub fn register() {
    bootstrap::register("overlay", init_overlay);
}

/// Init subtree — replay-safe, delega para bootstrap kernel.
/// Substitui init_all() e scan global local.
pub fn init_subtree(root: &web_sys::Element) {
    bootstrap::init_subtree(root);
}

/// Dispatcher interno — chamado pelo bootstrap kernel para cada elemento overlay.
/// NÃO chamar diretamente fora deste crate.
pub fn init_overlay(el: web_sys::Element) {
    if el.has_attribute("data-rs-modal")          { modal::init(el.clone()); }
    if el.has_attribute("data-rs-drawer")         { drawer::init(el.clone()); }
    if el.has_attribute("data-rs-sheet")          { sheet::init(el.clone()); }
    if el.has_attribute("data-rs-alert-dialog")   { alert_dialog::init(el.clone()); }
    if el.has_attribute("data-rs-dialog")         { dialog::init(el.clone()); }
    if el.has_attribute("data-rs-confirm-dialog") { confirm_dialog::init(el.clone()); }
    if el.has_attribute("data-rs-popover")        { popover::init(el.clone()); }
    if el.has_attribute("data-rs-hover-card")     { hover_card::init(el.clone()); }
    if el.has_attribute("data-rs-context-menu")   { context_menu::init(el.clone()); }
    if el.has_attribute("data-rs-dropdown-menu")  { dropdown_menu::init(el.clone()); }
}