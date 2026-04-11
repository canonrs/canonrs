//! Tabs Init — DOM micro-interactions para [data-rs-tabs]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state, query, keyboard, aria};

fn activate_tab(root: &Element, value: &str) {
    for trigger in query::all(root, "[data-rs-tabs-trigger]") {
        let v = trigger.get_attribute("data-rs-value").unwrap_or_default();
        let is_active = v == value;
        state::remove_state(&trigger, "active");
        state::remove_state(&trigger, "inactive");
        if is_active { state::add_state(&trigger, "active"); } else { state::add_state(&trigger, "inactive"); }
        aria::set_selected(&trigger, is_active);
    }

    for content in query::all(root, "[data-rs-tabs-content]") {
        let v = content.get_attribute("data-rs-value").unwrap_or_default();
        let is_active = v == value;
        state::remove_state(&content, "active");
        state::remove_state(&content, "inactive");
        if is_active {
            state::add_state(&content, "active");
            let _ = content.remove_attribute("hidden");
        } else {
            state::add_state(&content, "inactive");
            let _ = content.set_attribute("hidden", "");
        }
    }

    // Notifica outros componentes
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        use web_sys::CustomEventInit;
        let init = CustomEventInit::new();
        init.set_bubbles(true);
        if let Ok(evt) = web_sys::CustomEvent::new_with_event_init_dict("canon:tab-activated", &init) {
            let _ = doc.dispatch_event(&evt);
        }
    }
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    // click
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(trigger) = target.closest("[data-rs-tabs-trigger]").ok().flatten() else { return };
            let state = trigger.get_attribute("data-rs-state").unwrap_or_default();
            if state.contains("disabled") { return; }
            let value = trigger.get_attribute("data-rs-value").unwrap_or_default();
            activate_tab(&root_cb, &value);
        });
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // keydown
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-tabs-trigger]").ok().flatten().is_none() { return; }

            let items: Vec<Element> = query::all(&root_cb, "[data-rs-tabs-trigger]")
                .into_iter()
                .filter(|el| !el.get_attribute("data-rs-state").map(|s| s.contains("disabled")).unwrap_or(false))
                .collect();

            let pos = keyboard::find_pos(&items, &target);
            let len = items.len();

            match e.key().as_str() {
                "Enter" | " " => {
                    e.prevent_default();
                    let Some(trigger) = target.closest("[data-rs-tabs-trigger]").ok().flatten() else { return };
                    let state = trigger.get_attribute("data-rs-state").unwrap_or_default();
                    if !state.contains("disabled") {
                        let value = trigger.get_attribute("data-rs-value").unwrap_or_default();
                        activate_tab(&root_cb, &value);
                    }
                }
                "ArrowRight" | "ArrowDown" => {
                    e.prevent_default();
                    if let Some(p) = pos {
                        let next = (p + 1) % len;
                        if let Some(el) = items.get(next) {
                            let _ = el.clone().dyn_into::<web_sys::HtmlElement>().map(|el| el.focus());
                        }
                    }
                }
                "ArrowLeft" | "ArrowUp" => {
                    e.prevent_default();
                    if let Some(p) = pos {
                        let prev = if p == 0 { len - 1 } else { p - 1 };
                        if let Some(el) = items.get(prev) {
                            let _ = el.clone().dyn_into::<web_sys::HtmlElement>().map(|el| el.focus());
                        }
                    }
                }
                "Home" => { e.prevent_default(); keyboard::focus_first(&items, "[data-rs-tabs-trigger]"); }
                "End"  => { e.prevent_default(); keyboard::focus_last(&items, "[data-rs-tabs-trigger]"); }
                _ => {}
            }
        });
        let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}
