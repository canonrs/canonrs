//! Button Behavior — composable data-rs-state (hover active focus disabled)

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
fn remove_states(el: &web_sys::Element, to_remove: &[&str]) {
    if let Some(states) = el.get_attribute("data-rs-state") {
        let filtered = states
            .split_whitespace()
            .filter(|s| !to_remove.contains(s))
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
    register_behavior("data-rs-button", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {
        if root.has_attribute("disabled")
            || root.get_attribute("aria-disabled").as_deref() == Some("true") {
            root.set_attribute("data-rs-state", "disabled").ok();
            return Ok(());
        }

        // mouseenter → add hover
        {
            let el = root.clone();
            let cb = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
                add_state(&el, "hover");
            }) as Box<dyn FnMut(_)>);
            root.add_event_listener_with_callback("mouseenter", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        // mouseleave → remove hover + active
        {
            let el = root.clone();
            let cb = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
                remove_states(&el, &["hover", "active"]);
            }) as Box<dyn FnMut(_)>);
            root.add_event_listener_with_callback("mouseleave", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        // pointerdown → add active
        {
            let el = root.clone();
            let cb = Closure::wrap(Box::new(move |_: web_sys::PointerEvent| {
                add_state(&el, "active");
            }) as Box<dyn FnMut(_)>);
            root.add_event_listener_with_callback("pointerdown", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        // pointerup → remove active
        {
            let el = root.clone();
            let cb = Closure::wrap(Box::new(move |_: web_sys::PointerEvent| {
                remove_state(&el, "active");
            }) as Box<dyn FnMut(_)>);
            root.add_event_listener_with_callback("pointerup", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        // pointercancel → remove active
        {
            let el = root.clone();
            let cb = Closure::wrap(Box::new(move |_: web_sys::PointerEvent| {
                remove_state(&el, "active");
            }) as Box<dyn FnMut(_)>);
            root.add_event_listener_with_callback("pointercancel", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        // focus → add focus
        {
            let el = root.clone();
            let cb = Closure::wrap(Box::new(move |_: web_sys::FocusEvent| {
                add_state(&el, "focus");
            }) as Box<dyn FnMut(_)>);
            root.add_event_listener_with_callback("focus", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        // blur → remove focus
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
