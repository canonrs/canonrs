#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_shared::{BehaviorResult, BehaviorError};
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::{MouseEvent, KeyboardEvent};
#[cfg(feature = "hydrate")]
use leptos::prelude::Set;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-drawer", Box::new(|id: &str, state: &ComponentState| -> BehaviorResult<()> {
        use leptos::leptos_dom::helpers::document;

        let Some(drawer) = document().get_element_by_id(id) else {
            return Err(BehaviorError::ElementNotFound { selector: id.into() });
        };
        if drawer.get_attribute("data-drawer-attached").as_deref() == Some("1") { return Ok(()); }
        drawer.set_attribute("data-drawer-attached", "1").ok();

        let open_signal = state.open;
        let drawer_id = id.to_string();

        if let Some(trigger) = document().query_selector(&format!("[data-drawer-trigger='{}']", drawer_id)).ok().flatten() {
            let drawer_clone = drawer.clone();
            let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                let is_open = drawer_clone.get_attribute("data-state").as_deref() == Some("open");
                open_signal.set(!is_open);
                drawer_clone.set_attribute("data-state", if !is_open { "open" } else { "closed" }).ok();
            }) as Box<dyn FnMut(_)>);
            trigger.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        if let Some(overlay) = drawer.query_selector("[data-drawer-overlay]").ok().flatten() {
            let drawer_clone = drawer.clone();
            let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                open_signal.set(false);
                drawer_clone.set_attribute("data-state", "closed").ok();
            }) as Box<dyn FnMut(_)>);
            overlay.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        // ESC key
        let drawer_clone = drawer.clone();
        let cb_esc = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            if e.key() == "Escape" && drawer_clone.get_attribute("data-state").as_deref() == Some("open") {
                open_signal.set(false);
                drawer_clone.set_attribute("data-state", "closed").ok();
            }
        }) as Box<dyn FnMut(_)>);
        document().add_event_listener_with_callback("keydown", cb_esc.as_ref().unchecked_ref()).ok();
        cb_esc.forget();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
