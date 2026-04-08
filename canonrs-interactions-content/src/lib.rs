//! canonrs-interactions-content

pub mod shared;
pub mod markdown;
pub mod copy_button;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn init_content(el: web_sys::Element) {
    if el.has_attribute("data-rs-markdown") { markdown::init(el.clone()); }
    if el.has_attribute("data-rs-copy-button") { copy_button::init(el.clone()); }
}
