#![deny(warnings)]
//! canonrs-interactions-data

pub mod runtime;
pub mod shared;
pub mod engines;
pub mod data_table;
pub mod virtual_list;
pub mod list_item;
pub mod chart;

pub fn init_data(el: web_sys::Element) {
    if el.has_attribute("data-rs-datatable") { data_table::init(el.clone()); }
    if el.has_attribute("data-rs-virtual-list") { virtual_list::init(el.clone()); }
    if el.has_attribute("data-rs-list") { list_item::init(el.clone()); }
    if el.has_attribute("data-rs-chart") { chart::init(el.clone()); }
}
