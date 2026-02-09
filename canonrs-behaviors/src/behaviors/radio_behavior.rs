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
    register_behavior("data-radio-group", Box::new(|element_id, state| {
        let document = window().unwrap().document().unwrap();
        let radio_group = document.get_element_by_id(element_id)
            .ok_or_else(|| BehaviorError::ElementNotFound { selector: element_id.to_string() })?;

        let item_selector = format!("#{} [data-radio-item]", element_id);
        
        if let Ok(items) = document.query_selector_all(&item_selector) {
            for i in 0..items.length() {
                if let Some(node) = items.get(i) {
                    if let Some(item) = node.dyn_ref::<Element>() {
                        let document_clone = document.clone();
                        let element_id_clone = element_id.to_string();
                        let item_clone = item.clone();
                        
                        let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                            if let Ok(all_items) = document_clone.query_selector_all(&format!("#{} [data-radio-item]", element_id_clone)) {
                                for j in 0..all_items.length() {
                                    if let Some(n) = all_items.get(j) {
                                        if let Some(i) = n.dyn_ref::<Element>() {
                                            i.set_attribute("data-state", "unchecked").ok();
                                        }
                                    }
                                }
                            }
                            item_clone.set_attribute("data-state", "checked").ok();
                        }) as Box<dyn FnMut(MouseEvent)>);
                        item.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).unwrap();
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
