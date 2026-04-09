//! AlertDialog Interaction Engine
//! Alert dialogs are intentional — no Escape to close (destructive action)

use wasm_bindgen::prelude::*;
use crate::shared::{add_state, remove_state};
use wasm_bindgen::JsCast;
use web_sys::Element;

fn is_element_alive(el: &Element) -> bool {
    use wasm_bindgen::JsValue;
    let val: &JsValue = el.as_ref();
    !val.is_undefined() && !val.is_null()
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
            if target.closest("[data-rs-alert-dialog-trigger]").ok().flatten().is_none() { return; }
            open(&root_cb);
        });
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            if !is_element_alive(&root_cb) { return; }
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-alert-dialog-cancel]").ok().flatten().is_some() { close(&root_cb); }
            if target.closest("[data-rs-alert-dialog-confirm]").ok().flatten().is_some() { close(&root_cb); }
        });
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}
