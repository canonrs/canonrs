//! NavigationMenu Init — open/close items via [data-rs-navigation-menu]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state, query};

fn close_all(root: &Element) {
    for item in query::all(root, "[data-rs-navigation-menu-item]") {
        state::close(&item);
        if let Some(content) = query::first(&item, "[data-rs-navigation-menu-content]") {
            if let Ok(h) = content.dyn_into::<web_sys::HtmlElement>() {
                let _ = h.set_hidden(true);
            }
        }
    }
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    // click trigger
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            e.stop_propagation();
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(_trigger) = target.closest("[data-rs-navigation-menu-trigger]").ok().flatten() else { return };
            let Some(item) = target.closest("[data-rs-navigation-menu-item]").ok().flatten() else { return };
            let is_open = state::is_open(&item);
            close_all(&root_cb);
            if !is_open {
                state::open(&item);
                if let Some(content) = query::first(&item, "[data-rs-navigation-menu-content]") {
                    if let Ok(h) = content.dyn_into::<web_sys::HtmlElement>() {
                        let _ = h.set_hidden(false);
                    }
                }
            }
        });
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // click outside
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if root_cb.contains(Some(&target)) { return; }
            close_all(&root_cb);
        });
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let _ = doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        }
        cb.forget();
    }
}
