//! Accordion Init — DOM micro-interactions para [data-rs-accordion]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state, query, keyboard, aria};

fn is_disabled(item: &Element) -> bool {
    item.get_attribute("data-rs-state").map(|s| s.contains("disabled")).unwrap_or(false)
}

fn open_item(item: &Element) {
    state::open(item);
    if let Some(trigger) = query::first(item, "[data-rs-accordion-trigger]") {
        aria::set_expanded(&trigger, true);
    }
}

fn close_item(item: &Element) {
    state::close(item);
    if let Some(trigger) = query::first(item, "[data-rs-accordion-trigger]") {
        aria::set_expanded(&trigger, false);
    }
}

fn toggle_item(root: &Element, item: &Element) {
    let collapsible = root.get_attribute("data-rs-collapsible").as_deref() != Some("false");
    let is_single = root.get_attribute("data-rs-selection").as_deref() != Some("multiple");
    let currently_open = state::is_open(item);

    if currently_open {
        if collapsible { close_item(item); }
    } else {
        if is_single {
            for other in query::all(root, "[data-rs-accordion-item]") {
                if state::is_open(&other) { close_item(&other); }
            }
        }
        open_item(item);
    }
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-accordion-trigger]").ok().flatten().is_none() { return; }
            let Some(item) = target.closest("[data-rs-accordion-item]").ok().flatten() else { return };
            if is_disabled(&item) { return; }
            toggle_item(&root_cb, &item);
        });
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-accordion-trigger]").ok().flatten().is_none() { return; }

            let items = query::all(&root_cb, "[data-rs-accordion-item]");
            let pos = keyboard::find_pos(&items, &target);

            match e.key().as_str() {
                "Enter" | " " => {
                    e.prevent_default();
                    let Some(item) = target.closest("[data-rs-accordion-item]").ok().flatten() else { return };
                    if !is_disabled(&item) { toggle_item(&root_cb, &item); }
                }
                "ArrowDown" => { e.prevent_default(); if let Some(p) = pos { keyboard::focus_next(&items, p, "[data-rs-accordion-trigger]"); } }
                "ArrowUp"   => { e.prevent_default(); if let Some(p) = pos { keyboard::focus_prev(&items, p, "[data-rs-accordion-trigger]"); } }
                "Home"      => { e.prevent_default(); keyboard::focus_first(&items, "[data-rs-accordion-trigger]"); }
                "End"       => { e.prevent_default(); keyboard::focus_last(&items, "[data-rs-accordion-trigger]"); }
                _ => {}
            }
        });
        let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}
