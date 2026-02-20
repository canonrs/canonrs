#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::{HtmlElement, MouseEvent, KeyboardEvent};
#[cfg(feature = "hydrate")]
use leptos::prelude::Set;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-modal", Box::new(|id: &str, state: &ComponentState| -> BehaviorResult<()> {
        use leptos::leptos_dom::helpers::document;

        let Some(modal) = document().get_element_by_id(id) else { return Ok(()); };
        if modal.get_attribute("data-modal-attached").as_deref() == Some("1") { return Ok(()); }
        modal.set_attribute("data-modal-attached", "1").ok();

        let open_signal = state.open;
        let modal_id = id.to_string();

        let close = {
            let modal_clone = modal.clone();
            let modal_id = modal_id.clone();
            move || {
                open_signal.set(false);
                modal_clone.set_attribute("data-state", "closed").ok();
                modal_clone.set_attribute("aria-hidden", "true").ok();
                if let Some(trigger) = document().query_selector(&format!("[data-modal-trigger='{}']", modal_id)).ok().flatten() {
                    if let Ok(el) = trigger.dyn_into::<HtmlElement>() { el.focus().ok(); }
                }
            }
        };

        // Trigger
        if let Some(trigger) = document().query_selector(&format!("[data-modal-trigger='{}']", modal_id)).ok().flatten() {
            let modal_clone = modal.clone();
            let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                let is_open = modal_clone.get_attribute("data-state").as_deref() == Some("open");
                if is_open {
                    open_signal.set(false);
                    modal_clone.set_attribute("data-state", "closed").ok();
                    modal_clone.set_attribute("aria-hidden", "true").ok();
                } else {
                    open_signal.set(true);
                    modal_clone.set_attribute("data-state", "open").ok();
                    modal_clone.set_attribute("aria-hidden", "false").ok();
                }
            }) as Box<dyn FnMut(_)>);
            trigger.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        // Overlay click
        if let Some(overlay) = modal.query_selector("[data-modal-overlay]").ok().flatten() {
            let close_clone = close.clone();
            let cb = Closure::wrap(Box::new(move |_: MouseEvent| { close_clone(); }) as Box<dyn FnMut(_)>);
            overlay.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        // Close button
        if let Some(btn) = modal.query_selector("[data-modal-close]").ok().flatten() {
            let close_clone = close.clone();
            let cb = Closure::wrap(Box::new(move |_: MouseEvent| { close_clone(); }) as Box<dyn FnMut(_)>);
            btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        // ESC key
        let modal_clone = modal.clone();
        let cb_esc = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            if e.key() == "Escape" && modal_clone.get_attribute("data-state").as_deref() == Some("open") {
                close();
            }
        }) as Box<dyn FnMut(_)>);
        document().add_event_listener_with_callback("keydown", cb_esc.as_ref().unchecked_ref()).ok();
        cb_esc.forget();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
