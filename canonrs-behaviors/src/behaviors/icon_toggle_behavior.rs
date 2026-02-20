#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::HtmlElement;
#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-icon-toggle", Box::new(|id: &str, _state: &ComponentState| -> BehaviorResult<()> {
        use leptos::leptos_dom::helpers::document;

        let Some(el) = document().get_element_by_id(id) else { return Ok(()); };
        if el.get_attribute("data-icon-toggle-attached").as_deref() == Some("1") { return Ok(()); }
        el.set_attribute("data-icon-toggle-attached", "1").ok();

        let Ok(toggle) = el.dyn_into::<HtmlElement>() else { return Ok(()); };
        let toggle_clone = toggle.clone();

        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let is_active = toggle_clone.has_attribute("data-active");
            if is_active {
                toggle_clone.remove_attribute("data-active").ok();
                toggle_clone.set_attribute("aria-pressed", "false").ok();
            } else {
                toggle_clone.set_attribute("data-active", "true").ok();
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
