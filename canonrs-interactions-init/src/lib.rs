//! canonrs-interactions-init
//! Canon Init Loader — DOM micro-interactions (init type)
//! Mirror of canon-loader pattern for lightweight DOM behaviors

use wasm_bindgen::prelude::*;

pub mod runtime;
pub mod table;

#[wasm_bindgen]
pub fn init_all() {
    table::init_all();
}
