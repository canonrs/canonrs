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
    register_behavior("data-hover-card", Box::new(|element_id, state| {
        let document = window().unwrap().document().unwrap();
        let hover_card = document.get_element_by_id(element_id)
            .ok_or_else(|| BehaviorError::ElementNotFound { selector: element_id.to_string() })?;

        let open_signal = state.open;
        let trigger_selector = format!("[data-hover-card-trigger=\"{}\"]", element_id);
        
        if let Ok(Some(trigger)) = document.query_selector(&trigger_selector) {
            let hover_card_clone = hover_card.clone();
            let cb_enter = Closure::wrap(Box::new(move |_: MouseEvent| {
                open_signal.set(true);
                hover_card_clone.set_attribute("data-state", "open").ok();
            }) as Box<dyn FnMut(MouseEvent)>);
            trigger.add_event_listener_with_callback("mouseenter", cb_enter.as_ref().unchecked_ref()).unwrap();
            cb_enter.forget();

            let hover_card_clone = hover_card.clone();
            let cb_leave = Closure::wrap(Box::new(move |_: MouseEvent| {
                open_signal.set(false);
                hover_card_clone.set_attribute("data-state", "closed").ok();
            }) as Box<dyn FnMut(MouseEvent)>);
            trigger.add_event_listener_with_callback("mouseleave", cb_leave.as_ref().unchecked_ref()).unwrap();
            cb_leave.forget();
        }
        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
