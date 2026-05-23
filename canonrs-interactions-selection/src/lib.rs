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
