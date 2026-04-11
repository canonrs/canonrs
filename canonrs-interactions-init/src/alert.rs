//! Alert Init — dismiss via [data-rs-alert-close]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state, query};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    let Some(close_btn) = query::first(&root, "[data-rs-alert-close]") else { return };

    let root_cb = root.clone();
    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
        state::remove_state(&root_cb, "open");
        state::add_state(&root_cb, "closed");
    });
    let _ = close_btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
    cb.forget();
}
