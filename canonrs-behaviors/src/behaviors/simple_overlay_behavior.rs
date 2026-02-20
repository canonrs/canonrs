#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::MouseEvent;
#[cfg(feature = "hydrate")]
use leptos::prelude::Set;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-simple-overlay", Box::new(|id: &str, state: &ComponentState| -> BehaviorResult<()> {
        use leptos::leptos_dom::helpers::document;

        let Some(overlay) = document().get_element_by_id(id) else { return Ok(()); };
        if overlay.get_attribute("data-simple-overlay-attached").as_deref() == Some("1") { return Ok(()); }
        overlay.set_attribute("data-simple-overlay-attached", "1").ok();

        let open_signal = state.open;
        let trigger_id = overlay.get_attribute("data-simple-overlay-trigger").unwrap_or_default();

        if !trigger_id.is_empty() {
            if let Some(trigger) = document().get_element_by_id(&trigger_id) {
                let overlay_clone = overlay.clone();
                let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                    let is_open = overlay_clone.get_attribute("data-state").as_deref() == Some("open");
                    if is_open {
                        open_signal.set(false);
                        overlay_clone.set_attribute("data-state", "closed").ok();
                    } else {
                        open_signal.set(true);
                        overlay_clone.set_attribute("data-state", "open").ok();
                    }
                }) as Box<dyn FnMut(_)>);
                trigger.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
                cb.forget();
            }
        }

        if let Some(close_btn) = overlay.query_selector("[data-close]").ok().flatten() {
            let overlay_clone = overlay.clone();
            let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                open_signal.set(false);
                overlay_clone.set_attribute("data-state", "closed").ok();
            }) as Box<dyn FnMut(_)>);
            close_btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
