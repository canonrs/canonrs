//! RadioGroup Behavior — estado exclusivo (mutually exclusive)

#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;

#[cfg(feature = "hydrate")]
fn add_state(el: &web_sys::Element, state: &str) {
    let mut states = el.get_attribute("data-rs-state").unwrap_or_default();
    if !states.split_whitespace().any(|s| s == state) {
        states = format!("{} {}", states, state).trim().to_string();
    }
    el.set_attribute("data-rs-state", &states).ok();
}

#[cfg(feature = "hydrate")]
fn remove_state(el: &web_sys::Element, to_remove: &str) {
    if let Some(states) = el.get_attribute("data-rs-state") {
        let filtered = states.split_whitespace().filter(|s| *s != to_remove).collect::<Vec<_>>().join(" ");
        if filtered.is_empty() { el.remove_attribute("data-rs-state").ok(); }
        else { el.set_attribute("data-rs-state", &filtered).ok(); }
    }
}

#[cfg(feature = "hydrate")]
fn sync_group_selected(group: &web_sys::Element) {
    if let Ok(items) = group.query_selector_all("[data-rs-radio]") {
        for i in 0..items.length() {
            if let Some(node) = items.get(i) {
                if let Some(item) = node.dyn_ref::<web_sys::Element>() {
                    remove_state(item, "selected");
                    if let Ok(Some(input)) = item.query_selector("[data-rs-radio-input]") {
                        if let Ok(input_el) = input.dyn_into::<web_sys::HtmlInputElement>() {
                            if input_el.checked() {
                                add_state(item, "selected");
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-radio-group", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {
        let group_disabled = root.has_attribute("data-rs-disabled");

        // sync selected determinístico no init
        sync_group_selected(root);

        if let Ok(items) = root.query_selector_all("[data-rs-radio]") {
            for i in 0..items.length() {
                if let Some(node) = items.get(i) {
                    if let Some(item) = node.dyn_ref::<web_sys::Element>() {
                        if group_disabled {
                            add_state(item, "disabled");
                            if let Ok(Some(input)) = item.query_selector("[data-rs-radio-input]") {
                                input.set_attribute("disabled", "").ok();
                            }
                        }
                        let item_focus = item.clone();
                        let item_blur = item.clone();
                        let root_change = root.clone();
                        if let Ok(Some(input)) = item.query_selector("[data-rs-radio-input]") {
                            {
                                let cb = Closure::wrap(Box::new(move |_: web_sys::FocusEvent| {
                                    add_state(&item_focus, "focus");
                                }) as Box<dyn FnMut(_)>);
                                input.add_event_listener_with_callback("focus", cb.as_ref().unchecked_ref()).ok();
                                cb.forget();
                            }
                            {
                                let cb = Closure::wrap(Box::new(move |_: web_sys::FocusEvent| {
                                    remove_state(&item_blur, "focus");
                                }) as Box<dyn FnMut(_)>);
                                input.add_event_listener_with_callback("blur", cb.as_ref().unchecked_ref()).ok();
                                cb.forget();
                            }
                            // change — reset all, set only checked
                            {
                                let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
                                    sync_group_selected(&root_change);
                                }) as Box<dyn FnMut(_)>);
                                input.add_event_listener_with_callback("change", cb.as_ref().unchecked_ref()).ok();
                                cb.forget();
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
