#[cfg(feature = "hydrate")]
use super::*;
#[cfg(feature = "hydrate")]
use canonrs_core::{BehaviorResult, BehaviorError};
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use leptos::web_sys::{window, MouseEvent, HtmlElement};

#[cfg(feature = "hydrate")]
fn activate_tab(tabs: &web_sys::Element, value: &str) {
    if let Ok(triggers) = tabs.query_selector_all("[data-tabs-trigger]") {
        for i in 0..triggers.length() {
            let Some(node) = triggers.item(i) else { continue };
            let Ok(el) = node.dyn_into::<web_sys::Element>() else { continue };
            let v = el.get_attribute("data-tabs-value").unwrap_or_default();
            if v == value {
                el.set_attribute("data-state", "active").ok();
                el.set_attribute("aria-selected", "true").ok();
            } else {
                el.set_attribute("data-state", "inactive").ok();
                el.set_attribute("aria-selected", "false").ok();
            }
        }
    }
    if let Ok(contents) = tabs.query_selector_all("[data-tabs-content]") {
        for i in 0..contents.length() {
            let Some(node) = contents.item(i) else { continue };
            let Ok(el) = node.dyn_into::<web_sys::Element>() else { continue };
            let v = el.get_attribute("data-value").unwrap_or_default();
            if v == value {
                el.set_attribute("data-state", "active").ok();
            } else {
                el.set_attribute("data-state", "inactive").ok();
            }
        }
    }
}

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-tabs", Box::new(|element_id, _state| -> BehaviorResult<()> {
        let document = window().unwrap().document().unwrap();
        let tabs = document.get_element_by_id(element_id)
            .ok_or_else(|| BehaviorError::ElementNotFound { selector: element_id.to_string() })?;

        if tabs.get_attribute("data-tabs-attached").as_deref() == Some("1") { return Ok(()); }
        tabs.set_attribute("data-tabs-attached", "1").ok();

        let default_val = tabs.get_attribute("data-default").unwrap_or_default();
        if !default_val.is_empty() {
            activate_tab(&tabs, &default_val);
        }

        let triggers = tabs.query_selector_all("[data-tabs-trigger]")
            .map_err(|_| BehaviorError::JsError { message: "query triggers".into() })?;

        for i in 0..triggers.length() {
            let Some(node) = triggers.item(i) else { continue };
            let Ok(btn) = node.dyn_into::<HtmlElement>() else { continue };

            let tabs_id = element_id.to_string();
            let cb = Closure::wrap(Box::new(move |e: MouseEvent| {
                let doc = window().unwrap().document().unwrap();
                let target = e.current_target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok());
                let Some(target) = target else { return };
                let value = target.get_attribute("data-tabs-value").unwrap_or_default();
                if value.is_empty() { return; }
                let Some(tabs_el) = doc.get_element_by_id(&tabs_id) else { return };
                activate_tab(&tabs_el, &value);
            }) as Box<dyn FnMut(MouseEvent)>);

            btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
