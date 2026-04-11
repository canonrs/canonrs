//! canonrs-interactions-data

pub mod runtime;
pub mod shared;
pub mod engines;
pub mod data_table;
pub mod table;
pub mod virtual_list;
pub mod list_item;
pub mod chart;
pub mod table_row_sheet_preview;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn init_data(el: web_sys::Element) {
    if el.has_attribute("data-rs-datatable") { data_table::init(el.clone()); }
    if el.has_attribute("data-rs-table") { table::init(el.clone()); }
    if el.has_attribute("data-rs-virtual-list") { virtual_list::init(el.clone()); }
    if el.has_attribute("data-rs-list-item") { list_item::init(el.clone()); }
    if el.has_attribute("data-rs-chart") { chart::init(el.clone()); }
    if el.has_attribute("data-rs-table-context") { table_row_sheet_preview::init(el.clone()); }
}
