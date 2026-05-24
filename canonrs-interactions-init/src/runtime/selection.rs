//! Selection — single/multi selection pattern para menu, tabs

use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::{state, query};
use canonrs_interactions_core::integration::aria;
use canonrs_interactions_core::runtime::listeners;

pub fn init_single(root: &Element, item_selector: &'static str) {
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    let root_ref = root.clone();
    listeners::listen(&uid, root, "click", move |e: web_sys::Event| {
        let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
        let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        let Some(item) = target.closest(item_selector).ok().flatten() else { return };
        if item.get_attribute("data-rs-state").map(|s| s.contains("disabled")).unwrap_or(false) { return; }
        for el in query::all(&root_ref, item_selector) {
            state::remove_state(&el, "selected");
            state::add_state(&el, "unselected");
            aria::set_selected(&el, false);
        }
        state::remove_state(&item, "unselected");
        state::add_state(&item, "selected");
        aria::set_selected(&item, true);
    });
}
