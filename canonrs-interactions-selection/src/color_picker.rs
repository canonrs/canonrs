//! ColorPicker Interaction Engine

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


fn is_open(root: &Element) -> bool {
    root.get_attribute("data-rs-state").map(|s| s.contains("open")).unwrap_or(false)
}

pub fn init(root: Element) {
    if is_initialized(&root) { return; }
    mark_initialized(&root);
    // toggle on trigger click
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-color-picker-trigger]").ok().flatten().is_none() { return; }
            e.stop_propagation();
            if is_open(&root_cb) {
                remove_state(&root_cb, "open");
                add_state(&root_cb, "closed");
            } else {
                remove_state(&root_cb, "closed");
                add_state(&root_cb, "open");
            }
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // input change → update swatch via data-rs-color only (token system)
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::Event)>::wrap(Box::new(move |e: web_sys::Event| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) else { return };
            if target.get_attribute("data-rs-color-picker-input").is_none() { return; }
            let value = target.value();
            if let Ok(Some(swatch)) = root_cb.query_selector("[data-rs-color-picker-trigger]") {
                let _ = swatch.set_attribute("data-rs-color", &value);
            }
            if let Ok(Some(display)) = root_cb.query_selector("[data-rs-color-display-value]") {
                display.set_text_content(Some(&value));
                let _ = display.set_attribute("data-rs-color-value", &value);
            }
        }));
        let _ = root.add_event_listener_with_callback("input", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // click outside → close
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if root_cb.contains(Some(&target)) { return; }
            remove_state(&root_cb, "open");
            add_state(&root_cb, "closed");
        }));
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let _ = doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        }
        cb.forget();
    }
}

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-color-picker]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init(el); }
        }
    }
}
