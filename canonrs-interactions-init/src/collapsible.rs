//! Collapsible Init — DOM micro-interactions para [data-rs-collapsible]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state, query, aria};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
        let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        if target.closest("[data-rs-collapsible-trigger]").ok().flatten().is_none() { return; }
        let Some(root) = target.closest("[data-rs-collapsible]").ok().flatten() else { return };

        let is_open = state::is_open(&root);

        if is_open {
            state::close(&root);
            if let Some(trigger) = query::first(&root, "[data-rs-collapsible-trigger]") {
                aria::set_expanded(&trigger, false);
            }
            if let Some(content) = query::first(&root, "[data-rs-collapsible-content]") {
                state::close(&content);
                aria::set_hidden(&content, true);
            }
        } else {
            state::open(&root);
            if let Some(trigger) = query::first(&root, "[data-rs-collapsible-trigger]") {
                aria::set_expanded(&trigger, true);
            }
            if let Some(content) = query::first(&root, "[data-rs-collapsible-content]") {
                state::open(&content);
                aria::set_hidden(&content, false);
            }
        }
    });

    let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
    cb.forget();
}
