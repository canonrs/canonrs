#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::{MouseEvent, HtmlElement};

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-context-menu", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {

        if root.get_attribute("data-rs-context-menu-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-context-menu-attached", "1").ok();

        let doc = web_sys::window().unwrap().document().unwrap();

        if let Ok(Some(trigger)) = root.query_selector("[data-rs-context-menu-trigger]") {
            let root_clone = root.clone();
            let cb_open = Closure::wrap(Box::new(move |e: MouseEvent| {
                e.prevent_default();
                if let Ok(Some(content)) = root_clone.query_selector("[data-rs-context-menu-content]") {
                    if let Ok(c) = content.dyn_into::<HtmlElement>() {
                        c.style().set_property("--context-menu-x", &format!("{}px", e.client_x())).ok();
                        c.style().set_property("--context-menu-y", &format!("{}px", e.client_y())).ok();
                    }
                }
                root_clone.set_attribute("data-rs-state", "open").ok();
            }) as Box<dyn FnMut(_)>);
            trigger.add_event_listener_with_callback("contextmenu", cb_open.as_ref().unchecked_ref()).ok();
            cb_open.forget();
        }

        let root_close = root.clone();
        let cb_close = Closure::wrap(Box::new(move |_: MouseEvent| {
            if root_close.get_attribute("data-rs-state").as_deref() != Some("open") { return; }
            root_close.set_attribute("data-rs-state", "closed").ok();
        }) as Box<dyn FnMut(_)>);
        doc.add_event_listener_with_callback("click", cb_close.as_ref().unchecked_ref()).ok();
        cb_close.forget();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
