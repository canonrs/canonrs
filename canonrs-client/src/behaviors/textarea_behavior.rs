//! Textarea Behavior — data-rs-state only (focus, disabled)

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
    register_behavior("data-rs-textarea", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {
        if root.has_attribute("data-rs-disabled") || root.get_attribute("aria-disabled").as_deref() == Some("true")
            || root.get_attribute("aria-disabled").as_deref() == Some("true") {
            add_state(root, "disabled");
            return Ok(());
        }

        {
            let el = root.clone();
            let cb = Closure::wrap(Box::new(move |_: web_sys::FocusEvent| {
                add_state(&el, "focus");
            }) as Box<dyn FnMut(_)>);
            root.add_event_listener_with_callback("focus", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        {
            let el = root.clone();
            let cb = Closure::wrap(Box::new(move |_: web_sys::FocusEvent| {
                remove_state(&el, "focus");
            }) as Box<dyn FnMut(_)>);
            root.add_event_listener_with_callback("blur", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
