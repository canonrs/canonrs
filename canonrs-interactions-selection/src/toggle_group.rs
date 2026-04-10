//! ToggleGroup Interaction Engine
//! Single/multiple selection, keyboard navigation, disabled state

use wasm_bindgen::prelude::*;
use crate::runtime::{lifecycle, state, context};

use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

fn get_toggles(root: &Element) -> Vec<Element> {
    let Ok(nodes) = root.query_selector_all("[data-rs-toggle]") else { return vec![] };
    (0..nodes.length())
        .filter_map(|i| nodes.item(i))
        .filter_map(|n| n.dyn_into::<Element>().ok())
        .collect()
}

fn navigable_toggles(root: &Element) -> Vec<Element> {
    get_toggles(root).into_iter()
        .filter(|el| !state::has(el, "disabled"))
        .collect()
}

fn toggle_item(root: &Element, item: &Element) {
    let multiple = root.get_attribute("data-rs-multiple").as_deref() == Some("true");
    let currently_on = state::has(item, "on");
    if !multiple {
        for toggle in get_toggles(root) {
            state::remove(&toggle, "on");
            state::add(&toggle, "off");
            let _ = toggle.set_attribute("aria-pressed", "false");
        }
        if !currently_on {
            state::remove(item, "off");
            state::add(item, "on");
            let _ = item.set_attribute("aria-pressed", "true");
        }
    } else {
        if currently_on {
            state::remove(item, "on");
            state::add(item, "off");
            let _ = item.set_attribute("aria-pressed", "false");
        } else {
            state::remove(item, "off");
            state::add(item, "on");
            let _ = item.set_attribute("aria-pressed", "true");
        }
    }
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    context::propagate_owner(&root);

    // click
    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(rc) = context::find_root(&t, "[data-rs-toggle-group]") else { return };
            let Some(item) = t.closest("[data-rs-toggle]").ok().flatten() else { return };
            if state::has(&rc, "disabled") { return; }
            if state::has(&item, "disabled") { return; }
            e.stop_propagation();
            toggle_item(&rc, &item);
        }));
        root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }

    // keydown
    {
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(rc) = context::find_root(&t, "[data-rs-toggle-group]") else { return };
            if t.closest("[data-rs-toggle]").ok().flatten().is_none() { return; }
            if state::has(&rc, "disabled") { return; }
            match e.key().as_str() {
                " " | "Enter" => {
                    e.prevent_default();
                    if let Some(item) = t.closest("[data-rs-toggle]").ok().flatten() {
                        if !state::has(&item, "disabled") { toggle_item(&rc, &item); }
                    }
                }
                "ArrowRight" | "ArrowDown" => {
                    e.prevent_default();
                    let items = navigable_toggles(&rc);
                    let len = items.len();
                    if let Some(pos) = items.iter().position(|el| el.contains(Some(&t))) {
                        let next = (pos + 1) % len;
                        if let Ok(btn) = items[next].clone().dyn_into::<HtmlElement>() { btn.focus().ok(); }
                    }
                }
                "ArrowLeft" | "ArrowUp" => {
                    e.prevent_default();
                    let items = navigable_toggles(&rc);
                    let len = items.len();
                    if let Some(pos) = items.iter().position(|el| el.contains(Some(&t))) {
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
    let nodes = match doc.query_selector_all("[data-rs-toggle-group]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init(el); }
        }
    }
}
