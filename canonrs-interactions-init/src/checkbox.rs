//! Checkbox Init — DOM micro-interactions para [data-rs-checkbox]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state, query};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    let Some(input) = query::first(&root, "[data-rs-checkbox-input]") else { return };

    // change no input
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::Event)>::new(move |e: web_sys::Event| {
            let is_checked = e.target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                .map(|i| i.checked())
                .unwrap_or(false);
            if is_checked {
                state::remove_state(&root_cb, "inactive");
                state::add_state(&root_cb, "active");
            } else {
                state::remove_state(&root_cb, "active");
                state::add_state(&root_cb, "inactive");
            }
        });
        let _ = input.add_event_listener_with_callback("change", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // focus
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::new(move |_: web_sys::FocusEvent| {
            state::add_state(&root_cb, "focus");
        });
        let _ = root.add_event_listener_with_callback("focusin", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // blur
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::new(move |_: web_sys::FocusEvent| {
            state::remove_state(&root_cb, "focus");
        });
        let _ = root.add_event_listener_with_callback("focusout", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}
