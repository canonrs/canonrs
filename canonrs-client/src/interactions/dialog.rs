//! Dialog Interaction Engine

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

fn is_open(root: &Element) -> bool {
    root.get_attribute("data-rs-state").map(|s| s.contains("open")).unwrap_or(false)
}

fn set_scroll_lock(locked: bool) {
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        if let Some(body) = doc.body() {
            if locked {
                let _ = body.set_attribute("data-rs-scroll-lock", "true");
            } else {
                let _ = body.remove_attribute("data-rs-scroll-lock");
            }
        }
    }
}

fn open(root: &Element) {
    remove_state(root, "closed");
    add_state(root, "open");
    set_scroll_lock(true);
    if let Ok(Some(content)) = root.query_selector("[data-rs-dialog-content]") {
        if let Ok(el) = content.dyn_into::<web_sys::HtmlElement>() { let _ = el.focus(); }
    }
}

fn close(root: &Element) {
    remove_state(root, "open");
    add_state(root, "closed");
    set_scroll_lock(false);
}

pub fn init(root: Element) {
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-dialog-trigger]").ok().flatten().is_none() { return; }
            open(&root_cb);
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-dialog-overlay]").ok().flatten().is_some() { close(&root_cb); }
            if target.closest("[data-rs-dialog-close]").ok().flatten().is_some() { close(&root_cb); }
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            if e.key() == "Escape" && is_open(&root_cb) { close(&root_cb); }
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
    let nodes = match doc.query_selector_all("[data-rs-dialog]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init(el); }
        }
    }
}
