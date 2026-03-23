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
pub fn register() {
    register_behavior("data-rs-switch", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {

        if root.get_attribute("data-rs-switch-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-switch-attached", "1").ok();

        let switch = root.clone();
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let is_checked = switch.get_attribute("data-rs-checked").unwrap_or_default() == "true";
            switch.set_attribute("data-rs-checked", if is_checked { "false" } else { "true" }).ok();
            switch.set_attribute("aria-checked", if is_checked { "false" } else { "true" }).ok();
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
