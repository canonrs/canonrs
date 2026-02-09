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
    register_behavior("data-modal", Box::new(|element_id, state| {
        let document = window().unwrap().document().unwrap();
        let modal = document.get_element_by_id(element_id)
            .ok_or_else(|| BehaviorError::ElementNotFound { selector: element_id.to_string() })?;

        let open_signal = state.open;
        let trigger_selector = format!("[data-modal-trigger=\"{}\"]", element_id);

        if let Ok(Some(trigger)) = document.query_selector(&trigger_selector) {
            let modal_clone = modal.clone();
            let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                let current = modal_clone.get_attribute("data-state").unwrap_or_else(|| "closed".to_string());
                let is_open = current == "open";
                open_signal.set(!is_open);
                modal_clone.set_attribute("data-state", if !is_open { "open" } else { "closed" }).ok();
            }) as Box<dyn FnMut(MouseEvent)>);
            trigger.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).unwrap();
            cb.forget();

            if let Ok(Some(overlay)) = document.query_selector(&format!("#{} [data-modal-overlay]", element_id)) {
                let modal_clone = modal.clone();
                let cb_close = Closure::wrap(Box::new(move |_: MouseEvent| {
                    open_signal.set(false);
                    modal_clone.set_attribute("data-state", "closed").ok();
                }) as Box<dyn FnMut(MouseEvent)>);
                overlay.add_event_listener_with_callback("click", cb_close.as_ref().unchecked_ref()).unwrap();
                cb_close.forget();
            }
        }
        Ok(())
    }));
}
