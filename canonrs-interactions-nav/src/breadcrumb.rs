//! Breadcrumb Interaction — hover + active state sync

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::runtime::{lifecycle, state, query};
use web_sys::Element;

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    // sync active — primitive já seta data-rs-state via activity_attrs
    for el in query::all(&root, "[data-rs-breadcrumb-link]") {
        if state::has(&el, "active") {
            let _ = el.set_attribute("aria-current", "page");
        } else {
            let _ = el.remove_attribute("aria-current");
        }
    }

    // hover
    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(link) = target.closest("[data-rs-breadcrumb-link]").ok().flatten() else { return };
            if !state::has(&link, "active") { state::add_state(&link, "hover"); }
        }));
        let _ = root.add_event_listener_with_callback("mouseover", cb.as_ref().unchecked_ref());
        cb.forget();
    }
    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(link) = target.closest("[data-rs-breadcrumb-link]").ok().flatten() else { return };
            state::remove_state(&link, "hover");
        }));
        let _ = root.add_event_listener_with_callback("mouseout", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}
