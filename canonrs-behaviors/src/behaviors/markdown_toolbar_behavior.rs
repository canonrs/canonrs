//! Markdown Toolbar Behavior
//! Handles toggle actions for TOC and line numbers

#[cfg(feature = "hydrate")]
use super::register_behavior;
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;
#[cfg(feature = "hydrate")]
use leptos::leptos_dom::helpers::document;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-toolbar-toggle", Box::new(|element_id, _state| {
        let Some(btn) = document().get_element_by_id(element_id) else {
            return Ok(());
        };

        let Some(action) = btn.get_attribute("data-action") else {
            return Ok(());
        };

        let Some(target_id) = btn.get_attribute("data-target") else {
            return Ok(());
        };

        let action_owned = action.clone();
        let target_id_owned = target_id.clone();
        let btn_clone = btn.clone();
        
        let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Some(target) = document().get_element_by_id(&target_id_owned) {
                match action_owned.as_str() {
                    "toggle-toc" => {
                        let current = target.get_attribute("data-hide-toc")
                            .unwrap_or_else(|| "false".to_string());
                        let new_val = if current == "true" { "false" } else { "true" };
                        let _ = target.set_attribute("data-hide-toc", &new_val);
                        let _ = btn_clone.set_attribute("data-active", if new_val == "false" { "true" } else { "false" });
                    },
                    "toggle-line-numbers" => {
                        let current = target.get_attribute("data-show-line-numbers")
                            .unwrap_or_else(|| "false".to_string());
                        let new_val = if current == "true" { "false" } else { "true" };
                        let _ = target.set_attribute("data-show-line-numbers", &new_val);
                        let _ = btn_clone.set_attribute("data-active", &new_val);
                    },
                    _ => {}
                }
            }
        }) as Box<dyn FnMut(_)>);

        btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .map_err(|_| canonrs_shared::BehaviorError::JsError { 
                message: "toolbar listener failed".into() 
            })?;
        
        closure.forget();
        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
