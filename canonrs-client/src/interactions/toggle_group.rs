//! ToggleGroup Interaction Engine
//! Single/multiple selection, keyboard navigation, disabled state

use wasm_bindgen::prelude::*;
use crate::shared::{add_state, remove_state, is_initialized, mark_initialized};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, KeyboardEvent, MouseEvent};


fn get_toggles(root: &Element) -> Vec<Element> {
    let Ok(nodes) = root.query_selector_all("[data-rs-toggle]") else { return vec![] };
    (0..nodes.length()).filter_map(|i| nodes.item(i).and_then(|n| n.dyn_into::<Element>().ok())).collect()
}

fn is_on(el: &Element) -> bool {
    el.get_attribute("data-rs-state").map(|s| s.contains("on")).unwrap_or(false)
}

fn is_disabled_el(el: &Element) -> bool {
    el.get_attribute("data-rs-state").map(|s| s.contains("disabled")).unwrap_or(false)
}

fn is_group_disabled(root: &Element) -> bool {
    root.get_attribute("data-rs-state").map(|s| s.contains("disabled")).unwrap_or(false)
}

fn is_multiple(root: &Element) -> bool {
    root.get_attribute("data-rs-multiple").as_deref() == Some("true")
}

fn toggle_item(root: &Element, item: &Element) {
    let multiple = is_multiple(root);
    let currently_on = is_on(item);
    if !multiple {
        for toggle in get_toggles(root) {
            remove_state(&toggle, "on");
            add_state(&toggle, "off");
            toggle.set_attribute("aria-pressed", "false").ok();
        }
        if !currently_on {
            remove_state(item, "off");
            add_state(item, "on");
            item.set_attribute("aria-pressed", "true").ok();
        }
    } else {
        if currently_on {
            remove_state(item, "on");
            add_state(item, "off");
            item.set_attribute("aria-pressed", "false").ok();
        } else {
            remove_state(item, "off");
            add_state(item, "on");
            item.set_attribute("aria-pressed", "true").ok();
        }
    }
}

pub fn init(root: Element) {
    if is_initialized(&root) { return; }
    mark_initialized(&root);
    // click
    {
        let root_c = root.clone();
        let cb = Closure::<dyn Fn(MouseEvent)>::wrap(Box::new(move |e: MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(item) = target.closest("[data-rs-toggle]").ok().flatten() else { return };
            if is_group_disabled(&root_c) { return; }
            if is_disabled_el(&item) { return; }
            e.stop_propagation();
            toggle_item(&root_c, &item);
        }));
        root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }

    // keydown
    {
        let root_c = root.clone();
        let cb = Closure::<dyn Fn(KeyboardEvent)>::wrap(Box::new(move |e: KeyboardEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-toggle]").ok().flatten().is_none() { return; }
            if is_group_disabled(&root_c) { return; }
            match e.key().as_str() {
                " " | "Enter" => {
                    e.prevent_default();
                    if let Some(item) = target.closest("[data-rs-toggle]").ok().flatten() {
                        if !is_disabled_el(&item) { toggle_item(&root_c, &item); }
                    }
                }
                "ArrowRight" | "ArrowDown" => {
                    e.prevent_default();
                    let items = get_toggles(&root_c);
                    let len = items.len();
                    if let Some(pos) = items.iter().position(|el| el.contains(Some(&target))) {
                        let next = (pos + 1) % len;
                        if let Ok(btn) = items[next].clone().dyn_into::<HtmlElement>() { btn.focus().ok(); }
                    }
                }
                "ArrowLeft" | "ArrowUp" => {
                    e.prevent_default();
                    let items = get_toggles(&root_c);
                    let len = items.len();
                    if let Some(pos) = items.iter().position(|el| el.contains(Some(&target))) {
                        let prev = if pos == 0 { len - 1 } else { pos - 1 };
                        if let Ok(btn) = items[prev].clone().dyn_into::<HtmlElement>() { btn.focus().ok(); }
                    }
                }
                _ => {}
            }
        }));
        root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }
}

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-toggle]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init(el); }
        }
    }
}
