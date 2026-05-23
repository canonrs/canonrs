//! Accordion Interaction Engine
//! Core: dom/{state} + behavior/disclosure::{toggle, active_triggers, init_state}

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::{state};
use canonrs_interactions_core::behavior::disclosure::{
    DisclosureConfig, SelectionMode, toggle, active_triggers, init_state,
};

fn make_config(root: &Element) -> DisclosureConfig {
    let collapsible = root.get_attribute("data-rs-collapsible").as_deref() != Some("false");
    let mode = if root.get_attribute("data-rs-selection").as_deref() == Some("multiple") {
        SelectionMode::Multiple
    } else {
        SelectionMode::Single
    };
    DisclosureConfig {
        item_selector:    "[data-rs-accordion-item]",
        trigger_selector: "[data-rs-accordion-trigger]",
        mode,
        collapsible,
    }
}

fn focus_trigger(el: &Element) {
    if let Ok(h) = el.clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); }
}

pub fn init(root: Element) {

    init_state(&root, &make_config(&root));

    // click
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-accordion-trigger]").ok().flatten().is_none() { return; }
            let Some(item) = target.closest("[data-rs-accordion-item]").ok().flatten() else { return };
            if state::has(&item, canonrs_interactions_core::dom::state::State::Disabled.as_str()) { return; }
            toggle(&root_cb, &item, &make_config(&root_cb));
        });
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // hover
    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(trigger) = target.closest("[data-rs-accordion-trigger]").ok().flatten() else { return };
            state::add(&trigger, canonrs_interactions_core::dom::state::State::Hover.as_str());
        });
        let _ = root.add_event_listener_with_callback("mouseover", cb.as_ref().unchecked_ref());
        cb.forget();
    }
    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(trigger) = target.closest("[data-rs-accordion-trigger]").ok().flatten() else { return };
            state::remove(&trigger, canonrs_interactions_core::dom::state::State::Hover.as_str());
        });
        let _ = root.add_event_listener_with_callback("mouseout", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // keyboard
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(trigger) = target.closest("[data-rs-accordion-trigger]").ok().flatten() else { return };
            let config   = make_config(&root_cb);
            let triggers = active_triggers(&root_cb, &config);
            if triggers.is_empty() { return; }
            let pos = triggers.iter().position(|t| t == &trigger);
            let len = triggers.len();
            match e.key().as_str() {
                "Enter" | " " => {
                    e.prevent_default();
                    let Some(item) = trigger.closest("[data-rs-accordion-item]").ok().flatten() else { return };
                    if !state::has(&item, canonrs_interactions_core::dom::state::State::Disabled.as_str()) { toggle(&root_cb, &item, &config); }
                }
                "ArrowDown" => {
                    e.prevent_default();
                    let next = pos.map(|p| (p + 1).min(len - 1)).unwrap_or(0);
                    focus_trigger(&triggers[next]);
                }
                "ArrowUp" => {
                    e.prevent_default();
                    let prev = pos.map(|p| if p == 0 { 0 } else { p - 1 }).unwrap_or(0);
                    focus_trigger(&triggers[prev]);
                }
                "Home" => { e.prevent_default(); focus_trigger(&triggers[0]); }
                "End"  => { e.prevent_default(); focus_trigger(&triggers[len - 1]); }
                _ => {}
            }
        });
        let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}
