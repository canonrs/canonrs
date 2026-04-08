//! HoverCard Interaction Engine

use wasm_bindgen::prelude::*;
use crate::shared::{remove_state, is_initialized, mark_initialized};
use wasm_bindgen::JsCast;
use web_sys::Element;

fn add_state(el: &Element, state: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    if !current.split_whitespace().any(|s| s == state) {
        let next = if current.is_empty() { state.to_string() } else { format!("{} {}", current, state) };
        el.set_attribute("data-rs-state", &next).ok();
    }
}


pub fn init(root: Element) {
    if is_initialized(&root) { return; }
    mark_initialized(&root);
    // pointerenter trigger → open
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::PointerEvent)>::wrap(Box::new(move |e: web_sys::PointerEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-hover-card-trigger]").ok().flatten().is_none() { return; }
            remove_state(&root_cb, "closed");
            add_state(&root_cb, "open");
        }));
        let _ = root.add_event_listener_with_callback("pointerenter", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // pointerleave → close
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::PointerEvent)>::wrap(Box::new(move |_| {
            remove_state(&root_cb, "open");
            add_state(&root_cb, "closed");
        }));
        let _ = root.add_event_listener_with_callback("pointerleave", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // focusin trigger → open
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::wrap(Box::new(move |e: web_sys::FocusEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-hover-card-trigger]").ok().flatten().is_none() { return; }
            remove_state(&root_cb, "closed");
            add_state(&root_cb, "open");
        }));
        let _ = root.add_event_listener_with_callback("focusin", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // focusout → close
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::wrap(Box::new(move |_| {
            remove_state(&root_cb, "open");
            add_state(&root_cb, "closed");
        }));
        let _ = root.add_event_listener_with_callback("focusout", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-hover-card]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init(el); }
        }
    }
}
