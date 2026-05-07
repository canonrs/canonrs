//! Tabs Interaction Engine
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::{lifecycle, state, query};
use canonrs_interactions_core::behavior::selection::{SelectionConfig, activate_by_value};

const TRIGGER_SEL: &str = "[data-rs-tabs-trigger]";
const CONTENT_SEL: &str = "[data-rs-tabs-content]";

fn trigger_config() -> SelectionConfig {
    SelectionConfig {
        item_selector: TRIGGER_SEL,
        value_attr:    "data-rs-value",
        aria_selected: true,
        aria_current:  false,
    }
}

fn activate_tab(root: &Element, value: &str) {
    // ativa triggers
    activate_by_value(root, value, &trigger_config());

    // ativa contents
    for content in query::all(root, CONTENT_SEL) {
        let v = content.get_attribute("data-rs-value").unwrap_or_default();
        let is_active = v == value;
        state::remove(&content, "active");
        state::remove(&content, "inactive");
        if is_active {
            state::add(&content, "active");
            let _ = content.remove_attribute("hidden");
        } else {
            state::add(&content, "inactive");
            let _ = content.set_attribute("hidden", "");
        }
    }

    // notifica
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        use web_sys::CustomEventInit;
        let init = CustomEventInit::new();
        init.set_bubbles(true);
        if let Ok(evt) = web_sys::CustomEvent::new_with_event_init_dict("canon:tab-activated", &init) {
            let _ = doc.dispatch_event(&evt);
        }
    }
}

fn init_active_tab(root: &Element) {
    let default_val = query::all(root, "[data-rs-tabs-list]")
        .first()
        .and_then(|el| el.get_attribute("data-rs-default-tab"))
        .or_else(|| root.get_attribute("data-rs-default-tab"))
        .unwrap_or_default();

    if !default_val.is_empty() {
        activate_tab(root, &default_val);
    } else if let Some(first) = query::all(root, TRIGGER_SEL).into_iter().next() {
        let value = first.get_attribute("data-rs-value").unwrap_or_default();
        if !value.is_empty() { activate_tab(root, &value); }
    }
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    init_active_tab(&root);

    // click
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(trigger) = target.closest(TRIGGER_SEL).ok().flatten() else { return };
            if state::has(&trigger, "disabled") { return; }
            let value = trigger.get_attribute("data-rs-value").unwrap_or_default();
            activate_tab(&root_cb, &value);
        });
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // keyboard
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest(TRIGGER_SEL).ok().flatten().is_none() { return; }

            let items: Vec<Element> = query::all(&root_cb, TRIGGER_SEL)
                .into_iter()
                .filter(|el| !state::has(el, "disabled"))
                .collect();
            let len = items.len();
            let pos = items.iter().position(|el| el.contains(Some(target.as_ref())));

            match e.key().as_str() {
                "Enter" | " " => {
                    e.prevent_default();
                    let Some(trigger) = target.closest(TRIGGER_SEL).ok().flatten() else { return };
                    if !state::has(&trigger, "disabled") {
                        let value = trigger.get_attribute("data-rs-value").unwrap_or_default();
                        activate_tab(&root_cb, &value);
                    }
                }
                "ArrowRight" | "ArrowDown" => {
                    e.prevent_default();
                    if let Some(p) = pos {
                        let next = (p + 1) % len;
                        if let Ok(h) = items[next].clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); }
                    }
                }
                "ArrowLeft" | "ArrowUp" => {
                    e.prevent_default();
                    if let Some(p) = pos {
                        let prev = if p == 0 { len - 1 } else { p - 1 };
                        if let Ok(h) = items[prev].clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); }
                    }
                }
                "Home" => {
                    e.prevent_default();
                    if let Ok(h) = items[0].clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); }
                }
                "End" => {
                    e.prevent_default();
                    if let Ok(h) = items[len - 1].clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); }
                }
                _ => {}
            }
        });
        let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}
