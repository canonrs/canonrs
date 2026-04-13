#![deny(warnings)]
//! canonrs-interactions-selection

pub mod runtime;
pub mod select;
pub mod combobox;
pub mod color_picker;
pub mod radio;
pub mod toggle_group;
pub mod tree;

pub fn init_selection(el: web_sys::Element) {
    if el.has_attribute("data-rs-select") { select::init(el.clone()); }
    if el.has_attribute("data-rs-combobox") { combobox::init(el.clone()); }
    if el.has_attribute("data-rs-color-picker") { color_picker::init(el.clone()); }
    if el.has_attribute("data-rs-radio-group") { radio::init(el.clone()); }
    if el.has_attribute("data-rs-toggle-group") { toggle_group::init(el.clone()); }
    if el.has_attribute("data-rs-tree") { tree::init(el.clone()); }
}
