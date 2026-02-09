#[cfg(feature = "hydrate")]
use super::*;
#[cfg(feature = "hydrate")]
use canonrs_shared::{BehaviorResult, BehaviorError};
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use leptos::web_sys::{window, KeyboardEvent, Element};
#[cfg(feature = "hydrate")]
use leptos::prelude::Set;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-command", Box::new(|element_id, state| {
        let document = window().unwrap().document().unwrap();
        let command = document.get_element_by_id(element_id)
            .ok_or_else(|| BehaviorError::ElementNotFound { selector: element_id.to_string() })?;

        let input_selector = format!("#{} [data-command-input]", element_id);
        
        if let Ok(Some(input)) = document.query_selector(&input_selector) {
            let document_clone = document.clone();
            let element_id_clone = element_id.to_string();
            
            let cb_search = Closure::wrap(Box::new(move |e: KeyboardEvent| {
                if let Some(target) = e.target() {
                    if let Some(input_el) = target.dyn_ref::<web_sys::HtmlInputElement>() {
                        let search_value = input_el.value().to_lowercase();
                        
                        let item_selector = format!("#{} [data-command-item]", element_id_clone);
                        if let Ok(items) = document_clone.query_selector_all(&item_selector) {
                            for i in 0..items.length() {
                                if let Some(node) = items.get(i) {
                                    if let Some(item) = node.dyn_ref::<Element>() {
                                        let item_text = item.text_content().unwrap_or_default().to_lowercase();
                                        
                                        if search_value.is_empty() || item_text.contains(&search_value) {
                                            item.set_attribute("data-hidden", "false").ok();
                                        } else {
                                            item.set_attribute("data-hidden", "true").ok();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }) as Box<dyn FnMut(KeyboardEvent)>);
            input.add_event_listener_with_callback("input", cb_search.as_ref().unchecked_ref()).unwrap();
            cb_search.forget();
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
