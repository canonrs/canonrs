//! ButtonGroup Behavior — marca first/last nos filhos via data-rs-state

#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;

#[cfg(feature = "hydrate")]
fn add_state(el: &web_sys::Element, state: &str) {
    let mut states = el.get_attribute("data-rs-state").unwrap_or_default();
    if !states.split_whitespace().any(|s| s == state) {
        states = format!("{} {}", states, state).trim().to_string();
    }
    el.set_attribute("data-rs-state", &states).ok();
}

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-button-group", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {
        let children = root.query_selector_all("[data-rs-button]").ok();
        let children = match children {
            Some(c) => c,
            None => return Ok(()),
        };

        let len = children.length();
        if len == 0 {
            return Ok(());
        }

        use wasm_bindgen::JsCast;

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

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
