//! IconButton Init — hover/focus/active/pressed states

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, interactive, state, aria};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    interactive::init(&root);

    // toggle pressed — apenas se for um toggle button (tem aria-pressed)
    let is_toggle = root.has_attribute("aria-pressed");
    if is_toggle {
        let r = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
            if r.get_attribute("data-rs-state").map(|s| s.contains("disabled")).unwrap_or(false) { return; }
            let is_on = r.get_attribute("data-rs-state")
                .map(|s| s.contains("on"))
                .unwrap_or(false);
            if is_on {
                state::remove_state(&r, "on");
                aria::set_pressed(&r, false);
            } else {
                state::add_state(&r, "on");
                aria::set_pressed(&r, true);
            }
        });
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}
