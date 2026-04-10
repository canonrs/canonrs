//! Tree Interaction Engine — expand/collapse + selection + keyboard navigation

use wasm_bindgen::prelude::*;
use crate::runtime::{lifecycle, state, context};

use wasm_bindgen::JsCast;
use web_sys::Element;

fn get_items(root: &Element) -> Vec<Element> {
    let Ok(nodes) = root.query_selector_all("[data-rs-tree-item]") else { return vec![] };
    (0..nodes.length())
        .filter_map(|i| nodes.item(i))
        .filter_map(|n| n.dyn_into::<Element>().ok())
        .collect()
}

fn is_expandable(item: &Element) -> bool {
    item.has_attribute("data-rs-expanded")
}

fn is_expanded(item: &Element) -> bool {
    item.get_attribute("data-rs-expanded").as_deref() == Some("true")
}

fn is_disabled(item: &Element) -> bool {
    item.has_attribute("data-rs-disabled")
}

fn toggle_expand(item: &Element) {
    if !is_expandable(item) { return; }
    if is_expanded(item) {
        let _ = item.set_attribute("data-rs-expanded", "false");
        let _ = item.set_attribute("aria-expanded", "false");
    } else {
        let _ = item.set_attribute("data-rs-expanded", "true");
        let _ = item.set_attribute("aria-expanded", "true");
    }
}

fn select_item(root: &Element, item: &Element) {
    for i in get_items(root) {
        state::remove(&i, "active");
        state::remove(&i, "selected");
    }
    state::add(item, "active");
    state::add(item, "selected");
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    context::propagate_owner(&root);

    // click → select + expand/collapse
    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(rc) = context::find_root(&t, "[data-rs-tree]") else { return };
            let Some(item) = t.closest("[data-rs-tree-item]").ok().flatten() else { return };
            if is_disabled(&item) { return; }
            select_item(&rc, &item);
            toggle_expand(&item);
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // keydown → navigation + expand/collapse
    {
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(rc) = context::find_root(&t, "[data-rs-tree]") else { return };
            if t.closest("[data-rs-tree-item]").ok().flatten().is_none() { return; }
            let items = get_items(&rc);
            let len = items.len();
            if len == 0 { return; }
            let pos = items.iter().position(|el| el.contains(Some(&t)));
            match e.key().as_str() {
                "ArrowDown" => {
                    e.prevent_default();
                    if let Some(p) = pos {
                        let next = (p + 1).min(len - 1);
                        if let Ok(el) = items[next].clone().dyn_into::<web_sys::HtmlElement>() { let _ = el.focus(); }
                    }
                }
                "ArrowUp" => {
                    e.prevent_default();
                    if let Some(p) = pos {
                        let prev = if p == 0 { 0 } else { p - 1 };
                        if let Ok(el) = items[prev].clone().dyn_into::<web_sys::HtmlElement>() { let _ = el.focus(); }
                    }
                }
                "ArrowRight" => {
                    e.prevent_default();
                    if let Some(item) = t.closest("[data-rs-tree-item]").ok().flatten() {
                        if is_expandable(&item) && !is_expanded(&item) { toggle_expand(&item); }
                    }
                }
                "ArrowLeft" => {
                    e.prevent_default();
                    if let Some(item) = t.closest("[data-rs-tree-item]").ok().flatten() {
                        if is_expandable(&item) && is_expanded(&item) { toggle_expand(&item); }
                    }
                }
                "Enter" | " " => {
                    e.prevent_default();
                    if let Some(item) = t.closest("[data-rs-tree-item]").ok().flatten() {
                        if !is_disabled(&item) {
                            select_item(&rc, &item);
                            toggle_expand(&item);
                        }
                    }
                }
                _ => {}
            }
        }));
        let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-tree]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init(el); }
        }
    }
}
