//! Collapsible Init

use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::{state, query};
use canonrs_interactions_core::integration::aria;
use canonrs_interactions_core::runtime::listeners;

pub fn init(root: Element) {
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    listeners::listen(&uid, &root, "click", move |e: web_sys::Event| {
        let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
        let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        if target.closest("[data-rs-collapsible-trigger]").ok().flatten().is_none() { return; }
        let Some(root_el) = target.closest("[data-rs-collapsible]").ok().flatten() else { return };
        let is_open = state::is_open(&root_el);
        if is_open {
            state::close(&root_el);
            if let Some(t) = query::first(&root_el, "[data-rs-collapsible-trigger]") { aria::set_expanded(&t, false); }
            if let Some(c) = query::first(&root_el, "[data-rs-collapsible-content]") { state::close(&c); aria::set_hidden(&c, true); }
        } else {
            state::open(&root_el);
            if let Some(t) = query::first(&root_el, "[data-rs-collapsible-trigger]") { aria::set_expanded(&t, true); }
            if let Some(c) = query::first(&root_el, "[data-rs-collapsible-content]") { state::open(&c); aria::set_hidden(&c, false); }
        }
    });
}
