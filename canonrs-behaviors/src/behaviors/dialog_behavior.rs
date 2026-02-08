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
    register_behavior("data-dialog", Box::new(|element_id, state| {
        let document = window().unwrap().document().unwrap();
        let dialog = document
            .get_element_by_id(element_id)
            .ok_or_else(|| BehaviorError::ElementNotFound { selector: element_id.to_string() })?;

        let open_signal = state.open;

        // Find trigger by data-dialog-trigger attribute
        let trigger_selector = format!("[data-dialog-trigger=\"{}\"]", element_id);
        if let Ok(Some(trigger)) = document.query_selector(&trigger_selector) {
            let dialog_clone = dialog.clone();
            let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                open_signal.set(true);
                dialog_clone.set_attribute("data-state", "open").ok();
            }) as Box<dyn FnMut(MouseEvent)>);
            trigger.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).unwrap();
            cb.forget();
        }

        // Find close by data-dialog-close attribute
        let close_selector = format!("[data-dialog-close=\"{}\"]", element_id);
        if let Ok(Some(close)) = document.query_selector(&close_selector) {
            let dialog_clone = dialog.clone();
            let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                open_signal.set(false);
                dialog_clone.set_attribute("data-state", "closed").ok();
            }) as Box<dyn FnMut(MouseEvent)>);
            close.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).unwrap();
            cb.forget();
        }

        // Close on backdrop click
        if let Ok(Some(backdrop)) = document.query_selector(&format!("#{} [data-dialog-backdrop]", element_id)) {
            let dialog_clone = dialog.clone();
            let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                open_signal.set(false);
                dialog_clone.set_attribute("data-state", "closed").ok();
            }) as Box<dyn FnMut(MouseEvent)>);
            backdrop.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).unwrap();
            cb.forget();
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
