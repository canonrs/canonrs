//! ConfirmDialog Interaction Engine

use wasm_bindgen::prelude::*;
use crate::shared::{add_state, remove_state};
use wasm_bindgen::JsCast;
use web_sys::Element;

fn is_element_alive(el: &Element) -> bool {
    use wasm_bindgen::JsValue;
    let val: &JsValue = el.as_ref();
    !val.is_undefined() && !val.is_null()
}

fn is_open(root: &Element) -> bool {
    root.get_attribute("data-rs-state").map(|s| s.contains("open")).unwrap_or(false)
}

fn set_scroll_lock(locked: bool) {
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        if let Some(body) = doc.body() {
            if locked { let _ = body.set_attribute("data-rs-scroll-lock", "true"); }
            else { let _ = body.remove_attribute("data-rs-scroll-lock"); }
        }
    }
}

fn open(root: &Element) {
    if !is_element_alive(root) { return; }
    remove_state(root, "closed");
    add_state(root, "open");
    set_scroll_lock(true);
}

fn close(root: &Element) {
    if !is_element_alive(root) { return; }
    remove_state(root, "open");
    add_state(root, "closed");
    set_scroll_lock(false);
}

pub fn init(root: Element) {
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            if !is_element_alive(&root_cb) { return; }
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-confirm-dialog-trigger]").ok().flatten().is_some() { open(&root_cb); return; }
            if target.closest("[data-rs-confirm-dialog-overlay]").ok().flatten().is_some() { close(&root_cb); return; }
            if target.closest("[data-rs-confirm-dialog-cancel]").ok().flatten().is_some() { close(&root_cb); return; }
            if target.closest("[data-rs-confirm-dialog-confirm]").ok().flatten().is_some() { close(&root_cb); }
        });
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
            if !is_element_alive(&root_cb) { return; }
            if e.key() == "Escape" && is_open(&root_cb) { close(&root_cb); }
        });
        if let Some(win) = web_sys::window() {
            let _ = win.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        }
        cb.forget();
    }
}
