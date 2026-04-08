//! Tree Interaction Engine — expand/collapse + keyboard navigation

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

fn is_expanded(el: &Element) -> bool {
    el.get_attribute("data-rs-state").map(|s| s.contains("expanded")).unwrap_or(false)
}

fn get_items(root: &Element) -> Vec<Element> {
    let mut result = Vec::new();
    let Ok(nodes) = root.query_selector_all("[data-rs-tree-item]") else { return result };
    for i in 0..nodes.length() {
        if let Some(n) = nodes.item(i) {
            if let Ok(el) = n.dyn_into::<Element>() { result.push(el); }
        }
    }
    result
}

pub fn init(root: Element) {
    if root.get_attribute("data-rs-initialized").as_deref() == Some("true") { return; }
    let _ = root.set_attribute("data-rs-initialized", "true");

    // click → toggle expand
    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(item) = target.closest("[data-rs-tree-item]").ok().flatten() else { return };
            let has_children = item.get_attribute("data-rs-has-children").as_deref() == Some("true");
            if !has_children { return; }
            if is_expanded(&item) {
                remove_state(&item, "expanded");
                add_state(&item, "collapsed");
                let _ = item.set_attribute("aria-expanded", "false");
            } else {
                remove_state(&item, "collapsed");
                add_state(&item, "expanded");
                let _ = item.set_attribute("aria-expanded", "true");
            }
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // keyboard navigation
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-tree-item]").ok().flatten().is_none() { return; }
            let items = get_items(&root_cb);
            let len = items.len();
            if len == 0 { return; }
            let pos = items.iter().position(|el| el.contains(Some(&target)));
            match e.key().as_str() {
                "ArrowDown" => {
                    e.prevent_default();
                    if let Some(p) = pos {
                        let next = (p + 1).min(len - 1);
                        if let Ok(el) = items[next].clone().dyn_into::<web_sys::HtmlElement>() { let _ = el.focus(); }
                    }
                }
                "ArrowUp" => {
                    e.prevent_default();
                    if let Some(p) = pos {
                        let prev = if p == 0 { 0 } else { p - 1 };
                        if let Ok(el) = items[prev].clone().dyn_into::<web_sys::HtmlElement>() { let _ = el.focus(); }
                    }
                }
                "Enter" | " " => {
                    e.prevent_default();
                    if let Some(item) = target.closest("[data-rs-tree-item]").ok().flatten() {
                        let has_children = item.get_attribute("data-rs-has-children").as_deref() == Some("true");
                        if has_children {
                            if is_expanded(&item) {
                                remove_state(&item, "expanded"); add_state(&item, "collapsed");
                                let _ = item.set_attribute("aria-expanded", "false");
                            } else {
                                remove_state(&item, "collapsed"); add_state(&item, "expanded");
                                let _ = item.set_attribute("aria-expanded", "true");
                            }
                        }
                    }
                }
                _ => {}
            }
        }));
        let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}

fn try_init_all(doc: &web_sys::Document) {
    let nodes = match doc.query_selector_all("[data-rs-tree]") { Ok(n) => n, Err(_) => return };
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
    let observer = match web_sys::MutationObserver::new(cb.as_ref().unchecked_ref()) { Ok(o) => o, Err(_) => { cb.forget(); return } };
    let opts = web_sys::MutationObserverInit::new(); opts.set_child_list(true); opts.set_subtree(true);
    if let Some(body) = doc.body() { observer.observe_with_options(&body, &opts).ok(); }
    cb.forget();
}
