//! canonrs-interactions-init
//! Canon Init Loader — DOM micro-interactions (init type)
//! Tier S: registry-driven, selector-based, idempotent, zero state

use wasm_bindgen::prelude::*;

pub mod runtime;
pub mod table;
pub mod tooltip;
pub mod table_row_sheet_preview;

#[wasm_bindgen]
pub fn init_all() {
    runtime::registry::scan_all();
}
