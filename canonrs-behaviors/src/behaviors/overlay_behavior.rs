#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::{HtmlElement, MouseEvent};
#[cfg(feature = "hydrate")]
use leptos::prelude::Set;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-overlay", Box::new(|id: &str, state: &ComponentState| -> BehaviorResult<()> {
        use leptos::leptos_dom::helpers::document;

        let Some(overlay) = document().get_element_by_id(id) else { return Ok(()); };
        if overlay.get_attribute("data-overlay-attached").as_deref() == Some("1") { return Ok(()); }
        overlay.set_attribute("data-overlay-attached", "1").ok();

        let open_signal = state.open;
        let trigger_id = overlay.get_attribute("data-overlay-trigger").unwrap_or_default();
        let trigger_type = overlay.get_attribute("data-overlay-trigger-type").unwrap_or_else(|| "click".into());
        let close_on_outside = overlay.get_attribute("data-overlay-close-on-outside").as_deref() == Some("true");
        let overlay_id = id.to_string();

        let open_overlay = {
            let overlay_clone = overlay.clone();
            move || {
                open_signal.set(true);
                overlay_clone.set_attribute("data-state", "open").ok();
                overlay_clone.set_attribute("aria-hidden", "false").ok();
            }
        };
        let close_overlay = {
            let overlay_clone = overlay.clone();
            move || {
                open_signal.set(false);
                overlay_clone.set_attribute("data-state", "closed").ok();
                overlay_clone.set_attribute("aria-hidden", "true").ok();
            }
        };

        if !trigger_id.is_empty() {
            if let Some(trigger) = document().get_element_by_id(&trigger_id) {
                match trigger_type.as_str() {
                    "hover" => {
                        let open_clone = open_overlay.clone();
                        let close_clone = close_overlay.clone();
                        let enter = Closure::wrap(Box::new(move |_: MouseEvent| { open_clone(); }) as Box<dyn FnMut(_)>);
                        let leave = Closure::wrap(Box::new(move |_: MouseEvent| { close_clone(); }) as Box<dyn FnMut(_)>);
                        trigger.add_event_listener_with_callback("mouseenter", enter.as_ref().unchecked_ref()).ok();
                        trigger.add_event_listener_with_callback("mouseleave", leave.as_ref().unchecked_ref()).ok();
                        enter.forget();
                        leave.forget();
                    }
                    _ => {
                        let overlay_clone = overlay.clone();
                        let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                            let is_open = overlay_clone.get_attribute("data-state").as_deref() == Some("open");
                            if is_open { close_overlay(); } else { open_overlay(); }
                        }) as Box<dyn FnMut(_)>);
                        trigger.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
                        cb.forget();
                    }
                }
            }
        }

        if close_on_outside {
            let overlay_clone = overlay.clone();
            let cb = Closure::wrap(Box::new(move |e: MouseEvent| {
                if overlay_clone.get_attribute("data-state").as_deref() != Some("open") { return; }
                if let Some(target) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) {
                    if target.closest(&format!("#{}", overlay_id)).ok().flatten().is_none() {
                        open_signal.set(false);
                        overlay_clone.set_attribute("data-state", "closed").ok();
                    }
                }
            }) as Box<dyn FnMut(_)>);
            document().add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
