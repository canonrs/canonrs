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
    register_behavior("data-collapsible", Box::new(|id: &str, _state: &ComponentState| -> BehaviorResult<()> {
        if let Some(root) = document().get_element_by_id(id) {
            if let Ok(Some(trigger)) = root.query_selector("[data-collapsible-trigger]") {
                if let Ok(Some(content)) = root.query_selector("[data-collapsible-content]") {
                    let root_clone = root.clone();
                    let content_clone = content.clone();
                    
                    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: leptos::web_sys::Event| {
                        let is_open = root_clone.get_attribute("data-state").unwrap_or_default() != "open";
                        
                        if is_open {
                            root_clone.set_attribute("data-state", "open").ok();
                            content_clone.remove_attribute("hidden").ok();
                            content_clone.set_attribute("data-state", "open").ok();
                        } else {
                            root_clone.set_attribute("data-state", "closed").ok();
                            content_clone.set_attribute("hidden", "").ok();
                            content_clone.set_attribute("data-state", "closed").ok();
                        }
                    }) as Box<dyn FnMut(_)>);
                    
                    trigger.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
                    closure.forget();
                }
            }
        }
        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
