//! Radio Interaction Engine — keyboard navigation + selection sync

use wasm_bindgen::prelude::*;
use crate::runtime::{lifecycle, state, context};
use wasm_bindgen::JsCast;
use web_sys::Element;

fn get_items(root: &Element) -> Vec<Element> {
    let Ok(nodes) = root.query_selector_all("[data-rs-radio]") else { return vec![] };
    (0..nodes.length())
        .filter_map(|i| nodes.item(i))
        .filter_map(|n| n.dyn_into::<Element>().ok())
        .collect()
}

fn navigable_items(root: &Element) -> Vec<Element> {
    get_items(root).into_iter()
        .filter(|el| !state::has(el, "disabled"))
        .collect()
}

fn item_value(item: &Element) -> String {
    item.query_selector("[data-rs-radio-input]").ok().flatten()
        .and_then(|n| n.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|el| el.value())
        .unwrap_or_default()
}

fn set_input_checked(item: &Element, checked: bool) {
    if let Ok(Some(node)) = item.query_selector("[data-rs-radio-input]") {
        if let Ok(input) = node.dyn_into::<web_sys::HtmlInputElement>() {
            input.set_checked(checked);
        }
    }
}

fn set_tabindex(item: &Element, idx: &str) {
    if let Ok(el) = item.clone().dyn_into::<web_sys::HtmlElement>() {
        let _ = el.set_attribute("tabindex", idx);
    }
}

fn focus_item(item: &Element) {
    if let Ok(el) = item.clone().dyn_into::<web_sys::HtmlElement>() {
        let _ = el.focus();
    }
}

fn select_item(root: &Element, value: &str) {
    for item in get_items(root) {
        let matches = item_value(&item) == value;
        state::remove(&item, "selected");
        state::remove(&item, "unselected");
        if matches {
            state::add(&item, "selected");
            let _ = item.set_attribute("aria-checked", "true");
            set_input_checked(&item, true);
            set_tabindex(&item, "0");
        } else {
            state::add(&item, "unselected");
            let _ = item.set_attribute("aria-checked", "false");
            set_input_checked(&item, false);
            set_tabindex(&item, "-1");
        }
    }
    let _ = root.set_attribute("data-rs-value", value);
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    context::propagate_owner(&root);

    // SSR bootstrap — roving tabindex + garantir consistência
    {
        let items = get_items(&root);
        let has_selected = items.iter().any(|el| state::has(el, "selected"));
        for (i, item) in items.iter().enumerate() {
            let selected = state::has(item, "selected");
            if selected {
                set_tabindex(item, "0");
            } else if !has_selected && i == 0 {
                // sem seleção SSR → primeiro item focável
                set_tabindex(item, "0");
            } else {
                set_tabindex(item, "-1");
            }
        }
    }



    // focus/blur — state "focus" no item
    {
        let root_focus = root.clone();
        let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::wrap(Box::new(move |e: web_sys::FocusEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(item) = t.closest("[data-rs-radio]").ok().flatten() else { return };
            for el in get_items(&root_focus) { state::remove(&el, "focus"); }
            state::add(&item, "focus");
        }));
        let _ = root.add_event_listener_with_callback("focusin", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    {
        let root_blur = root.clone();
        let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::wrap(Box::new(move |_: web_sys::FocusEvent| {
            for el in get_items(&root_blur) { state::remove(&el, "focus"); }
        }));
        let _ = root.add_event_listener_with_callback("focusout", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // click → select
    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(rc) = context::find_root(&t, "[data-rs-radio-group]") else { return };
            let Some(item) = t.closest("[data-rs-radio]").ok().flatten() else { return };
            if state::has(&item, "disabled") { return; }
            let value = item_value(&item);
            select_item(&rc, &value);
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // keydown → roving tabindex navigation
    {
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(rc) = context::find_root(&t, "[data-rs-radio-group]") else { return };
            if t.closest("[data-rs-radio]").ok().flatten().is_none() { return; }
            let items = navigable_items(&rc);
            let len = items.len();
            if len == 0 { return; }
            let pos = items.iter().position(|el| el.contains(Some(&t)));
            let next_idx = match e.key().as_str() {
                "ArrowDown" | "ArrowRight" => { e.prevent_default(); pos.map(|p| (p + 1) % len) }
                "ArrowUp"   | "ArrowLeft"  => { e.prevent_default(); pos.map(|p| if p == 0 { len - 1 } else { p - 1 }) }
                _ => None,
            };
            if let Some(idx) = next_idx {
                if let Some(item) = items.get(idx) {
                    let value = item_value(item);
                    select_item(&rc, &value);
                    focus_item(item);
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
    let nodes = match doc.query_selector_all("[data-rs-radio-group]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init(el); }
        }
    }
}
