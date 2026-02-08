//! Checkbox Behavior - Auto-discovery via [data-checkbox]

#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;

#[cfg(feature = "hydrate")]
pub fn register() {
    use leptos::prelude::*;

    register_behavior("data-checkbox", Box::new(|element_id, state| -> BehaviorResult<()> {
        use leptos::leptos_dom::helpers::document;
        use wasm_bindgen::JsCast;

        if let Some(checkbox) = document().get_element_by_id(element_id) {
            let state_clone = state.clone();
            let element_id = element_id.to_string();

            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: leptos::web_sys::Event| {
                let is_checked = state_clone.open.get();
                state_clone.open.update(|v| *v = !is_checked);

                if let Some(el) = document().get_element_by_id(&element_id) {
                    if is_checked {
                        el.remove_attribute("data-checked").ok();
                        el.set_attribute("data-state", "unchecked").ok();
                    } else {
                        el.set_attribute("data-checked", "true").ok();
                        el.set_attribute("data-state", "checked").ok();
                    }
                }
            }) as Box<dyn FnMut(_)>);

            checkbox.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
            closure.forget();
        }
        
        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
