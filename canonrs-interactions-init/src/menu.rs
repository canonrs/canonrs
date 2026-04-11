//! Menu Init — item selection via [data-rs-menu]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state, query, aria};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
        let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        let Some(item) = target.closest("[data-rs-menu-item]").ok().flatten() else { return };
        if item.get_attribute("data-rs-state").map(|s| s.contains("disabled")).unwrap_or(false) { return; }
        let Some(menu) = target.closest("[data-rs-menu]").ok().flatten() else { return };
        for el in query::all(&menu, "[data-rs-menu-item]") {
            state::remove_state(&el, "selected");
            state::add_state(&el, "unselected");
            aria::set_selected(&el, false);
        }
        state::remove_state(&item, "unselected");
        state::add_state(&item, "selected");
        aria::set_selected(&item, true);
    });

    let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
    cb.forget();
}
