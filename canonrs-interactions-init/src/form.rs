//! Form Init — submit state + field linking

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state, query, focus};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    focus::init_within(&root);

    // submit — adicionar state submitting
    let root_submit = root.clone();
    let cb = Closure::<dyn Fn(web_sys::Event)>::new(move |_: web_sys::Event| {
        state::remove_state(&root_submit, "idle");
        state::add_state(&root_submit, "submitting");
    });
    let _ = root.add_event_listener_with_callback("submit", cb.as_ref().unchecked_ref());
    cb.forget();

    // linking label → input via data-rs-uid
    for field in query::all(&root, "[data-rs-form-field]") {
        if let (Some(input), Some(label)) = (
            query::first(&field, "input, textarea, select"),
            query::first(&field, "[data-rs-form-label]")
        ) {
            let uid = input.get_attribute("data-rs-uid").unwrap_or_default();
            if !uid.is_empty() {
                let _ = input.set_attribute("id", &uid);
                let _ = label.set_attribute("for", &uid);
            }
            // propagar required do label para o input
            if label.get_attribute("data-rs-required").as_deref() == Some("true") {
                let _ = input.set_attribute("required", "");
                let _ = input.set_attribute("aria-required", "true");
            }
        }
    }
}
