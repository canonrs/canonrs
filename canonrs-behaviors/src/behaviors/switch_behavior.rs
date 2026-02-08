#[cfg(feature = "hydrate")]
use leptos::leptos_dom::helpers::document;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-switch", Box::new(|element_id, _state| -> BehaviorResult<()> {
        if let Some(switch_el) = document().get_element_by_id(element_id) {
            let switch_clone = switch_el.clone();
            
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: leptos::web_sys::Event| {
                let is_checked = switch_clone.get_attribute("data-checked").unwrap_or_default() == "true";
                
                if is_checked {
                    switch_clone.set_attribute("data-checked", "false").ok();
                } else {
                    switch_clone.set_attribute("data-checked", "true").ok();
                }
            }) as Box<dyn FnMut(_)>);
            
            switch_el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
            closure.forget();
        }
        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
