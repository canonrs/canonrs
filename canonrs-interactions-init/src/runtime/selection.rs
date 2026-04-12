//! Selection — single/multi selection pattern para menu, tabs

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{state, query, aria};

/// Single selection — click item → deselect all → select clicked
/// item_selector: seletor dos items (ex: "[data-rs-menu-item]")
pub fn init_single(root: &Element, item_selector: &'static str) {
    let root_cb = root.clone();
    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
        let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        let Some(item) = target.closest(item_selector).ok().flatten() else { return };
        if item.get_attribute("data-rs-state").map(|s| s.contains("disabled")).unwrap_or(false) { return; }
        let Some(root) = target.closest("[data-rs-menu]").ok().flatten()
            .or_else(|| target.closest("[data-rs-tabs]").ok().flatten()) else { return };
        for el in query::all(&root, item_selector) {
            state::remove_state(&el, "selected");
            state::add_state(&el, "unselected");
            aria::set_selected(&el, false);
        }
        state::remove_state(&item, "unselected");
        state::add_state(&item, "selected");
        aria::set_selected(&item, true);
    });
    let _ = root_cb.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
    cb.forget();
}
