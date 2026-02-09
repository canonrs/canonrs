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
    register_behavior("data-tree", Box::new(|element_id, state| {
        let document = window().unwrap().document().unwrap();
        let tree = document.get_element_by_id(element_id)
            .ok_or_else(|| BehaviorError::ElementNotFound { selector: element_id.to_string() })?;

        let toggle_selector = format!("#{} [data-tree-toggle]", element_id);
        
        if let Ok(toggles) = document.query_selector_all(&toggle_selector) {
            for i in 0..toggles.length() {
                if let Some(node) = toggles.get(i) {
                    if let Some(toggle) = node.dyn_ref::<Element>() {
                        let toggle_clone = toggle.clone();
                        
                        let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                            if let Some(parent) = toggle_clone.parent_element() {
                                let current_state = parent.get_attribute("data-state").unwrap_or_else(|| "closed".to_string());
                                let new_state = if current_state == "open" { "closed" } else { "open" };
                                parent.set_attribute("data-state", &new_state).ok();
                            }
                        }) as Box<dyn FnMut(MouseEvent)>);
                        toggle.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).unwrap();
                        cb.forget();
                    }
                }
            }
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
