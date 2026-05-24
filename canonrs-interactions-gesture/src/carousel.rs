//! Carousel Interaction Engine

use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::{state, attrs};
use canonrs_interactions_core::runtime::{listeners, timers};

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
        state::remove(item, canonrs_interactions_core::dom::state::State::Active.as_str());
        state::remove(item, canonrs_interactions_core::dom::state::State::Inactive.as_str());
        if i == idx {
            state::add(item, canonrs_interactions_core::dom::state::State::Active.as_str());
            let _ = item.remove_attribute("hidden");
        } else {
            state::add(item, canonrs_interactions_core::dom::state::State::Inactive.as_str());
            let _ = item.set_attribute("hidden", "");
        }
        let _ = item.set_attribute("aria-hidden", if i == idx { "false" } else { "true" });
    }
    for (i, dot) in get_dots(root).iter().enumerate() {
        state::remove(dot, canonrs_interactions_core::dom::state::State::Active.as_str());
        state::remove(dot, canonrs_interactions_core::dom::state::State::Inactive.as_str());
        if i == idx { state::add(dot, canonrs_interactions_core::dom::state::State::Active.as_str()); }
        else        { state::add(dot, canonrs_interactions_core::dom::state::State::Inactive.as_str()); }
    }
    let _ = root.set_attribute("data-rs-current-index", &idx.to_string());
}

pub fn init(root: Element) {
    let items = get_items(&root);
    if items.is_empty() { return; }

    let cfg       = root.query_selector("[data-rs-carousel-wrapper]").ok().flatten().unwrap_or_else(|| root.clone());
    let autoplay  = cfg.has_attribute("data-rs-autoplay");
    let loop_mode = !cfg.has_attribute("data-rs-loop") || cfg.get_attribute("data-rs-loop").as_deref() != Some("off");
    let interval  = cfg.get_attribute("data-rs-interval").and_then(|s| s.parse::<u32>().ok()).unwrap_or(5000);
    let initial   = attrs::get_usize(&cfg, "data-rs-initial-index", 0);
    let uid       = root.get_attribute("data-rs-uid").unwrap_or_default();

    let _ = root.set_attribute("data-rs-carousel-ready", "");
    if autoplay { let _ = root.set_attribute("data-rs-autoplay-active", ""); }
    go_to(&root, initial);

    // click
    listeners::listen(&uid, &root, "click", move |e: web_sys::Event| {
        let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
        let Some(cur) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        let Some(t)   = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
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

    // autoplay — use uid-tracked interval via scheduler
    if autoplay {
        let uid_auto = uid.clone();
        schedule_autoplay(root.clone(), uid_auto, interval, loop_mode);
    }
}

fn schedule_autoplay(root: Element, uid: String, interval: u32, loop_mode: bool) {
    let root_c = root.clone();
    let uid_c  = uid.clone();
    timers::timeout(interval as i32, move || {
        if !root_c.is_connected() { return; } // element removed — stop autoplay
        if !root_c.has_attribute("data-rs-autoplay-active") { return; }
        let items = {
            let Ok(nodes) = root_c.query_selector_all("[data-rs-carousel-item]") else { return };
            (0..nodes.length()).filter_map(|i| nodes.item(i)).filter_map(|n| n.dyn_into::<Element>().ok()).collect::<Vec<_>>()
        };
        let c = root_c.get_attribute("data-rs-current-index").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
        let len = items.len();
        let next = if c >= len-1 { if loop_mode { 0 } else { c } } else { c+1 };
        // go_to inline
        for (i, item) in items.iter().enumerate() {
            canonrs_interactions_core::dom::state::remove(item, canonrs_interactions_core::dom::state::State::Active.as_str());
            canonrs_interactions_core::dom::state::remove(item, canonrs_interactions_core::dom::state::State::Inactive.as_str());
            if i == next { canonrs_interactions_core::dom::state::add(item, canonrs_interactions_core::dom::state::State::Active.as_str()); let _ = item.remove_attribute("hidden"); }
            else         { canonrs_interactions_core::dom::state::add(item, canonrs_interactions_core::dom::state::State::Inactive.as_str()); let _ = item.set_attribute("hidden", ""); }
        }
        let _ = root_c.set_attribute("data-rs-current-index", &next.to_string());
        // reschedule
        schedule_autoplay(root_c, uid_c, interval, loop_mode);
    });
}
