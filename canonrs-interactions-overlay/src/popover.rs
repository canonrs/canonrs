//! Popover Interaction Engine

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state, query, positioning};

fn open(root: &Element) {
    positioning::auto_side(root, "[data-rs-popover-content]");
    state::open(root);
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(current) = query::safe_current(&e) else { return };
            let Some(target)  = query::safe_target(&e)  else { return };
            if query::closest(&target, "[data-rs-popover-trigger]") {
                if state::is_open(&current) { state::close(&current); } else { open(&current); }
                return;
            }
            if query::closest(&target, "[data-rs-popover-close]") { state::close(&current); }
        });
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(target) = query::safe_target(&e) else { return };
            query::each("[data-rs-popover][data-rs-initialized='true']", |node| {
                if !node.contains(Some(target.as_ref())) && state::is_open(&node) { state::close(&node); }
            });
        });
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let _ = doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        }
        cb.forget();
    }

    {
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
            if e.key() != "Escape" { return; }
            query::each("[data-rs-popover][data-rs-initialized='true']", |node| {
                if state::is_open(&node) { state::close(&node); }
            });
        });
        if let Some(win) = web_sys::window() {
            let _ = win.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        }
        cb.forget();
    }
}
