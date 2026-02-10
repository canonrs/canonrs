#[cfg(feature = "hydrate")]
use super::*;
#[cfg(feature = "hydrate")]
use canonrs_shared::{BehaviorResult, BehaviorError};
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use leptos::web_sys::{window, Event, HtmlInputElement};
#[cfg(feature = "hydrate")]
use leptos::prelude::*;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-command", Box::new(|element_id, _state| {
        let document = window().unwrap().document().unwrap();
        let command = document.get_element_by_id(element_id)
            .ok_or_else(|| BehaviorError::ElementNotFound { selector: element_id.to_string() })?;

        let input_selector = format!("#{} [data-command-input]", element_id);
        
        if let Ok(Some(input)) = document.query_selector(&input_selector) {
            let document_clone = document.clone();
            let element_id_clone = element_id.to_string();
            
            let cb = Closure::wrap(Box::new(move |_: Event| {
                if let Some(input_el) = document_clone.query_selector(&format!("#{} [data-command-input]", element_id_clone))
                    .ok()
                    .flatten()
                    .and_then(|el| el.dyn_into::<HtmlInputElement>().ok()) 
                {
                    let search = input_el.value().to_lowercase();
                    
                    let items_selector = format!("#{} [data-command-item]", element_id_clone);
                    if let Ok(items) = document_clone.query_selector_all(&items_selector) {
                        for i in 0..items.length() {
                            if let Some(item) = items.get(i).and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok()) {
                                let text = item.text_content().unwrap_or_default().to_lowercase();
                                let matches = search.is_empty() || text.contains(&search);
                                
                                let style = item.style();
                                style.set_property("display", if matches { "flex" } else { "none" }).ok();
                            }
                        }
                    }
                }
            }) as Box<dyn FnMut(Event)>);
            
            input.add_event_listener_with_callback("input", cb.as_ref().unchecked_ref()).unwrap();
            cb.forget();
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
