#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_shared::{BehaviorResult, BehaviorError};
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
    register_behavior("data-sheet", Box::new(|id: &str, state: &ComponentState| -> BehaviorResult<()> {
        use leptos::leptos_dom::helpers::document;

        let Some(sheet) = document().get_element_by_id(id) else {
            return Err(BehaviorError::ElementNotFound { selector: id.into() });
        };
        if sheet.get_attribute("data-sheet-attached").as_deref() == Some("1") { return Ok(()); }
        sheet.set_attribute("data-sheet-attached", "1").ok();

        let open_signal = state.open;
        let sheet_id = id.to_string();

        if let Some(trigger) = document().query_selector(&format!("[data-sheet-trigger='{}']", sheet_id)).ok().flatten() {
            let sheet_clone = sheet.clone();
            let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                let is_open = sheet_clone.get_attribute("data-state").as_deref() == Some("open");
                open_signal.set(!is_open);
                sheet_clone.set_attribute("data-state", if !is_open { "open" } else { "closed" }).ok();
            }) as Box<dyn FnMut(_)>);
            trigger.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        if let Some(overlay) = sheet.query_selector("[data-sheet-overlay]").ok().flatten() {
            let sheet_clone = sheet.clone();
            let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                open_signal.set(false);
                sheet_clone.set_attribute("data-state", "closed").ok();
            }) as Box<dyn FnMut(_)>);
            overlay.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
