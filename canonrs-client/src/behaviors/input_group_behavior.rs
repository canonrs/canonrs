//! InputGroup Behavior — focus-within e first/last via data-rs-state

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
        let filtered = states
            .split_whitespace()
            .filter(|s| *s != to_remove)
            .collect::<Vec<_>>()
            .join(" ");
        if filtered.is_empty() {
            el.remove_attribute("data-rs-state").ok();
        } else {
            el.set_attribute("data-rs-state", &filtered).ok();
        }
    }
}

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-input-group", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {
        // mark first/last children
        let children = root.query_selector_all("[data-rs-input],[data-rs-input-group-addon]").ok();
        if let Some(children) = children {
            let len = children.length();
            if len > 0 {
                if let Some(first) = children.item(0) {
                    if let Ok(el) = first.dyn_into::<web_sys::Element>() {
                        add_state(&el, "first");
                    }
                }
                if let Some(last) = children.item(len - 1) {
                    if let Ok(el) = last.dyn_into::<web_sys::Element>() {
                        add_state(&el, "last");
                    }
                }
            }
        }

        // focus-within on group
        {
            let group = root.clone();
            let cb = Closure::wrap(Box::new(move |_: web_sys::FocusEvent| {
                add_state(&group, "focus-within");
            }) as Box<dyn FnMut(_)>);
            root.add_event_listener_with_callback("focusin", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        {
            let group = root.clone();
            let cb = Closure::wrap(Box::new(move |_: web_sys::FocusEvent| {
                remove_state(&group, "focus-within");
            }) as Box<dyn FnMut(_)>);
            root.add_event_listener_with_callback("focusout", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
