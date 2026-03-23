//! Toggle Behavior
#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_core::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::HtmlElement;
#[cfg(feature = "hydrate")]
use leptos::prelude::Set;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-toggle", Box::new(|root: &web_sys::Element, state: &ComponentState| -> BehaviorResult<()> {

        if root.get_attribute("data-rs-toggle-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-toggle-attached", "1").ok();

        let open_signal = state.open;
        let root_clone = root.clone();

        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let is_on = root_clone.get_attribute("data-rs-state").as_deref() == Some("on");
            let new_state = if is_on { "off" } else { "on" };
            root_clone.set_attribute("data-rs-state", new_state).ok();
            open_signal.set(!is_on);
        }) as Box<dyn FnMut(_)>);

        if let Ok(el) = root.clone().dyn_into::<HtmlElement>() {
            el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
        }
        closure.forget();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
