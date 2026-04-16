//! ListItem Interaction Engine — selection + keyboard navigation

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::shared::{is_initialized, mark_initialized};
use crate::runtime::state;

fn get_items(root: &Element) -> Vec<Element> {
    let Ok(nodes) = root.query_selector_all("[data-rs-list-item]:not([data-rs-disabled])") else { return vec![] };
    (0..nodes.length())
        .filter_map(|i| nodes.item(i))
        .filter_map(|n| n.dyn_into::<Element>().ok())
        .collect()
}

fn is_multiple(root: &Element) -> bool {
    root.get_attribute("data-rs-selection").as_deref() == Some("multiple")
}

fn is_selected(el: &Element) -> bool {
    el.get_attribute("data-rs-state")
        .unwrap_or_default()
        .split_whitespace()
        .any(|s| s == "selected")
}

fn select(el: &Element) {
    state::remove(el, "unselected");
    state::add(el, "selected");
    el.set_attribute("aria-selected", "true").ok();
}

fn deselect(el: &Element) {
    state::remove(el, "selected");
    state::add(el, "unselected");
    el.set_attribute("aria-selected", "false").ok();
}

fn focus_item(items: &[Element], idx: usize) {
    if let Some(el) = items.get(idx) {
        if let Ok(el_h) = el.clone().dyn_into::<web_sys::HtmlElement>() {
            el_h.focus().ok();
        }
    }
}

pub fn init(root: Element) {
    if is_initialized(&root) { return; }
    mark_initialized(&root);

    if is_multiple(&root) {
        root.set_attribute("aria-multiselectable", "true").ok();
    }

    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(item) = target.closest("[data-rs-list-item]").ok().flatten() else { return };
            if item.has_attribute("data-rs-disabled") { return; }

            let multiple = is_multiple(&root_cb);
            let selected = is_selected(&item);

            if !multiple {
                for i in get_items(&root_cb) { deselect(&i); }
                select(&item);
            } else if selected {
                deselect(&item);
            } else {
                select(&item);
            }

            let event_init = web_sys::CustomEventInit::new();
            event_init.set_bubbles(true);
            if let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict("rs-list-select", &event_init) {
                root_cb.dispatch_event(&event).ok();
            }
        });
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    {
        let root_kb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
            let key = e.key();
            if !["ArrowUp", "ArrowDown", "Enter", " ", "Home", "End"].contains(&key.as_str()) { return; }

            let items = get_items(&root_kb);
            if items.is_empty() { return; }

            let doc = match web_sys::window().and_then(|w| w.document()) {
                Some(d) => d, None => return,
            };
            let active = doc.active_element();
            let cur_idx = active.as_ref().and_then(|a| {
                items.iter().position(|item| item == a)
            });

            match key.as_str() {
                "ArrowDown" => {
                    e.prevent_default();
                    let next = cur_idx.map(|i| (i + 1).min(items.len() - 1)).unwrap_or(0);
                    focus_item(&items, next);
                }
                "ArrowUp" => {
                    e.prevent_default();
                    let next = cur_idx.map(|i| if i == 0 { 0 } else { i - 1 }).unwrap_or(0);
                    focus_item(&items, next);
                }
                "Home" => {
                    e.prevent_default();
                    focus_item(&items, 0);
                }
                "End" => {
                    e.prevent_default();
                    focus_item(&items, items.len() - 1);
                }
                "Enter" | " " => {
                    e.prevent_default();
                    if let Some(idx) = cur_idx {
                        if let Some(item) = items.get(idx) {
                            let multiple = is_multiple(&root_kb);
                            let selected = is_selected(item);
                            if !multiple {
                                for i in &items { deselect(i); }
                                select(item);
                            } else if selected {
                                deselect(item);
                            } else {
                                select(item);
                            }
                        }
                    }
                }
                _ => {}
            }
        });
        let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-list]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init(el); }
        }
    }
}
