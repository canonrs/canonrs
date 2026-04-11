//! InputOtp Init — focus first input on click

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, query};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
        if let Some(input) = query::first(&root, "[data-rs-input-otp]") {
            if let Ok(el) = input.dyn_into::<web_sys::HtmlInputElement>() {
                let _ = el.focus();
            }
        }
    });

    let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
    cb.forget();
}
