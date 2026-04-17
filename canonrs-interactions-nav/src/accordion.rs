//! Accordion Interaction — DOM behavior para [data-rs-accordion]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state, query, aria};

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

/// Retorna triggers ativos (não-disabled) na ordem DOM
fn active_triggers(root: &Element) -> Vec<Element> {
    query::all(root, "[data-rs-accordion-item]")
        .into_iter()
        .filter(|item| !is_disabled(item))
        .filter_map(|item| query::first(&item, "[data-rs-accordion-trigger]"))
        .collect()
}

fn focus_trigger(el: &Element) {
    if let Ok(h) = el.clone().dyn_into::<web_sys::HtmlElement>() {
        let _ = h.focus();
    }
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    // click
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

    // hover
    {
        let cb_enter = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(trigger) = target.closest("[data-rs-accordion-trigger]").ok().flatten() else { return };
            state::add_state(&trigger, "hover");
        });
        let _ = root.add_event_listener_with_callback("mouseover", cb_enter.as_ref().unchecked_ref());
        cb_enter.forget();
    }
    {
        let cb_leave = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(trigger) = target.closest("[data-rs-accordion-trigger]").ok().flatten() else { return };
            state::remove_state(&trigger, "hover");
        });
        let _ = root.add_event_listener_with_callback("mouseout", cb_leave.as_ref().unchecked_ref());
        cb_leave.forget();
    }

    // keyboard
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            // só age se foco está num trigger
            let Some(trigger) = target.closest("[data-rs-accordion-trigger]").ok().flatten() else { return };

            let triggers = active_triggers(&root_cb);
            if triggers.is_empty() { return; }

            // posição do trigger atual na lista de ativos
            let pos = triggers.iter().position(|t| t == &trigger);
            let len = triggers.len();

            match e.key().as_str() {
                "Enter" | " " => {
                    e.prevent_default();
                    let Some(item) = trigger.closest("[data-rs-accordion-item]").ok().flatten() else { return };
                    if !is_disabled(&item) { toggle_item(&root_cb, &item); }
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
