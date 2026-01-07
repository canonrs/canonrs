#![recursion_limit = "1024"]

pub mod app;
pub mod pages;
pub mod providers;
pub mod utils;

use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(app::App);
}
