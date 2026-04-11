//! Sidebar Interaction Engine

use wasm_bindgen::prelude::*;
use crate::runtime::{lifecycle, state};
use wasm_bindgen::JsCast;
use web_sys::Element;



fn is_expanded(root: &Element) -> bool {
    root.get_attribute("data-rs-state").map(|s| s.contains("expanded")).unwrap_or(false)
}

fn is_pinned(root: &Element) -> bool {
    root.get_attribute("data-rs-pinned").as_deref() == Some("true")
}

pub fn init(root: Element) {
    if lifecycle::is_initialized(&root) { return; }
    lifecycle::mark_initialized(&root);
    let is_rail = root.get_attribute("data-rs-variant").as_deref() == Some("rail");

    // restore pin from localStorage
    if let Some(win) = web_sys::window() {
        if let Ok(Some(storage)) = win.local_storage() {
            if let Ok(Some(val)) = storage.get_item("sidebar-pinned") {
                if val == "true" {
                    let _ = root.set_attribute("data-rs-pinned", "true");
                    state::remove_state(&root, "collapsed");
                    state::add_state(&root, "expanded");
                }
            }
        }
    }

    // toggle button
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-sidebar-toggle]").ok().flatten().is_none() { return; }
            if is_pinned(&root_cb) { return; }
            if is_expanded(&root_cb) {
                state::remove_state(&root_cb, "expanded");
                state::add_state(&root_cb, "collapsed");
            } else {
                state::remove_state(&root_cb, "collapsed");
                state::add_state(&root_cb, "expanded");
            }
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // pin toggle
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-sidebar-pin-toggle]").ok().flatten().is_none() { return; }
            let pinned = is_pinned(&root_cb);
            let _ = root_cb.set_attribute("data-rs-pinned", if pinned { "false" } else { "true" });
            if !pinned {
                state::remove_state(&root_cb, "collapsed");
                state::add_state(&root_cb, "expanded");
            }
            if let Some(win) = web_sys::window() {
                if let Ok(Some(storage)) = win.local_storage() {
                    let _ = storage.set_item("sidebar-pinned", if pinned { "false" } else { "true" });
                }
            }
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // rail hover
    if is_rail {
        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |_| {
                state::remove_state(&root_cb, "collapsed");
                state::add_state(&root_cb, "expanded");
            }));
            let _ = root.add_event_listener_with_callback("mouseenter", cb.as_ref().unchecked_ref());
            cb.forget();
        }
        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |_| {
                if !is_pinned(&root_cb) {
                    state::remove_state(&root_cb, "expanded");
                    state::add_state(&root_cb, "collapsed");
                }
            }));
            let _ = root.add_event_listener_with_callback("mouseleave", cb.as_ref().unchecked_ref());
            cb.forget();
        }
    }
}

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-sidebar]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init(el); }
        }
    }
}
