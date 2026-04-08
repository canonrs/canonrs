//! DropdownMenu Interaction — Canon Rule #342
//! Toda a lógica vive aqui. Island apenas chama init_all().

use web_sys::Element;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-dropdown-menu]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init_dropdown(el); }
        }
    }
    bind_click_outside();
}

fn init_dropdown(root: Element) {
    // click — trigger toggle + item select
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-dropdown-menu-trigger]").ok().flatten().is_some() {
                e.stop_propagation();
                if !is_disabled(&root_cb) {
                    let open = is_open(&root_cb);
                    set_open(&root_cb, !open);
                    if !open { clear_focused(&root_cb); }
                }
                return;
            }
            if let Ok(Some(item)) = target.closest("[data-rs-dropdown-menu-item]") {
                let state = item.get_attribute("data-rs-state").unwrap_or_default();
                if state.contains("disabled") { return; }
                e.stop_propagation();
                set_open(&root_cb, false);
                clear_focused(&root_cb);
            }
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }
    // mouseover — focus item
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if let Ok(Some(item)) = target.closest("[data-rs-dropdown-menu-item]") {
                let state = item.get_attribute("data-rs-state").unwrap_or_default();
                if state.contains("disabled") { return; }
                clear_focused(&root_cb);
                add_state(&item, "focus");
            }
        }));
        let _ = root.add_event_listener_with_callback("mouseover", cb.as_ref().unchecked_ref());
        cb.forget();
    }
    // keydown — arrow/enter/escape
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            if !is_open(&root_cb) { return; }
            match e.key().as_str() {
                "Escape" | "Tab" => {
                    set_open(&root_cb, false);
                    clear_focused(&root_cb);
                }
                "Enter" | " " => {
                    e.prevent_default();
                    let items = navigable_items(&root_cb);
                    if focused_index(&items).is_some() {
                        set_open(&root_cb, false);
                        clear_focused(&root_cb);
                    }
                }
                "ArrowDown" => {
                    e.prevent_default();
                    let items = navigable_items(&root_cb);
                    let len = items.len(); if len == 0 { return; }
                    let next = match focused_index(&items) { None => 0, Some(i) => (i + 1).min(len - 1) };
                    clear_focused(&root_cb);
                    if let Some(el) = items.get(next) { add_state(el, "focus"); }
                }
                "ArrowUp" => {
                    e.prevent_default();
                    let items = navigable_items(&root_cb);
                    let len = items.len(); if len == 0 { return; }
                    let next = match focused_index(&items) { None => len - 1, Some(i) => if i == 0 { 0 } else { i - 1 } };
                    clear_focused(&root_cb);
                    if let Some(el) = items.get(next) { add_state(el, "focus"); }
                }
                _ => {}
            }
        }));
        let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}

fn bind_click_outside() {
    let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
        let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        let win = match web_sys::window() { Some(w) => w, None => return };
        let doc = match win.document() { Some(d) => d, None => return };
        let nodes = match doc.query_selector_all("[data-rs-dropdown-menu]") { Ok(n) => n, Err(_) => return };
        for i in 0..nodes.length() {
            if let Some(node) = nodes.item(i) {
                if let Ok(el) = node.dyn_into::<Element>() {
                    if !el.contains(Some(&target)) {
                        set_open(&el, false);
                        clear_focused(&el);
                    }
                }
            }
        }
    }));
    let win = match web_sys::window() { Some(w) => w, None => return };
    if let Some(doc) = win.document() {
        let _ = doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
    }
    cb.forget();
}

fn add_state(el: &Element, token: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    if current.split_whitespace().any(|t| t == token) { return; }
    let _ = el.set_attribute("data-rs-state", &format!("{} {}", current, token).trim().to_string());
}

fn remove_state(el: &Element, token: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    let next = current.split_whitespace().filter(|t| *t != token).collect::<Vec<_>>().join(" ");
    let _ = el.set_attribute("data-rs-state", &next);
}

fn is_disabled(el: &Element) -> bool {
    el.get_attribute("data-rs-state").map(|s| s.contains("disabled")).unwrap_or(false)
}

fn is_open(el: &Element) -> bool {
    el.get_attribute("data-rs-state").map(|s| s.contains("open")).unwrap_or(false)
}

fn set_open(root: &Element, open: bool) {
    if open { remove_state(root, "closed"); add_state(root, "open"); }
    else    { remove_state(root, "open");   add_state(root, "closed"); }
    if let Ok(Some(trigger)) = root.query_selector("[data-rs-dropdown-menu-trigger]") {
        if let Ok(el) = trigger.dyn_into::<web_sys::HtmlElement>() {
            let _ = el.set_attribute("aria-expanded", if open { "true" } else { "false" });
        }
    }
}

fn get_items(root: &Element) -> Vec<Element> {
    root.query_selector_all("[data-rs-dropdown-menu-item]").ok()
        .map(|list| (0..list.length()).filter_map(|i| list.item(i))
            .filter_map(|n| n.dyn_into::<Element>().ok()).collect())
        .unwrap_or_default()
}

fn clear_focused(root: &Element) {
    for item in get_items(root) { remove_state(&item, "focus"); }
}

fn navigable_items(root: &Element) -> Vec<Element> {
    get_items(root).into_iter()
        .filter(|el| !el.get_attribute("data-rs-state").map(|s| s.contains("disabled")).unwrap_or(false))
        .collect()
}

fn focused_index(items: &[Element]) -> Option<usize> {
    items.iter().position(|el| el.get_attribute("data-rs-state").map(|s| s.contains("focus")).unwrap_or(false))
}

pub fn init(root: web_sys::Element) {
    init_dropdown(root);
}
