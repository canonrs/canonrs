//! Radio Interaction Engine — keyboard navigation + selection sync

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


fn get_items(root: &Element) -> Vec<Element> {
    let mut result = Vec::new();
    let Ok(nodes) = root.query_selector_all("[data-rs-radio]") else { return result };
    for i in 0..nodes.length() {
        if let Some(n) = nodes.item(i) {
            if let Ok(el) = n.dyn_into::<Element>() { result.push(el); }
        }
    }
    result
}

fn select_item(root: &Element, value: &str) {
    for item in get_items(root) {
        let item_value = item.get_attribute("data-rs-value").unwrap_or_default();
        remove_state(&item, "selected");
        remove_state(&item, "unselected");
        if item_value == value {
            add_state(&item, "selected");
            let _ = item.set_attribute("aria-checked", "true");
        } else {
            add_state(&item, "unselected");
            let _ = item.set_attribute("aria-checked", "false");
        }
    }
}

pub fn init(root: Element) {
    if is_initialized(&root) { return; }
    mark_initialized(&root);
    // click → select
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(item) = target.closest("[data-rs-radio]").ok().flatten() else { return };
            let state = item.get_attribute("data-rs-state").unwrap_or_default();
            if state.contains("disabled") { return; }
            let value = item.get_attribute("data-rs-value").unwrap_or_default();
            select_item(&root_cb, &value);
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // keyboard → arrow navigation
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-radio]").ok().flatten().is_none() { return; }
            let items = get_items(&root_cb);
            let len = items.len();
            if len == 0 { return; }
            let pos = items.iter().position(|el| el.contains(Some(&target)));
            let next = match e.key().as_str() {
                "ArrowDown" | "ArrowRight" => { e.prevent_default(); pos.map(|p| (p + 1) % len) }
                "ArrowUp"   | "ArrowLeft"  => { e.prevent_default(); pos.map(|p| if p == 0 { len - 1 } else { p - 1 }) }
                _ => None,
            };
            if let Some(next_idx) = next {
                if let Some(item) = items.get(next_idx) {
                    let state = item.get_attribute("data-rs-state").unwrap_or_default();
                    if !state.contains("disabled") {
                        let value = item.get_attribute("data-rs-value").unwrap_or_default();
                        select_item(&root_cb, &value);
                        if let Ok(el) = item.clone().dyn_into::<web_sys::HtmlElement>() { let _ = el.focus(); }
                    }
                }
            }
        }));
        let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-radio]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init(el); }
        }
    }
}
