#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::MouseEvent;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-icon-button", Box::new(|id: &str, _state: &ComponentState| -> BehaviorResult<()> {
        use leptos::leptos_dom::helpers::document;

        let Some(el) = document().get_element_by_id(id) else { return Ok(()); };
        if el.get_attribute("data-icon-button-attached").as_deref() == Some("1") { return Ok(()); }
        el.set_attribute("data-icon-button-attached", "1").ok();

        let el_clone = el.clone();
        let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
            let is_active = el_clone.has_attribute("data-active");
            if is_active {
                el_clone.remove_attribute("data-active").ok();
                el_clone.set_attribute("aria-pressed", "false").ok();
            } else {
                el_clone.set_attribute("data-active", "true").ok();
                el_clone.set_attribute("aria-pressed", "true").ok();
            }
        }) as Box<dyn FnMut(_)>);
        el.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
        cb.forget();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}

#[cfg(feature = "hydrate")]
pub fn init_icon_button() { register(); }
#[cfg(not(feature = "hydrate"))]
pub fn init_icon_button() {}
