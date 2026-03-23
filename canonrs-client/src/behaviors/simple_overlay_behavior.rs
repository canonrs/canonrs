#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_core::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::MouseEvent;
#[cfg(feature = "hydrate")]
use leptos::prelude::Set;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-simple-overlay", Box::new(|root: &web_sys::Element, state: &ComponentState| -> BehaviorResult<()> {

        if root.get_attribute("data-rs-simple-overlay-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-simple-overlay-attached", "1").ok();

        let open_signal = state.open;

        if let Ok(Some(trigger)) = root.query_selector("[data-rs-trigger]") {
            let root_clone = root.clone();
            let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                let is_open = root_clone.get_attribute("data-rs-state").as_deref() == Some("open");
                if is_open {
                    open_signal.set(false);
                    root_clone.set_attribute("data-rs-state", "closed").ok();
                } else {
                    open_signal.set(true);
                    root_clone.set_attribute("data-rs-state", "open").ok();
                }
            }) as Box<dyn FnMut(_)>);
            trigger.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        if let Ok(Some(close_btn)) = root.query_selector("[data-rs-close]") {
            let root_clone = root.clone();
            let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                open_signal.set(false);
                root_clone.set_attribute("data-rs-state", "closed").ok();
            }) as Box<dyn FnMut(_)>);
            close_btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
