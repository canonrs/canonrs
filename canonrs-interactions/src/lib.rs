#![deny(warnings)]
use wasm_bindgen::prelude::*;
use web_sys::Element;

struct CanonDispatcher;

impl canonrs_runtime::Dispatcher for CanonDispatcher {
    fn dispatch(&self, el: &Element) {
        runtime::dispatcher::dispatch(el);
    }
}

pub mod runtime;

#[wasm_bindgen]
pub fn init_all() {
    canonrs_runtime::register_dispatcher(CanonDispatcher);
    canonrs_runtime::scan_and_init();
}

#[wasm_bindgen]
pub fn gc() {
    canonrs_runtime::registry::gc();
}

#[wasm_bindgen]
pub fn init_subtree(el: web_sys::Element) {
    canonrs_runtime::init_element(&el);
}
