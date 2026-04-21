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

pub fn scan_all() {
    runtime::registry::scan_all();
}

pub fn init_init(el: web_sys::Element) {
    runtime::registry::dispatch(&el);
}
