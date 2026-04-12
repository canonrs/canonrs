//! IconButton Init — hover/focus/active/pressed states

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, interactive, state, aria};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    interactive::init(&root);

    // toggle pressed on click
    let r = root.clone();
    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
        if r.get_attribute("data-rs-state").map(|s| s.contains("disabled")).unwrap_or(false) { return; }
        if state::is_open(&r) {
            state::remove_state(&r, "pressed");
            aria::set_pressed(&r, false);
        } else {
            state::add_state(&r, "pressed");
            aria::set_pressed(&r, true);
        }
    });
    let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
    cb.forget();
}
