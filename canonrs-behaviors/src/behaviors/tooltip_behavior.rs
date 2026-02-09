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
use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use canonrs_ui::primitives::floating::use_floating_position;
use canonrs_ui::primitives::floating::types::{FloatingConfig, Placement};

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-tooltip", Box::new(|element_id, state| {
        let document = window().unwrap().document().unwrap();
        let tooltip = document.get_element_by_id(element_id)
            .ok_or_else(|| BehaviorError::ElementNotFound { selector: element_id.to_string() })?;

        let open_signal = state.open;
        let trigger_selector = format!("[data-tooltip-trigger=\"{}\"]", element_id);
        
        if let Ok(Some(trigger)) = document.query_selector(&trigger_selector) {
            let trigger_id = format!("{}-trigger", element_id);
            trigger.set_id(&trigger_id);
            
            let config = FloatingConfig {
                placement: Placement::Top,
                offset: 8.0,
                flip: true,
            };
            use_floating_position(&trigger_id, element_id, config, open_signal.read_only());
            
            let tooltip_clone = tooltip.clone();
            let cb_enter = Closure::wrap(Box::new(move |_: MouseEvent| {
                open_signal.set(true);
                tooltip_clone.set_attribute("data-state", "open").ok();
            }) as Box<dyn FnMut(MouseEvent)>);
            trigger.add_event_listener_with_callback("mouseenter", cb_enter.as_ref().unchecked_ref()).unwrap();
            cb_enter.forget();

            let tooltip_clone = tooltip.clone();
            let cb_leave = Closure::wrap(Box::new(move |_: MouseEvent| {
                open_signal.set(false);
                tooltip_clone.set_attribute("data-state", "closed").ok();
            }) as Box<dyn FnMut(MouseEvent)>);
            trigger.add_event_listener_with_callback("mouseleave", cb_leave.as_ref().unchecked_ref()).unwrap();
            cb_leave.forget();
        }
        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
