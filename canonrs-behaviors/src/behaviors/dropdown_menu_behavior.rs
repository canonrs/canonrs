#[cfg(feature = "hydrate")]
use super::*;
#[cfg(feature = "hydrate")]
use canonrs_shared::{BehaviorResult, BehaviorError};
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use leptos::web_sys::{window, MouseEvent};
#[cfg(feature = "hydrate")]
use leptos::prelude::Set;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-dropdown-menu", Box::new(|element_id, state| {
        let document = window().unwrap().document().unwrap();
        let dropdown = document
            .get_element_by_id(element_id)
            .ok_or_else(|| BehaviorError::ElementNotFound { selector: element_id.to_string() })?;

        let open_signal = state.open;

        // Find trigger by data-dropdown-menu-trigger attribute (like dialog does)
        let trigger_selector = format!("[data-dropdown-menu-trigger=\"{}\"]", element_id);
        if let Ok(Some(trigger)) = document.query_selector(&trigger_selector) {
            let dropdown_clone = dropdown.clone();
            let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                // Toggle by checking current state
                let current_state = dropdown_clone.get_attribute("data-state").unwrap_or_else(|| "closed".to_string());
                let is_open = current_state == "open";
                
                open_signal.set(!is_open);
                dropdown_clone.set_attribute("data-state", if !is_open { "open" } else { "closed" }).ok();
            }) as Box<dyn FnMut(MouseEvent)>);
            trigger.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).unwrap();
            cb.forget();
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
