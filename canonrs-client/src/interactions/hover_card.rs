//! HoverCard Interaction Engine

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;

fn add_state(el: &Element, state: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    if !current.split_whitespace().any(|s| s == state) {
        let next = if current.is_empty() { state.to_string() } else { format!("{} {}", current, state) };
        el.set_attribute("data-rs-state", &next).ok();
    }
}

fn remove_state(el: &Element, state: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    let next: Vec<&str> = current.split_whitespace().filter(|s| *s != state).collect();
    el.set_attribute("data-rs-state", &next.join(" ")).ok();
}

pub fn init(root: Element) {
    if root.get_attribute("data-rs-initialized").as_deref() == Some("true") { return; }
    let _ = root.set_attribute("data-rs-initialized", "true");

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

fn try_init_all(doc: &web_sys::Document) {
    let nodes = match doc.query_selector_all("[data-rs-hover-card]") { Ok(n) => n, Err(_) => return };
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
    opts.set_child_list(true);
    opts.set_subtree(true);
    if let Some(body) = doc.body() { observer.observe_with_options(&body, &opts).ok(); }
    cb.forget();
}
