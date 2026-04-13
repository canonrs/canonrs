#![deny(warnings)]
pub mod runtime;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init_all() {
    runtime::scan_and_init();
}
