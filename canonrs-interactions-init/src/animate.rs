//! Animate Init — ativa animacao via IntersectionObserver

use web_sys::Element;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::runtime::{lifecycle, state};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    let animation = root.get_attribute("data-rs-animation").unwrap_or_default();
    if animation == "none" || animation.is_empty() { return; }

    let root_cb = root.clone();
    let cb = Closure::<dyn Fn(js_sys::Array)>::new(move |entries: js_sys::Array| {
        for entry in entries.iter() {
            let entry = entry.unchecked_into::<web_sys::IntersectionObserverEntry>();
            if entry.is_intersecting() {
                state::add_state(&root_cb, "active");
            } else {
                state::remove_state(&root_cb, "active");
                state::add_state(&root_cb, "inactive");
            }
        }
    });

    let mut opts = web_sys::IntersectionObserverInit::new();
    opts.threshold(&JsValue::from_f64(0.1));

    if let Ok(observer) = web_sys::IntersectionObserver::new_with_options(
        cb.as_ref().unchecked_ref(),
        &opts,
    ) {
        observer.observe(&root);
    }
    cb.forget();
}
