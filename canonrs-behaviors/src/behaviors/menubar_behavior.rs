#[cfg(feature = "hydrate")]
use super::*;
#[cfg(feature = "hydrate")]
use canonrs_shared::{BehaviorResult, BehaviorError};
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use leptos::web_sys::{window, MouseEvent, Element, HtmlElement};
#[cfg(feature = "hydrate")]
use leptos::prelude::Set;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-menubar", Box::new(|element_id, state| {
        let document = window().unwrap().document().unwrap();
        let menubar = document.get_element_by_id(element_id)
            .ok_or_else(|| BehaviorError::ElementNotFound { selector: element_id.to_string() })?;

        let trigger_selector = format!("#{} [data-menubar-trigger]", element_id);
        
        if let Ok(triggers) = document.query_selector_all(&trigger_selector) {
            for i in 0..triggers.length() {
                if let Some(node) = triggers.get(i) {
                    if let Some(trigger) = node.dyn_ref::<Element>() {
                        let document_clone = document.clone();
                        let element_id_clone = element_id.to_string();
                        let trigger_clone = trigger.clone();
                        
                        let cb_toggle = Closure::wrap(Box::new(move |_: MouseEvent| {
                            let target_menu = trigger_clone.get_attribute("data-target").unwrap_or_default();
                            let current_state = trigger_clone.get_attribute("data-state").unwrap_or_else(|| "closed".to_string());
                            let new_state = if current_state == "open" { "closed" } else { "open" };
                            
                            // Fechar todos
                            if let Ok(all_triggers) = document_clone.query_selector_all(&format!("#{} [data-menubar-trigger]", element_id_clone)) {
                                for j in 0..all_triggers.length() {
                                    if let Some(n) = all_triggers.get(j) {
                                        if let Some(t) = n.dyn_ref::<Element>() {
                                            t.set_attribute("data-state", "closed").ok();
                                        }
                                    }
                                }
                            }
                            
                            if let Ok(all_menus) = document_clone.query_selector_all(&format!("#{} [data-menubar-content]", element_id_clone)) {
                                for j in 0..all_menus.length() {
                                    if let Some(n) = all_menus.get(j) {
                                        if let Some(m) = n.dyn_ref::<Element>() {
                                            m.set_attribute("data-state", "closed").ok();
                                        }
                                    }
                                }
                            }
                            
                            // Abrir este
                            if new_state == "open" {
                                trigger_clone.set_attribute("data-state", "open").ok();
                                
                                let menu_selector = format!("#{} [data-menubar-content][data-menu='{}']", element_id_clone, target_menu);
                                if let Ok(Some(menu)) = document_clone.query_selector(&menu_selector) {
                                    menu.set_attribute("data-state", "open").ok();
                                    
                                    // Posicionar
                                    if let Some(html_menu) = menu.dyn_ref::<HtmlElement>() {
                                        let trigger_rect = trigger_clone.get_bounding_client_rect();
                                        let style = html_menu.style();
                                        style.set_property("left", &format!("{}px", trigger_rect.left())).ok();
                                        style.set_property("top", &format!("{}px", trigger_rect.bottom() + 4.0)).ok();
                                    }
                                }
                            }
                        }) as Box<dyn FnMut(MouseEvent)>);
                        trigger.add_event_listener_with_callback("click", cb_toggle.as_ref().unchecked_ref()).unwrap();
                        cb_toggle.forget();
                    }
                }
            }
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
