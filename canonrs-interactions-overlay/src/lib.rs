//! canonrs-interactions-overlay
pub mod runtime;

pub mod shared;
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

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn init_overlay(el: web_sys::Element) {
    if el.has_attribute("data-rs-modal") { modal::init(el.clone()); }
    if el.has_attribute("data-rs-drawer") { drawer::init(el.clone()); }
    if el.has_attribute("data-rs-sheet") { sheet::init(el.clone()); }
    if el.has_attribute("data-rs-alert-dialog") { alert_dialog::init(el.clone()); }
    if el.has_attribute("data-rs-dialog") { dialog::init(el.clone()); }
    if el.has_attribute("data-rs-confirm-dialog") { confirm_dialog::init(el.clone()); }
    if el.has_attribute("data-rs-popover") { popover::init(el.clone()); }
    if el.has_attribute("data-rs-hover-card") { hover_card::init(el.clone()); }
    if el.has_attribute("data-rs-context-menu") { context_menu::init(el.clone()); }
    if el.has_attribute("data-rs-dropdown-menu") { dropdown_menu::init(el.clone()); }
}
