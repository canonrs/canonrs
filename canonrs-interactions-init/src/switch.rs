//! Switch Init — DOM micro-interactions para [data-rs-switch]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state, aria};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
        let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        let Some(switch) = target.closest("[data-rs-switch]").ok().flatten() else { return };

        let is_selected = switch.get_attribute("data-rs-state")
            .map(|s| s.contains("selected"))
            .unwrap_or(false);

        if is_selected {
            state::remove_state(&switch, "selected");
            aria::set_checked(&switch, false);
        } else {
            state::add_state(&switch, "selected");
            aria::set_checked(&switch, true);
        }
    });

    let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
    cb.forget();
}
