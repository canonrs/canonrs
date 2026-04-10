//! Carousel Interaction Engine

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state, attrs};

fn get_items(root: &Element) -> Vec<Element> {
    let Ok(nodes) = root.query_selector_all("[data-rs-carousel-item]") else { return vec![] };
    (0..nodes.length()).filter_map(|i| nodes.item(i)).filter_map(|n| n.dyn_into::<Element>().ok()).collect()
}

fn get_dots(root: &Element) -> Vec<Element> {
    let Ok(nodes) = root.query_selector_all("[data-rs-carousel-dot]") else { return vec![] };
    (0..nodes.length()).filter_map(|i| nodes.item(i)).filter_map(|n| n.dyn_into::<Element>().ok()).collect()
}

fn current_index(root: &Element) -> usize {
    attrs::get_usize(root, "data-rs-current-index", 0)
}

fn go_to(root: &Element, idx: usize) {
    let items = get_items(root);
    let len = items.len(); if len == 0 { return; }
    let idx = idx.min(len - 1);
    for (i, item) in items.iter().enumerate() {
        state::remove(item, "active"); state::remove(item, "inactive");
        if i == idx { state::add(item, "active"); let _ = item.remove_attribute("hidden"); }
        else        { state::add(item, "inactive"); let _ = item.set_attribute("hidden", ""); }
        let _ = item.set_attribute("aria-hidden", if i == idx { "false" } else { "true" });
    }
    for (i, dot) in get_dots(root).iter().enumerate() {
        state::remove(dot, "active"); state::remove(dot, "inactive");
        if i == idx { state::add(dot, "active"); } else { state::add(dot, "inactive"); }
    }
    let _ = root.set_attribute("data-rs-current-index", &idx.to_string());
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    let items = get_items(&root);
    if items.is_empty() { return; }

    let cfg = root.query_selector("[data-rs-carousel-wrapper]").ok().flatten().unwrap_or_else(|| root.clone());
    let autoplay  = cfg.has_attribute("data-rs-autoplay");
    let loop_mode = !cfg.has_attribute("data-rs-loop") || cfg.get_attribute("data-rs-loop").as_deref() != Some("off");
    let interval  = cfg.get_attribute("data-rs-interval").and_then(|s| s.parse::<u32>().ok()).unwrap_or(5000);
    let initial   = attrs::get_usize(&cfg, "data-rs-initial-index", 0);

    let _ = root.set_attribute("data-rs-carousel-ready", "");
    if autoplay { let _ = root.set_attribute("data-rs-autoplay-active", ""); }
    go_to(&root, initial);

    // click
    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(cur) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let c = current_index(&cur); let items = get_items(&cur); let len = items.len();
            if t.closest("[data-rs-carousel-prev]").ok().flatten().is_some() {
                go_to(&cur, if c == 0 { if loop_mode { len-1 } else { 0 } } else { c-1 });
            } else if t.closest("[data-rs-carousel-next]").ok().flatten().is_some() {
                go_to(&cur, if c >= len-1 { if loop_mode { 0 } else { c } } else { c+1 });
            } else if let Ok(Some(dot)) = t.closest("[data-rs-carousel-dot]") {
                let idx = dot.get_attribute("data-rs-index")
                    .or_else(|| get_dots(&cur).iter().position(|d| d == &dot).map(|i| i.to_string()))
                    .and_then(|s| s.parse::<usize>().ok());
                if let Some(i) = idx { go_to(&cur, i); }
            }
        });
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // autoplay
    if autoplay {
        use std::cell::Cell;
        thread_local! { static CAROUSEL_COUNT: Cell<usize> = Cell::new(0); }
        let carousel_idx = CAROUSEL_COUNT.with(|c| { let v = c.get(); c.set(v+1); v });
        let cb = Closure::wrap(Box::new(move || {
            let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
            let Ok(nodes) = doc.query_selector_all("[data-rs-carousel-ready][data-rs-autoplay-active]") else { return };
            let Some(root_el) = nodes.item(carousel_idx as u32).and_then(|n| n.dyn_into::<Element>().ok()) else { return };
            let items = get_items(&root_el); let c = current_index(&root_el); let len = items.len();
            go_to(&root_el, if c >= len-1 { if loop_mode { 0 } else { c } } else { c+1 });
        }) as Box<dyn Fn()>);
        if let Some(win) = web_sys::window() {
            win.set_interval_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), interval as i32).ok();
        }
        cb.forget();
    }
}
