//! LinkGroup Interaction — active state + keyboard nav

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::runtime::{lifecycle, state, query};
use web_sys::Element;

fn active_items(root: &Element) -> Vec<Element> {
    query::all(root, "[data-rs-nav-item]")
        .into_iter()
        .filter(|el| !state::has(el, "disabled"))
        .collect()
}

fn activate(root: &Element, target: &Element) {
    for el in query::all(root, "[data-rs-nav-item]") {
        state::remove_state(&el, "active");
        state::add_state(&el, "inactive");
        let _ = el.remove_attribute("aria-current");
    }
    state::remove_state(target, "inactive");
    state::add_state(target, "active");
    let _ = target.set_attribute("aria-current", "page");
}

fn focus_item(el: &Element) {
    if let Ok(h) = el.clone().dyn_into::<web_sys::HtmlElement>() {
        let _ = h.focus();
    }
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    for el in query::all(&root, "[data-rs-nav-item]") {
        if state::has(&el, "active") {
            let _ = el.set_attribute("aria-current", "page");
        } else {
            let _ = el.remove_attribute("aria-current");
        }
    }

    // click
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(item) = target.closest("[data-rs-nav-item]").ok().flatten() else { return };
            if state::has(&item, "disabled") { return; }
            activate(&root_cb, &item);
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // keyboard
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-nav-item]").ok().flatten().is_none() { return; }

            let items = active_items(&root_cb);
            let len = items.len();
            if len == 0 { return; }
            let pos = items.iter().position(|el| el.contains(Some(target.as_ref())));

            match e.key().as_str() {
                "ArrowDown" | "ArrowRight" => {
                    e.prevent_default();
                    let next = pos.map(|p| (p + 1).min(len - 1)).unwrap_or(0);
                    focus_item(&items[next]);
                }
                "ArrowUp" | "ArrowLeft" => {
                    e.prevent_default();
                    let prev = pos.map(|p| if p == 0 { 0 } else { p - 1 }).unwrap_or(0);
                    focus_item(&items[prev]);
                }
                "Home" => { e.prevent_default(); focus_item(&items[0]); }
                "End"  => { e.prevent_default(); focus_item(&items[len - 1]); }
                _ => {}
            }
        }));
        let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}
