//! Carousel Interaction Engine — DOM-driven, no internal state

use wasm_bindgen::prelude::*;
use crate::shared::{add_state, remove_state};
use wasm_bindgen::JsCast;
use web_sys::Element;

fn is_element_alive(el: &Element) -> bool {
    use wasm_bindgen::JsValue;
    let val: &JsValue = el.as_ref();
    !val.is_undefined() && !val.is_null()
}

fn get_items(root: &Element) -> Vec<Element> {
    let Ok(nodes) = root.query_selector_all("[data-rs-carousel-item]") else { return vec![] };
    (0..nodes.length()).filter_map(|i| nodes.item(i)).filter_map(|n| n.dyn_into::<Element>().ok()).collect()
}

fn get_indicators(root: &Element) -> Vec<Element> {
    let Ok(nodes) = root.query_selector_all("[data-rs-carousel-dot]") else { return vec![] };
    (0..nodes.length()).filter_map(|i| nodes.item(i)).filter_map(|n| n.dyn_into::<Element>().ok()).collect()
}

fn current_index(root: &Element) -> usize {
    root.get_attribute("data-rs-current-index").and_then(|s| s.parse().ok()).unwrap_or(0)
}

fn go_to(root: &Element, idx: usize) {
    if !is_element_alive(root) { return; }
    let items = get_items(root);
    let len = items.len();
    if len == 0 { return; }
    let idx = idx.min(len - 1);
    for (i, item) in items.iter().enumerate() {
        remove_state(item, "active");
        remove_state(item, "inactive");
        if i == idx {
            add_state(item, "active");
            let _ = item.remove_attribute("hidden");
        } else {
            add_state(item, "inactive");
            let _ = item.set_attribute("hidden", "");
        }
        let _ = item.set_attribute("aria-hidden", if i == idx { "false" } else { "true" });
    }
    for (i, ind) in get_indicators(root).iter().enumerate() {
        remove_state(ind, "active");
        remove_state(ind, "inactive");
        if i == idx { add_state(ind, "active"); } else { add_state(ind, "inactive"); }
    }
    let _ = root.set_attribute("data-rs-current-index", &idx.to_string());
}

pub fn init(root: Element) {
    web_sys::console::log_1(&"[carousel] init called".into());
    let items = get_items(&root);
    web_sys::console::log_1(&format!("[carousel] items={}", items.len()).into());
    let len = items.len();
    if len == 0 { return; }

    // config está no wrapper filho data-rs-carousel-wrapper
    let cfg = root.query_selector("[data-rs-carousel-wrapper]").ok().flatten().unwrap_or_else(|| root.clone());
    let autoplay  = cfg.has_attribute("data-rs-autoplay");
    let loop_mode = !cfg.has_attribute("data-rs-loop") || cfg.get_attribute("data-rs-loop").as_deref() != Some("off");
    let interval  = cfg.get_attribute("data-rs-interval").and_then(|s| s.parse::<u32>().ok()).unwrap_or(5000);
    let initial   = cfg.get_attribute("data-rs-initial-index").and_then(|s| s.parse().ok()).unwrap_or(0);

    let _ = root.set_attribute("data-rs-carousel-ready", "");
    go_to(&root, initial);

    // prev
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            if !is_element_alive(&root_cb) { return; }
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-carousel-prev]").ok().flatten().is_none() { return; }
            let items = get_items(&root_cb);
            let c = current_index(&root_cb);
            let next = if c == 0 { if loop_mode { items.len() - 1 } else { 0 } } else { c - 1 };
            go_to(&root_cb, next);
        });
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // next
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            if !is_element_alive(&root_cb) { return; }
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-carousel-next]").ok().flatten().is_none() { return; }
            let items = get_items(&root_cb);
            let c = current_index(&root_cb);
            let next = if c >= items.len() - 1 { if loop_mode { 0 } else { c } } else { c + 1 };
            go_to(&root_cb, next);
        });
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // indicator
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            if !is_element_alive(&root_cb) { return; }
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if let Ok(Some(ind)) = target.closest("[data-rs-carousel-dot]") {
                if let Some(idx_str) = ind.get_attribute("data-rs-index").or_else(|| { let dots = get_indicators(&root_cb); dots.iter().position(|d| d == &ind).map(|i| i.to_string()) }) {
                    if let Ok(idx) = idx_str.parse::<usize>() { go_to(&root_cb, idx); }
                }
            }
        });
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // autoplay
    if autoplay {
        let root_cb = root.clone();
        let cb = Closure::wrap(Box::new(move || {
            if !is_element_alive(&root_cb) { return; }
            let items = get_items(&root_cb);
            let c = current_index(&root_cb);
            let next = if c >= items.len() - 1 { if loop_mode { 0 } else { c } } else { c + 1 };
            go_to(&root_cb, next);
        }) as Box<dyn Fn()>);
        if let Some(win) = web_sys::window() {
            win.set_interval_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), interval as i32).ok();
        }
        cb.forget();
    }
}
