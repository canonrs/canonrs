#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_core::{BehaviorResult, BehaviorError};
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
    register_behavior("data-command", Box::new(|root: &web_sys::Element, _state: &ComponentState| {
        let command = root;
        let document = web_sys::window().unwrap().document().unwrap();
        let input_selector = format!("[data-command-input]");
        
        if let Ok(Some(input)) = document.query_selector(&input_selector) {
            let document_clone = document.clone();
                        
            let cb = Closure::wrap(Box::new(move |_: Event| {
                if let Some(input_el) = root.query_selector("[data-command-input]")
                    .ok()
                    .flatten()
                    .and_then(|el| el.dyn_into::<HtmlInputElement>().ok()) 
                {
                    let search = input_el.value().to_lowercase();
                    
                    let items_selector = format!("[data-command-item]");
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
