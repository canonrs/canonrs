//! Menubar Interaction Engine

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

fn get_menus(root: &Element) -> Vec<Element> {
    let mut result = Vec::new();
    let Ok(nodes) = root.query_selector_all("[data-rs-menubar-menu]") else { return result };
    for i in 0..nodes.length() {
        if let Some(n) = nodes.item(i) {
            if let Ok(el) = n.dyn_into::<Element>() { result.push(el); }
        }
    }
    result
}

fn close_all(root: &Element) {
    for menu in get_menus(root) {
        remove_state(&menu, "open");
        add_state(&menu, "closed");
        if let Ok(Some(trigger)) = menu.query_selector("[data-rs-menubar-trigger]") {
            let _ = trigger.set_attribute("aria-expanded", "false");
        }
    }
}

pub fn init(root: Element) {
    // click trigger → toggle menu
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(trigger) = target.closest("[data-rs-menubar-trigger]").ok().flatten() else { return };
            let Some(menu) = trigger.closest("[data-rs-menubar-menu]").ok().flatten() else { return };
            e.stop_propagation();
            let is_open = menu.get_attribute("data-rs-state").map(|s| s.contains("open")).unwrap_or(false);
            close_all(&root_cb);
            if !is_open {
                remove_state(&menu, "closed");
                add_state(&menu, "open");
                let _ = trigger.set_attribute("aria-expanded", "true");
            }
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // click item → close all
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-menubar-item]").ok().flatten().is_none() { return; }
            let disabled = target.get_attribute("aria-disabled").as_deref() == Some("true");
            if !disabled { close_all(&root_cb); }
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // click outside → close all
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if root_cb.contains(Some(&target)) { return; }
            close_all(&root_cb);
        }));
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let _ = doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        }
        cb.forget();
    }

    // Escape → close all
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            if e.key() == "Escape" { close_all(&root_cb); }
        }));
        if let Some(win) = web_sys::window() {
            let _ = win.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        }
        cb.forget();
    }
}

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-menubar-menu]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init(el); }
        }
    }
}
