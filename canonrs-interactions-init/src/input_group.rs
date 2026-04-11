//! InputGroup Init — focus-within state

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    {
        let r = root.clone();
        let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::new(move |_: web_sys::FocusEvent| {
            state::add_state(&r, "focus-within");
        });
        let _ = root.add_event_listener_with_callback("focusin", cb.as_ref().unchecked_ref());
        cb.forget();
    }
    {
        let r = root.clone();
        let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::new(move |_: web_sys::FocusEvent| {
            state::remove_state(&r, "focus-within");
        });
        let _ = root.add_event_listener_with_callback("focusout", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}
