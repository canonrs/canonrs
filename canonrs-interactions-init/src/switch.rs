//! Switch Init — DOM micro-interactions para [data-rs-switch]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state, aria, focus};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    let Some(input) = crate::runtime::query::first(&root, "[data-rs-switch-input]") else { return };
    let root_cb = root.clone();

    let cb = Closure::<dyn Fn(web_sys::Event)>::new(move |e: web_sys::Event| {
        let is_checked = e.target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
            .map(|i| i.checked())
            .unwrap_or(false);

        if is_checked {
            state::remove_state(&root_cb, "unselected");
            state::add_state(&root_cb, "selected");
            aria::set_checked(&root_cb, true);
        } else {
            state::remove_state(&root_cb, "selected");
            state::add_state(&root_cb, "unselected");
            aria::set_checked(&root_cb, false);
        }
    });

    let _ = input.add_event_listener_with_callback("change", cb.as_ref().unchecked_ref());
    cb.forget();

    focus::init_focus(&root);
}
