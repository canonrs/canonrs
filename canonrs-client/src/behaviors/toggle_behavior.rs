//! Toggle Behavior - sincroniza data-rs-state + emite rs-change
#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::HtmlElement;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-toggle", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {

        if root.get_attribute("data-rs-toggle-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-toggle-attached", "1").ok();

        // inicializar data-rs-value com estado atual
        let initial = if root.get_attribute("data-rs-state").as_deref() == Some("on") { "on" } else { "off" };
        root.set_attribute("data-rs-value", initial).ok();

        let root_clone = root.clone();
        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let is_on = root_clone.get_attribute("data-rs-state").as_deref() == Some("on");
            let new_state = if is_on { "off" } else { "on" };
            root_clone.set_attribute("data-rs-state", new_state).ok();
            root_clone.set_attribute("data-rs-value", new_state).ok();

            // dispatch rs-change
            {
                let init = web_sys::CustomEventInit::new();
                init.set_bubbles(true);
                init.set_detail(&wasm_bindgen::JsValue::from_str(new_state));
                if let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict("rs-change", &init) {
                    root_clone.dispatch_event(&event).ok();
                }
            }
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
