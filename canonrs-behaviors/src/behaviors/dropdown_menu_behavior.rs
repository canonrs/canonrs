#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_shared::{BehaviorResult, BehaviorError};
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::{MouseEvent, HtmlElement};
#[cfg(feature = "hydrate")]
use leptos::prelude::Set;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-dropdown-menu", Box::new(|id: &str, state: &ComponentState| -> BehaviorResult<()> {
        use leptos::leptos_dom::helpers::document;

        let Some(dropdown) = document().get_element_by_id(id) else {
            return Err(BehaviorError::ElementNotFound { selector: id.into() });
        };
        if dropdown.get_attribute("data-dropdown-attached").as_deref() == Some("1") { return Ok(()); }
        dropdown.set_attribute("data-dropdown-attached", "1").ok();

        let open_signal = state.open;
        let dropdown_id = id.to_string();

        if let Some(trigger) = document().query_selector(&format!("[data-dropdown-menu-trigger='{}']", dropdown_id)).ok().flatten() {
            let dropdown_clone = dropdown.clone();
            let trigger_clone = trigger.clone();
            let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                let is_open = dropdown_clone.get_attribute("data-state").as_deref() == Some("open");
                open_signal.set(!is_open);
                dropdown_clone.set_attribute("data-state", if !is_open { "open" } else { "closed" }).ok();
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

        let dropdown_clone = dropdown.clone();
        let cb_outside = Closure::wrap(Box::new(move |e: MouseEvent| {
            if dropdown_clone.get_attribute("data-state").as_deref() != Some("open") { return; }
            if let Some(target) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) {
                if target.closest(&format!("#{}", dropdown_id)).ok().flatten().is_none() {
                    open_signal.set(false);
                    dropdown_clone.set_attribute("data-state", "closed").ok();
                }
            }
        }) as Box<dyn FnMut(_)>);
        document().add_event_listener_with_callback("click", cb_outside.as_ref().unchecked_ref()).ok();
        cb_outside.forget();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
