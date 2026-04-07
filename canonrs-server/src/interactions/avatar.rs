//! Avatar Interaction Engine
//! Image load/error state management, fallback visibility

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlImageElement};

pub fn init(root: Element) {
    let key = JsValue::from_str("__rs_avatar_bound");
    let root_val = JsCast::unchecked_ref::<JsValue>(&root);
    if js_sys::Reflect::get(root_val, &key).ok().filter(|v| v.is_truthy()).is_some() { return; }
    js_sys::Reflect::set(root_val, &key, &JsValue::TRUE).ok();

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

fn try_init_all(doc: &web_sys::Document) {
    let Ok(nodes) = doc.query_selector_all("[data-rs-avatar]") else { return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init(el); }
        }
    }
}

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    try_init_all(&doc);
    let doc_obs = doc.clone();
    let cb = Closure::wrap(Box::new(move |_: js_sys::Array, _: web_sys::MutationObserver| {
        try_init_all(&doc_obs);
    }) as Box<dyn FnMut(_, _)>);
    let observer = match web_sys::MutationObserver::new(cb.as_ref().unchecked_ref()) {
        Ok(o) => o, Err(_) => { cb.forget(); return }
    };
    let opts = web_sys::MutationObserverInit::new();
    opts.set_child_list(true); opts.set_subtree(true);
    if let Some(body) = doc.body() { observer.observe_with_options(&body, &opts).ok(); }
    cb.forget();
    let obs_clone = observer.clone();
    let disconnect = Closure::wrap(Box::new(move || { obs_clone.disconnect(); }) as Box<dyn Fn()>);
    win.set_timeout_with_callback_and_timeout_and_arguments_0(disconnect.as_ref().unchecked_ref(), 5000).ok();
    disconnect.forget();
}
