//! Tree Interaction Engine — expand/collapse + keyboard navigation

use wasm_bindgen::prelude::*;
use crate::runtime::{lifecycle, state};

use wasm_bindgen::JsCast;
use web_sys::Element;



fn is_expanded(el: &Element) -> bool {
    el.get_attribute("data-rs-state").map(|s| s.contains("expanded")).unwrap_or(false)
}

fn get_items(root: &Element) -> Vec<Element> {
    let Ok(nodes) = root.query_selector_all("[data-rs-tree-item]") else { return vec![] };
    (0..nodes.length()).filter_map(|i| nodes.item(i)).filter_map(|n| n.dyn_into::<Element>().ok()).collect()
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(item) = target.closest("[data-rs-tree-item]").ok().flatten() else { return };
            if item.get_attribute("data-rs-has-children").as_deref() != Some("true") { return; }
            if is_expanded(&item) {
                state::remove(&item, "expanded"); state::add(&item, "collapsed");
                let _ = item.set_attribute("aria-expanded", "false");
            } else {
                state::remove(&item, "collapsed"); state::add(&item, "expanded");
                let _ = item.set_attribute("aria-expanded", "true");
            }
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

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
                "ArrowDown" => { e.prevent_default();
                    if let Some(p) = pos {
                        if let Ok(el) = items[(p + 1).min(len - 1)].clone().dyn_into::<web_sys::HtmlElement>() { let _ = el.focus(); }
                    }
                }
                "ArrowUp" => { e.prevent_default();
                    if let Some(p) = pos {
                        if let Ok(el) = items[if p == 0 { 0 } else { p - 1 }].clone().dyn_into::<web_sys::HtmlElement>() { let _ = el.focus(); }
                    }
                }
                "Enter" | " " => { e.prevent_default();
                    if let Some(item) = target.closest("[data-rs-tree-item]").ok().flatten() {
                        if item.get_attribute("data-rs-has-children").as_deref() == Some("true") {
                            if is_expanded(&item) {
                                state::remove(&item, "expanded"); state::add(&item, "collapsed");
                                let _ = item.set_attribute("aria-expanded", "false");
                            } else {
                                state::remove(&item, "collapsed"); state::add(&item, "expanded");
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

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-tree]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init(el); }
        }
    }
}
