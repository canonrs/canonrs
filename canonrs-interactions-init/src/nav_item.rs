//! NavItem + NavGroup Init

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, interactive, state, query};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    if root.has_attribute("data-rs-nav-group") {
        // keyboard navigation usando focus real nos items
        let root_kb = root.clone();
        let kb_cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
            let items: Vec<Element> = query::all(&root_kb, "[data-rs-nav-item]")
                .into_iter()
                .filter(|el| el.get_attribute("data-rs-disabled").as_deref() != Some("true"))
                .collect();
            if items.is_empty() { return; }
            let direction = root_kb.get_attribute("data-rs-direction").unwrap_or_default();
            let is_horizontal = direction == "horizontal";
            let prev_key = if is_horizontal { "ArrowLeft" } else { "ArrowUp" };
            let next_key = if is_horizontal { "ArrowRight" } else { "ArrowDown" };
            let current = items.iter().position(|el| {
                el.get_attribute("data-rs-state").map(|s| s.contains("focused")).unwrap_or(false)
            });
            match e.key().as_str() {
                k if k == next_key => {
                    e.prevent_default();
                    let next = current.map(|i| (i + 1).min(items.len() - 1)).unwrap_or(0);
                    for item in &items { state::remove_state(item, "focused"); }
                    state::add_state(&items[next], "focused");
                    let _ = items[next].clone().dyn_into::<web_sys::HtmlElement>().map(|el| el.focus());
                }
                k if k == prev_key => {
                    e.prevent_default();
                    let prev = current.map(|i| if i == 0 { 0 } else { i - 1 }).unwrap_or(0);
                    for item in &items { state::remove_state(item, "focused"); }
                    state::add_state(&items[prev], "focused");
                    let _ = items[prev].clone().dyn_into::<web_sys::HtmlElement>().map(|el| el.focus());
                }
                _ => {}
            }
        });
        let _ = root.add_event_listener_with_callback("keydown", kb_cb.as_ref().unchecked_ref());
        kb_cb.forget();
    } else {
        interactive::init(&root);
        let r = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let href = r.get_attribute("href").unwrap_or_default();
            if href == "#" || href.is_empty() { e.prevent_default(); }
        });
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}
