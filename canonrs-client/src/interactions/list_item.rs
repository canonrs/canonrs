//! ListItem Interaction Engine — selection + keyboard

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

fn get_items(root: &Element) -> Vec<Element> {
    let mut result = Vec::new();
    let Ok(nodes) = root.query_selector_all("[data-rs-list-item]") else { return result };
    for i in 0..nodes.length() {
        if let Some(n) = nodes.item(i) { if let Ok(el) = n.dyn_into::<Element>() { result.push(el); } }
    }
    result
}

fn is_multiple(root: &Element) -> bool {
    root.get_attribute("data-rs-selection").as_deref() == Some("multiple")
}

pub fn init(root: Element) {
    if root.get_attribute("data-rs-initialized").as_deref() == Some("true") { return; }
    let _ = root.set_attribute("data-rs-initialized", "true");

    // click → select
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(item) = target.closest("[data-rs-list-item]").ok().flatten() else { return };
            let state = item.get_attribute("data-rs-state").unwrap_or_default();
            if state.contains("disabled") { return; }
            let is_selected = state.contains("selected");
            let multiple = is_multiple(&root_cb);
            if !multiple {
                for i in get_items(&root_cb) {
                    remove_state(&i, "selected"); add_state(&i, "unselected");
                    let _ = i.set_attribute("aria-selected", "false");
                }
            }
            if is_selected && multiple {
                remove_state(&item, "selected"); add_state(&item, "unselected");
                let _ = item.set_attribute("aria-selected", "false");
            } else {
                remove_state(&item, "unselected"); add_state(&item, "selected");
                let _ = item.set_attribute("aria-selected", "true");
            }
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // hover
    { let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
        let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        if let Ok(Some(item)) = t.closest("[data-rs-list-item]") { add_state(&item, "hover"); }
    })); let _ = root.add_event_listener_with_callback("mouseover", cb.as_ref().unchecked_ref()); cb.forget(); }

    { let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
        let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        if let Ok(Some(item)) = t.closest("[data-rs-list-item]") { remove_state(&item, "hover"); }
    })); let _ = root.add_event_listener_with_callback("mouseout", cb.as_ref().unchecked_ref()); cb.forget(); }
}

fn try_init_all(doc: &web_sys::Document) {
    let nodes = match doc.query_selector_all("[data-rs-list]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) { if let Ok(el) = node.dyn_into::<Element>() { init(el); } }
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
    let observer = match web_sys::MutationObserver::new(cb.as_ref().unchecked_ref()) { Ok(o) => o, Err(_) => { cb.forget(); return } };
    let opts = web_sys::MutationObserverInit::new(); opts.set_child_list(true); opts.set_subtree(true);
    if let Some(body) = doc.body() { observer.observe_with_options(&body, &opts).ok(); }
    cb.forget();
}
