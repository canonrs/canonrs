//! Toast Init — auto-dismiss + close button

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state, query};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    let duration_ms = root.get_attribute("data-rs-duration")
        .and_then(|d| d.parse::<i32>().ok())
        .unwrap_or(5000);

    // close button
    if let Some(btn) = query::first(&root, "[data-rs-toast-close]") {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
            state::remove_state(&root_cb, "open");
            state::add_state(&root_cb, "closed");
        });
        let _ = btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // auto-dismiss
    {
        let root_cb = root.clone();
        let cb = Closure::once(move || {
            state::remove_state(&root_cb, "open");
            state::add_state(&root_cb, "closed");
        });
        if let Some(win) = web_sys::window() {
            let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(), duration_ms
            );
        }
        cb.forget();
    }
}
