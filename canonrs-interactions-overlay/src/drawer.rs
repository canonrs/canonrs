//! Drawer Interaction Engine

use wasm_bindgen::prelude::*;
use web_sys::Element;
use crate::runtime::{lifecycle, state, query};

fn open(root: &Element)  { state::open(root);  state::set_scroll_lock(true); }
fn close(root: &Element) { state::close(root); state::set_scroll_lock(false); }

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(current) = query::safe_current(&e) else { return };
            let Some(target)  = query::safe_target(&e)  else { return };
            if query::closest(&target, "[data-rs-drawer-trigger]") { open(&current); return; }
            if query::closest(&target, "[data-rs-drawer-overlay]") { close(&current); return; }
            if query::closest(&target, "[data-rs-drawer-close]")   { close(&current); }
        });
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    {
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
            if e.key() != "Escape" { return; }
            query::each("[data-rs-drawer][data-rs-initialized='true']", |node| {
                if state::is_open(&node) { close(&node); }
            });
        });
        if let Some(win) = web_sys::window() {
            let _ = win.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        }
        cb.forget();
    }
}
