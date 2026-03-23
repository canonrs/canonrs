#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_core::BehaviorResult;
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
    register_behavior("data-popover", Box::new(|root: &web_sys::Element, state: &ComponentState| -> BehaviorResult<()> {
        let popover = root;
        if popover.get_attribute("data-popover-attached").as_deref() == Some("1") { return Ok(()); }
        popover.set_attribute("data-popover-attached", "1").ok();

        let open_signal = state.open;
        
        if let Ok(Some(trigger)) = root.query_selector("[data-rs-trigger]") {
            let popover_clone = root.clone();
            let trigger_clone = trigger.clone();
            let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                let is_open = popover_clone.get_attribute("data-rs-state").as_deref() == Some("open");
                if is_open {
                    open_signal.set(false);
                    popover_clone.set_attribute("data-rs-state", "closed").ok();
                } else {
                    open_signal.set(true);
                    popover_clone.set_attribute("data-rs-state", "open").ok();
                    if let Some(content) = popover_clone.query_selector("[data-popover-content]").ok().flatten() {
                        if let (Ok(c), Some(t)) = (content.dyn_into::<HtmlElement>(), trigger_clone.dyn_ref::<HtmlElement>()) {
                            let rect = t.get_bounding_client_rect();
                            c.style().set_property("left", &format!("{}px", rect.left())).ok();
                            c.style().set_property("top", &format!("{}px", rect.bottom() + 8.0)).ok();
                        }
                    }
                }
            }) as Box<dyn FnMut(_)>);
            trigger.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        let popover_clone = root.clone();
        let cb_outside = Closure::wrap(Box::new(move |e: MouseEvent| {
            if popover_clone.get_attribute("data-rs-state").as_deref() != Some("open") { return; }
            if let Some(target) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) {
                if target.closest("[data-rs-popover]").ok().flatten().is_none() {
                    open_signal.set(false);
                    popover_clone.set_attribute("data-rs-state", "closed").ok();
                }
            }
        }) as Box<dyn FnMut(_)>);
        web_sys::window().unwrap().document().unwrap().add_event_listener_with_callback("click", cb_outside.as_ref().unchecked_ref()).ok();
        cb_outside.forget();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
