//! Carousel Interaction Engine

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use std::cell::RefCell;
use std::rc::Rc;

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
    let Ok(nodes) = root.query_selector_all("[data-rs-carousel-item]") else { return result };
    for i in 0..nodes.length() {
        if let Some(n) = nodes.item(i) {
            if let Ok(el) = n.dyn_into::<Element>() { result.push(el); }
        }
    }
    result
}

fn get_indicators(root: &Element) -> Vec<Element> {
    let mut result = Vec::new();
    let Ok(nodes) = root.query_selector_all("[data-rs-carousel-indicator]") else { return result };
    for i in 0..nodes.length() {
        if let Some(n) = nodes.item(i) {
            if let Ok(el) = n.dyn_into::<Element>() { result.push(el); }
        }
    }
    result
}

fn go_to(root: &Element, idx: usize, items: &[Element]) {
    let len = items.len();
    if len == 0 { return; }
    let idx = idx.min(len - 1);
    for (i, item) in items.iter().enumerate() {
        remove_state(item, "active");
        remove_state(item, "inactive");
        if i == idx { add_state(item, "active"); } else { add_state(item, "inactive"); }
        let _ = item.set_attribute("aria-hidden", if i == idx { "false" } else { "true" });
    }
    let indicators = get_indicators(root);
    for (i, ind) in indicators.iter().enumerate() {
        remove_state(ind, "active");
        remove_state(ind, "inactive");
        if i == idx { add_state(ind, "active"); } else { add_state(ind, "inactive"); }
    }
    let _ = root.set_attribute("data-rs-current-index", &idx.to_string());
}

pub fn init(root: Element) {
    if root.get_attribute("data-rs-initialized").as_deref() == Some("true") { return; }
    let _ = root.set_attribute("data-rs-initialized", "true");

    let items = get_items(&root);
    let len = items.len();
    if len == 0 { return; }

    let current: Rc<RefCell<usize>> = Rc::new(RefCell::new(
        root.get_attribute("data-rs-initial-index")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0)
    ));

    let autoplay = root.get_attribute("data-rs-autoplay").as_deref() == Some("on");
    let loop_state = root.get_attribute("data-rs-loop").as_deref() != Some("off");
    let interval = root.get_attribute("data-rs-interval")
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(5000);

    go_to(&root, *current.borrow(), &items);

    // prev
    {
        let root_cb = root.clone();
        let cur = current.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-carousel-prev]").ok().flatten().is_none() { return; }
            let items = get_items(&root_cb);
            let c = *cur.borrow();
            let next = if c == 0 { if loop_state { items.len() - 1 } else { 0 } } else { c - 1 };
            *cur.borrow_mut() = next;
            go_to(&root_cb, next, &items);
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // next
    {
        let root_cb = root.clone();
        let cur = current.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-carousel-next]").ok().flatten().is_none() { return; }
            let items = get_items(&root_cb);
            let c = *cur.borrow();
            let next = if c >= items.len() - 1 { if loop_state { 0 } else { c } } else { c + 1 };
            *cur.borrow_mut() = next;
            go_to(&root_cb, next, &items);
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // autoplay
    if autoplay {
        let root_cb = root.clone();
        let cur = current.clone();
        let cb = Closure::wrap(Box::new(move || {
            let items = get_items(&root_cb);
            let c = *cur.borrow();
            let next = if c >= items.len() - 1 { if loop_state { 0 } else { c } } else { c + 1 };
            *cur.borrow_mut() = next;
            go_to(&root_cb, next, &items);
        }) as Box<dyn Fn()>);
        if let Some(win) = web_sys::window() {
            win.set_interval_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(), interval as i32
            ).ok();
        }
        cb.forget();
    }
}

fn try_init_all(doc: &web_sys::Document) {
    let nodes = match doc.query_selector_all("[data-rs-carousel]") { Ok(n) => n, Err(_) => return };
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
