//! Combobox Interaction Engine

use wasm_bindgen::prelude::*;
use crate::runtime::{lifecycle, state, popup, context};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlInputElement};

fn get_items(root: &Element) -> Vec<Element> {
    let Ok(list) = root.query_selector_all("[data-rs-combobox-item]") else { return vec![] };
    (0..list.length()).filter_map(|i| list.item(i)).filter_map(|n| n.dyn_into::<Element>().ok()).collect()
}

fn is_disabled(root: &Element) -> bool {
    root.get_attribute("data-rs-state").map(|s| s.contains("disabled")).unwrap_or(false)
}

fn set_open(root: &Element, open: bool) {
    if open { state::remove(root, "closed"); state::add(root, "open"); }
    else { state::remove(root, "open"); state::add(root, "closed"); }
    let _ = root.set_attribute("aria-expanded", if open { "true" } else { "false" });
}

fn is_open(root: &Element) -> bool {
    let v = root.get_attribute("data-rs-state").map(|s| s.contains("open")).unwrap_or(false);
    v
}

fn get_input(root: &Element) -> Option<HtmlInputElement> {
    root.query_selector("[data-rs-combobox-input]").ok().flatten()
        .and_then(|n| n.dyn_into::<HtmlInputElement>().ok())
}

fn filter_items(root: &Element, query: &str) {
    let q = query.to_lowercase();
    for item in get_items(root) {
        let label = item.clone().dyn_into::<web_sys::HtmlElement>().ok()
            .and_then(|el| el.text_content()).unwrap_or_default().to_lowercase();
        let hidden = !q.is_empty() && !label.contains(&q);
        if hidden { state::add(&item, "hidden"); } else { state::remove(&item, "hidden"); }
    }
}

fn set_selected(root: &Element, value: &str) {
    for item in get_items(root) {
        let matches = item.get_attribute("data-rs-value").map(|v| v == value).unwrap_or(false);
        state::remove(&item, "selected"); state::remove(&item, "unselected");
        if matches { state::add(&item, "selected"); let _ = item.set_attribute("aria-selected", "true"); }
        else { state::add(&item, "unselected"); let _ = item.set_attribute("aria-selected", "false"); }
    }
    let items = get_items(root);
    for it in &items {
    }
    let label = items.into_iter()
        .find(|el| el.get_attribute("data-rs-value").map(|v| v == value).unwrap_or(false))
        .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok())
        .and_then(|el| el.text_content()).unwrap_or_default();
    let input = get_input(root);
    if let Some(input) = input {
        input.set_value(&label);
    }
}

fn restore_input(root: &Element) {
    let items = get_items(root);
    for it in &items {
    }
    let label = items.into_iter()
        .find(|el| el.get_attribute("data-rs-state").map(|s| s.contains("selected")).unwrap_or(false))
        .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok())
        .and_then(|el| el.text_content()).unwrap_or_default();
    if let Some(input) = get_input(root) { input.set_value(&label); }
}

fn clear_focused(root: &Element) {
    for item in get_items(root) { state::remove(&item, "focus"); }
}

fn navigable_items(root: &Element) -> Vec<Element> {
    get_items(root).into_iter().filter(|el| {
        let state = el.get_attribute("data-rs-state").unwrap_or_default();
        !state.contains("disabled") && !state.contains("hidden")
    }).collect()
}

fn focused_index(items: &[Element]) -> Option<usize> {
    items.iter().position(|el| el.get_attribute("data-rs-state").map(|s| s.contains("focus")).unwrap_or(false))
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    register();
    context::propagate_owner(&root);

    // input — sobe pelo current_target
    { let cb = Closure::<dyn Fn(web_sys::Event)>::wrap(Box::new(move |e: web_sys::Event| {
        let Some(t) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        let Some(rc) = context::find_root(&t, "[data-rs-combobox]") else { return };
        if is_disabled(&rc) { return; }
        let q = get_input(&rc).map(|i| i.value()).unwrap_or_default();
        filter_items(&rc, &q); set_open(&rc, true); clear_focused(&rc);
    })); if let Some(i) = get_input(&root) { let _ = i.add_event_listener_with_callback("input", cb.as_ref().unchecked_ref()); } cb.forget(); }

    // focus no input
    { let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::wrap(Box::new(move |e: web_sys::FocusEvent| {
        let Some(t) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        let Some(rc) = context::find_root(&t, "[data-rs-combobox]") else { return };
        if !is_disabled(&rc) { set_open(&rc, true); }
    })); if let Some(i) = get_input(&root) { let _ = i.add_event_listener_with_callback("focus", cb.as_ref().unchecked_ref()); } cb.forget(); }

    // click no item
    { let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
        let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        if t.has_attribute("data-rs-combobox-input") { return; }
        let Some(rc) = context::find_root(&t, "[data-rs-combobox]") else { return };
        if let Ok(Some(item)) = t.closest("[data-rs-combobox-item]") {
            if item.get_attribute("data-rs-state").unwrap_or_default().contains("disabled") { return; }
            e.stop_propagation();
            let v = item.get_attribute("data-rs-value").unwrap_or_default();
            set_selected(&rc, &v); filter_items(&rc, ""); set_open(&rc, false); clear_focused(&rc);
        }
    })); let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()); cb.forget(); }

    // mouseover
    { let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
        let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        let Some(rc) = context::find_root(&t, "[data-rs-combobox]") else { return };
        if let Ok(Some(item)) = t.closest("[data-rs-combobox-item]") {
            if !item.get_attribute("data-rs-state").unwrap_or_default().contains("disabled") {
                clear_focused(&rc); state::add(&item, "focus");
            }
        }
    })); let _ = root.add_event_listener_with_callback("mouseover", cb.as_ref().unchecked_ref()); cb.forget(); }

    // keydown no input
    { let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
        let Some(t) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        let Some(rc) = context::find_root(&t, "[data-rs-combobox]") else { return };
        if is_disabled(&rc) { return; }
        match e.key().as_str() {
            "Escape" | "Tab" => { restore_input(&rc); filter_items(&rc, ""); set_open(&rc, false); clear_focused(&rc); }
            "Enter" => { e.prevent_default();
                let items = navigable_items(&rc);
                if let Some(idx) = focused_index(&items) {
                    if let Some(el) = items.get(idx) {
                        let v = el.get_attribute("data-rs-value").unwrap_or_default();
                        set_selected(&rc, &v); filter_items(&rc, ""); set_open(&rc, false); clear_focused(&rc);
                    }
                }
            }
            "ArrowDown" | "ArrowUp" => { e.prevent_default();
                if !is_open(&rc) { set_open(&rc, true); }
                let items = navigable_items(&rc); let len = items.len(); if len == 0 { return; }
                let cur = focused_index(&items);
                let next = match (e.key().as_str(), cur) {
                    ("ArrowDown", None) => 0, ("ArrowDown", Some(i)) => (i+1).min(len-1),
                    ("ArrowUp", None) => len-1, ("ArrowUp", Some(i)) => if i==0{0}else{i-1}, _ => 0,
                };
                clear_focused(&rc); if let Some(el) = items.get(next) { state::add(el, "focus"); }
            }
            _ => {}
        }
    })); if let Some(i) = get_input(&root) { let _ = i.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref()); } cb.forget(); }
}

fn close_combobox(root: &web_sys::Element) {
    filter_items(root, "");
    set_open(root, false);
    clear_focused(root);
}

pub fn register() {
    use std::cell::Cell;
    thread_local! { static REGISTERED: Cell<bool> = Cell::new(false); }
    REGISTERED.with(|r| {
        if r.get() { return; }
        r.set(true);
        popup::register_click_outside("[data-rs-combobox]", close_combobox);
    });
}

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-combobox]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<web_sys::Element>() { init(el); }
        }
    }
}
