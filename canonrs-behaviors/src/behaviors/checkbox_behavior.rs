#[cfg(feature = "hydrate")]
use super::*;
#[cfg(feature = "hydrate")]
use canonrs_shared::{BehaviorResult, BehaviorError};
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use leptos::web_sys::{window, MouseEvent, Element};
#[cfg(feature = "hydrate")]
use leptos::prelude::Set;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-checkbox", Box::new(|element_id, state| {
        let document = window().unwrap().document().unwrap();
        let checkbox = document.get_element_by_id(element_id)
            .ok_or_else(|| BehaviorError::ElementNotFound { selector: element_id.to_string() })?;

        let checked_signal = state.open; // Reutilizando open como checked
        
        let checkbox_clone = checkbox.clone();
        let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
            let current_checked = checkbox_clone.get_attribute("data-state").unwrap_or_else(|| "unchecked".to_string());
            let new_checked = current_checked != "checked";
            
            checked_signal.set(new_checked);
            checkbox_clone.set_attribute("data-state", if new_checked { "checked" } else { "unchecked" }).ok();
        }) as Box<dyn FnMut(MouseEvent)>);
        
        checkbox.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).unwrap();
        cb.forget();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
