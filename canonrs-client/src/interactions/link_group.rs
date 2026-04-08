//! LinkGroup Interaction Engine — active state sync via data-rs-current (DOM-driven)

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

pub fn init(root: Element) {
    if root.get_attribute("data-rs-initialized").as_deref() == Some("true") { return; }
    let _ = root.set_attribute("data-rs-initialized", "true");

    let Ok(links) = root.query_selector_all("[data-rs-nav-item]") else { return };
    for i in 0..links.length() {
        if let Some(node) = links.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() {
                let is_active = el.get_attribute("data-rs-current").as_deref() == Some("true");
                remove_state(&el, "active");
                remove_state(&el, "inactive");
                if is_active {
                    add_state(&el, "active");
                    let _ = el.set_attribute("aria-current", "page");
                } else {
                    add_state(&el, "inactive");
                    let _ = el.set_attribute("aria-current", "false");
                }
            }
        }
    }
}

fn try_init_all(doc: &web_sys::Document) {
    let nodes = match doc.query_selector_all("[data-rs-navigation-group]") { Ok(n) => n, Err(_) => return };
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
    opts.set_child_list(true);
    opts.set_subtree(true);
    if let Some(body) = doc.body() { observer.observe_with_options(&body, &opts).ok(); }
    cb.forget();
}
