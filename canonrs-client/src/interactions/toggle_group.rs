//! ToggleGroup Interaction Engine
//! Single/multiple selection, keyboard navigation, disabled state

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, KeyboardEvent, MouseEvent};

fn add_state(el: &Element, token: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    if current.split_whitespace().any(|t| t == token) { return; }
    let next = format!("{} {}", current, token).trim().to_string();
    el.set_attribute("data-rs-state", &next).ok();
}

fn remove_state(el: &Element, token: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    let next = current.split_whitespace().filter(|t| *t != token).collect::<Vec<_>>().join(" ");
    el.set_attribute("data-rs-state", &next).ok();
}

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
    if root.get_attribute("data-rs-initialized").as_deref() == Some("true") { return; }
    let _ = root.set_attribute("data-rs-initialized", "true");

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

fn try_init_all(doc: &web_sys::Document) {
    let Ok(nodes) = doc.query_selector_all("[data-rs-toggle-group]") else { return };
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
    opts.set_child_list(true); opts.set_subtree(true);
    if let Some(body) = doc.body() { observer.observe_with_options(&body, &opts).ok(); }
    cb.forget();
    let obs_clone = observer.clone();
    let disconnect = Closure::wrap(Box::new(move || { obs_clone.disconnect(); }) as Box<dyn Fn()>);
    win.set_timeout_with_callback_and_timeout_and_arguments_0(disconnect.as_ref().unchecked_ref(), 5000).ok();
    disconnect.forget();
}
