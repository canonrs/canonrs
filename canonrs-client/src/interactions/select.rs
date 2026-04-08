//! Select Interaction Engine

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;

fn add_state(el: &Element, token: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    if current.split_whitespace().any(|t| t == token) { return; }
    let next = format!("{} {}", current, token).trim().to_string();
    let _ = el.set_attribute("data-rs-state", &next);
}

fn remove_state(el: &Element, token: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    let next = current.split_whitespace().filter(|t| *t != token).collect::<Vec<_>>().join(" ");
    let _ = el.set_attribute("data-rs-state", &next);
}

fn get_items(root: &Element) -> Vec<Element> {
    let mut result = Vec::new();
    let mut i = 0u32;
    loop {
        match root.query_selector(&format!("[data-rs-select-item]:nth-of-type({})", i + 1)) {
            Ok(Some(el)) => { result.push(el); i += 1; }
            _ => break,
        }
    }
    result
}

fn is_disabled(root: &Element) -> bool {
    root.get_attribute("data-rs-state").map(|s| s.contains("disabled")).unwrap_or(false)
}

fn set_open(root: &Element, open: bool) {
    if open { remove_state(root, "closed"); add_state(root, "open"); }
    else { remove_state(root, "open"); add_state(root, "closed"); }
    if let Ok(Some(trigger)) = root.query_selector("[data-rs-select-trigger]") {
        if let Ok(el) = trigger.dyn_into::<web_sys::HtmlElement>() {
            let _ = el.set_attribute("aria-expanded", if open { "true" } else { "false" });
        }
    }
}

fn is_open(root: &Element) -> bool {
    root.get_attribute("data-rs-state").map(|s| s.contains("open")).unwrap_or(false)
}

fn set_selected(root: &Element, value: &str) {
    for item in get_items(root) {
        let matches = item.get_attribute("data-rs-value").map(|v| v == value).unwrap_or(false);
        remove_state(&item, "selected"); remove_state(&item, "unselected");
        if matches { add_state(&item, "selected"); let _ = item.set_attribute("aria-selected", "true"); }
        else { add_state(&item, "unselected"); let _ = item.set_attribute("aria-selected", "false"); }
    }
    if let Ok(Some(span)) = root.query_selector("[data-rs-select-value]") {
        if let Ok(span_el) = span.dyn_into::<web_sys::HtmlElement>() {
            let label = root.query_selector(&format!("[data-rs-select-item][data-rs-value='{}']", value))
                .ok().flatten().and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok())
                .and_then(|n| n.text_content())
                .unwrap_or_else(|| span_el.get_attribute("data-rs-placeholder").unwrap_or_default());
            span_el.set_text_content(Some(&label));
        }
    }
}

fn clear_focused(root: &Element) { for item in get_items(root) { remove_state(&item, "focus"); } }

fn navigable_items(root: &Element) -> Vec<Element> {
    get_items(root).into_iter().filter(|el| !el.get_attribute("data-rs-state").unwrap_or_default().contains("disabled")).collect()
}

fn focused_index(items: &[Element]) -> Option<usize> {
    items.iter().position(|el| el.get_attribute("data-rs-state").map(|s| s.contains("focus")).unwrap_or(false))
}

pub fn init(root: Element) {
    if root.get_attribute("data-rs-initialized").as_deref() == Some("true") { return; }
    let _ = root.set_attribute("data-rs-initialized", "true");

    // click
    { let rc = root.clone(); let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
        let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        if let Ok(Some(item)) = t.closest("[data-rs-select-item]") {
            if item.get_attribute("data-rs-state").unwrap_or_default().contains("disabled") { return; }
            e.stop_propagation();
            let v = item.get_attribute("data-rs-value").unwrap_or_default();
            set_selected(&rc, &v); set_open(&rc, false); clear_focused(&rc); return;
        }
        if t.closest("[data-rs-select-trigger]").ok().flatten().is_some() {
            e.stop_propagation();
            if !is_disabled(&rc) { let o = is_open(&rc); set_open(&rc, !o); }
        }
    })); let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()); cb.forget(); }

    // mouseover
    { let rc = root.clone(); let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
        let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        if let Ok(Some(item)) = t.closest("[data-rs-select-item]") {
            if !item.get_attribute("data-rs-state").unwrap_or_default().contains("disabled") {
                clear_focused(&rc); add_state(&item, "focus");
            }
        }
    })); let _ = root.add_event_listener_with_callback("mouseover", cb.as_ref().unchecked_ref()); cb.forget(); }

    // keydown
    { let rc = root.clone(); let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
        if is_disabled(&rc) { return; }
        match e.key().as_str() {
            "Escape" | "Tab" => { set_open(&rc, false); clear_focused(&rc); }
            " " | "Enter" => { e.prevent_default();
                if !is_open(&rc) { set_open(&rc, true); }
                else {
                    let items = navigable_items(&rc);
                    if let Some(idx) = focused_index(&items) {
                        if let Some(el) = items.get(idx) {
                            let v = el.get_attribute("data-rs-value").unwrap_or_default();
                            set_selected(&rc, &v); set_open(&rc, false); clear_focused(&rc);
                        }
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
                clear_focused(&rc); if let Some(el) = items.get(next) { add_state(el, "focus"); }
            }
            _ => {}
        }
    })); let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref()); cb.forget(); }

    // click outside
    { let rc = root.clone(); let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
        let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        if rc.contains(Some(&t)) { return; }
        set_open(&rc, false); clear_focused(&rc);
    })); if let Some(doc) = web_sys::window().and_then(|w| w.document()) { let _ = doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()); } cb.forget(); }
}

fn try_init_all(doc: &web_sys::Document) {
    let nodes = match doc.query_selector_all("[data-rs-select]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(n) = nodes.item(i) { if let Ok(el) = n.dyn_into::<Element>() { init(el); } }
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
