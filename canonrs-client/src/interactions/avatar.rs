//! Avatar Interaction Engine
//! Image load/error state management, fallback visibility

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlImageElement};
use crate::shared::{is_initialized, mark_initialized};

pub fn init(root: Element) {
    if is_initialized(&root) { return; }
    mark_initialized(&root);
    let Ok(Some(img_node)) = root.query_selector("[data-rs-avatar-image]") else {
        // no image — show fallback directly
        show_fallback(&root);
        return;
    };
    let Ok(img) = img_node.dyn_into::<HtmlImageElement>() else { return };

    // already loaded
    if img.complete() && img.natural_width() > 0 {
        root.set_attribute("data-rs-state", "loaded").ok();
        return;
    }

    // loading state
    root.set_attribute("data-rs-state", "loading").ok();

    // onload
    {
        let root_c = root.clone();
        let cb = Closure::wrap(Box::new(move || {
            root_c.set_attribute("data-rs-state", "loaded").ok();
            hide_fallback(&root_c);
        }) as Box<dyn Fn()>);
        img.set_onload(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    }

    // onerror
    {
        let root_c = root.clone();
        let cb = Closure::wrap(Box::new(move || {
            root_c.set_attribute("data-rs-state", "error").ok();
            show_fallback(&root_c);
        }) as Box<dyn Fn()>);
        img.set_onerror(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    }
}

fn show_fallback(root: &Element) {
    if let Ok(Some(fb)) = root.query_selector("[data-rs-avatar-fallback]") {
        fb.remove_attribute("hidden").ok();
        fb.set_attribute("data-rs-state", "visible").ok();
    }
    if let Ok(Some(img)) = root.query_selector("[data-rs-avatar-image]") {
        img.set_attribute("data-rs-state", "hidden").ok();
    }
}

fn hide_fallback(root: &Element) {
    if let Ok(Some(fb)) = root.query_selector("[data-rs-avatar-fallback]") {
        fb.set_attribute("hidden", "").ok();
        fb.set_attribute("data-rs-state", "hidden").ok();
    }
}

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-avatar]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init(el); }
        }
    }
}
