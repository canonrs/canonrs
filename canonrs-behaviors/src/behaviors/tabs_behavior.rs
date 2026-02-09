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
    register_behavior("data-tabs", Box::new(|element_id, state| {
        let document = window().unwrap().document().unwrap();
        let tabs = document.get_element_by_id(element_id)
            .ok_or_else(|| BehaviorError::ElementNotFound { selector: element_id.to_string() })?;

        let trigger_selector = format!("#{} [data-tabs-trigger]", element_id);
        
        if let Ok(triggers) = document.query_selector_all(&trigger_selector) {
            for i in 0..triggers.length() {
                if let Some(node) = triggers.get(i) {
                    if let Some(trigger) = node.dyn_ref::<Element>() {
                        let document_clone = document.clone();
                        let element_id_clone = element_id.to_string();
                        let trigger_clone = trigger.clone();
                        
                        let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                            let target_value = trigger_clone.get_attribute("data-value").unwrap_or_default();
                            
                            if let Ok(all_triggers) = document_clone.query_selector_all(&format!("#{} [data-tabs-trigger]", element_id_clone)) {
                                for j in 0..all_triggers.length() {
                                    if let Some(n) = all_triggers.get(j) {
                                        if let Some(t) = n.dyn_ref::<Element>() {
                                            t.set_attribute("data-state", "inactive").ok();
                                        }
                                    }
                                }
                            }
                            
                            trigger_clone.set_attribute("data-state", "active").ok();
                            
                            if let Ok(all_panels) = document_clone.query_selector_all(&format!("#{} [data-tabs-content]", element_id_clone)) {
                                for j in 0..all_panels.length() {
                                    if let Some(n) = all_panels.get(j) {
                                        if let Some(p) = n.dyn_ref::<Element>() {
                                            p.set_attribute("data-state", "inactive").ok();
                                        }
                                    }
                                }
                            }
                            
                            let panel_selector = format!("#{} [data-tabs-content][data-value='{}']", element_id_clone, target_value);
                            if let Ok(Some(panel)) = document_clone.query_selector(&panel_selector) {
                                panel.set_attribute("data-state", "active").ok();
                            }
                        }) as Box<dyn FnMut(MouseEvent)>);
                        trigger.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).unwrap();
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
