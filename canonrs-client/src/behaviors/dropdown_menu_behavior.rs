#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_core::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use leptos::prelude::Set;
#[cfg(feature = "hydrate")]
use web_sys::{MouseEvent, HtmlElement};

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-dropdown-menu", Box::new(|root: &web_sys::Element, state: &ComponentState| -> BehaviorResult<()> {
        if root.get_attribute("data-rs-dropdown-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-dropdown-attached", "1").ok();

        let open_signal = state.open;
        
        if let Ok(Some(trigger)) = root.query_selector("[data-rs-trigger]") {
            let dropdown_clone = root.clone();
            let trigger_clone = trigger.clone();
            let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                let is_open = dropdown_clone.get_attribute("data-rs-state").as_deref() == Some("open");
                open_signal.set(!is_open);
                dropdown_clone.set_attribute("data-rs-state", if !is_open { "open" } else { "closed" }).ok();
                if !is_open {
                    if let Some(content) = dropdown_clone.query_selector("[data-dropdown-menu-content]").ok().flatten() {
                        if let (Ok(c), Some(t)) = (content.dyn_into::<HtmlElement>(), trigger_clone.dyn_ref::<HtmlElement>()) {
                            let rect = t.get_bounding_client_rect();
                            c.style().set_property("left", &format!("{}px", rect.left())).ok();
                            c.style().set_property("top", &format!("{}px", rect.bottom() + 4.0)).ok();
                        }
                    }
                }
            }) as Box<dyn FnMut(_)>);
            trigger.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        let dropdown_clone = root.clone();
        let cb_outside = Closure::wrap(Box::new(move |e: MouseEvent| {
            if dropdown_clone.get_attribute("data-rs-state").as_deref() != Some("open") { return; }
            if let Some(target) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) {
                if target.closest("[data-rs-dropdown-menu]").ok().flatten().is_none() {
                    open_signal.set(false);
                    dropdown_clone.set_attribute("data-rs-state", "closed").ok();
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
