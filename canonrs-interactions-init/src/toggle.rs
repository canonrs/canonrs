//! Toggle Init — DOM micro-interactions para [data-rs-toggle]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::{state};
use canonrs_interactions_core::integration::aria;
use crate::runtime::{interactive};

pub fn init(root: Element) {
    if root.has_attribute("data-rs-disabled") { return; }

    // hover / focus / active via runtime
    interactive::init(&root);

    // change no input — toggle on/off
    let root_cb = root.clone();
    let cb = Closure::<dyn Fn(web_sys::Event)>::new(move |e: web_sys::Event| {
        let is_checked = e.target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
            .map(|i| i.checked())
            .unwrap_or(false);

        if is_checked {
            state::remove_state(&root_cb, "off");
            state::add_state(&root_cb, "on");
            aria::set_pressed(&root_cb, true);
        } else {
            state::remove_state(&root_cb, "on");
            state::add_state(&root_cb, "off");
            aria::set_pressed(&root_cb, false);
        }
    });

    let _ = root.add_event_listener_with_callback("change", cb.as_ref().unchecked_ref());
    cb.forget();
}
