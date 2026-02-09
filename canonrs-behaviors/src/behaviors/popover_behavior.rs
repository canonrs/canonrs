#[cfg(feature = "hydrate")]
use super::*;
#[cfg(feature = "hydrate")]
use canonrs_shared::{BehaviorResult, BehaviorError};
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use leptos::web_sys::{window, MouseEvent};
#[cfg(feature = "hydrate")]
use leptos::prelude::Set;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-popover", Box::new(|element_id, state| {
        let document = window().unwrap().document().unwrap();
        let popover = document.get_element_by_id(element_id)
            .ok_or_else(|| BehaviorError::ElementNotFound { selector: element_id.to_string() })?;

        let open_signal = state.open;
        let trigger_selector = format!("[data-popover-trigger=\"{}\"]", element_id);
        
        if let Ok(Some(trigger)) = document.query_selector(&trigger_selector) {
            let popover_clone = popover.clone();
            let cb_toggle = Closure::wrap(Box::new(move |_: MouseEvent| {
                let current_state = popover_clone.get_attribute("data-state").unwrap_or_else(|| "closed".to_string());
                let is_open = current_state == "open";
                
                open_signal.set(!is_open);
                popover_clone.set_attribute("data-state", if !is_open { "open" } else { "closed" }).ok();
            }) as Box<dyn FnMut(MouseEvent)>);
            trigger.add_event_listener_with_callback("click", cb_toggle.as_ref().unchecked_ref()).unwrap();
            cb_toggle.forget();

            // Fechar ao clicar fora
            let popover_clone = popover.clone();
            let cb_close = Closure::wrap(Box::new(move |e: MouseEvent| {
                if let Some(target) = e.target() {
                    if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                        if !popover_clone.contains(Some(&element)) {
                            open_signal.set(false);
                            popover_clone.set_attribute("data-state", "closed").ok();
                        }
                    }
                }
            }) as Box<dyn FnMut(MouseEvent)>);
            document.add_event_listener_with_callback("click", cb_close.as_ref().unchecked_ref()).unwrap();
            cb_close.forget();
        }
        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
