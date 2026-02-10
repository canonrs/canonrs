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
    register_behavior("data-tree", Box::new(|element_id, _state| {
        let document = window().unwrap().document().unwrap();
        let tree = document.get_element_by_id(element_id)
            .ok_or_else(|| BehaviorError::ElementNotFound { selector: element_id.to_string() })?;

        let toggle_selector = format!("#{} [data-tree-toggle]", element_id);

        if let Ok(toggles) = document.query_selector_all(&toggle_selector) {
            for i in 0..toggles.length() {
                if let Some(node) = toggles.get(i) {
                    if let Some(toggle) = node.dyn_ref::<Element>() {
                        let toggle_clone = toggle.clone();

                        let cb = Closure::wrap(Box::new(move |e: MouseEvent| {
                            e.stop_propagation();
                            
                            if let Ok(Some(tree_item)) = toggle_clone.closest("[data-tree-item]") {
                                let current_expanded = tree_item.get_attribute("data-expanded")
                                    .unwrap_or_else(|| "false".to_string());
                                let new_expanded = if current_expanded == "true" { "false" } else { "true" };
                                
                                tree_item.set_attribute("data-expanded", &new_expanded).ok();
                                tree_item.set_attribute("aria-expanded", &new_expanded).ok();
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
