#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::HtmlElement;
#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-icon-toggle", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {
        let el = root.clone();
        if el.get_attribute("data-rs-icon-toggle-attached").as_deref() == Some("1") { return Ok(()); }
        el.set_attribute("data-rs-icon-toggle-attached", "1").ok();

        let Ok(toggle) = el.dyn_into::<HtmlElement>() else { return Ok(()); };
        let toggle_clone = toggle.clone();

        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let is_active = toggle_clone.has_attribute("data-rs-active");
            if is_active {
                toggle_clone.remove_attribute("data-rs-active").ok();
                toggle_clone.set_attribute("aria-pressed", "false").ok();
            } else {
                toggle_clone.set_attribute("data-rs-active", "true").ok();
                toggle_clone.set_attribute("aria-pressed", "true").ok();
            }
        }) as Box<dyn FnMut(_)>);

        toggle.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
        closure.forget();
        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}

#[cfg(feature = "hydrate")]
pub fn init_icon_toggle() { register(); }
#[cfg(not(feature = "hydrate"))]
pub fn init_icon_toggle() {}
