#![deny(warnings)]
//! canonrs-interactions-data

pub mod runtime;
pub mod engines;
pub mod data_table;
pub mod virtual_list;
pub mod list_item;
pub mod chart;


use canonrs_interactions_core::runtime::bootstrap;

/// Registra o grupo data no bootstrap kernel.
pub fn register() {
    bootstrap::register("data", init_data);
}

/// Init subtree — replay-safe, delega para bootstrap kernel.
pub fn init_subtree(root: &web_sys::Element) {
    bootstrap::init_subtree(root);
}
pub fn init_data(el: web_sys::Element) {
    if el.has_attribute("data-rs-datatable")    { data_table::init(el.clone()); }
    if el.has_attribute("data-rs-virtual-list") { virtual_list::init(el.clone()); }
    if el.has_attribute("data-rs-list")         { list_item::init(el.clone()); }
    if el.has_attribute("data-rs-chart")        { chart::init(el.clone()); }
}
