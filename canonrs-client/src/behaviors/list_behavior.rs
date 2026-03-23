#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_core::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::HtmlElement;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-list", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {

        if root.get_attribute("data-rs-list-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-list-attached", "1").ok();

        let is_single = root.get_attribute("data-rs-selection").as_deref() != Some("multiple");

        let items = root.query_selector_all("[data-rs-list-item]")
            .map_err(|_| canonrs_core::BehaviorError::JsError { message: "query items".into() })?;

        for i in 0..items.length() {
            let node = match items.item(i) { Some(n) => n, None => continue };
            let item = match node.dyn_into::<HtmlElement>() { Ok(el) => el, Err(_) => continue };

            if item.has_attribute("data-rs-disabled") { continue; }

            let root_clone = root.clone();
            let item_clone = item.clone();

            let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
                let is_selected = item_clone.get_attribute("data-rs-state").as_deref() == Some("active");

                if is_single {
                    if let Ok(all) = root_clone.query_selector_all("[data-rs-list-item]") {
                        for j in 0..all.length() {
                            if let Some(n) = all.item(j) {
                                if let Ok(el) = n.dyn_into::<web_sys::Element>() {
                                    el.set_attribute("data-rs-state", "inactive").ok();
                                    el.set_attribute("aria-selected", "false").ok();
                                }
                            }
                        }
                    }
                    if !is_selected {
                        item_clone.set_attribute("data-rs-state", "active").ok();
                        item_clone.set_attribute("aria-selected", "true").ok();
                    }
                } else {
                    if is_selected {
                        item_clone.set_attribute("data-rs-state", "inactive").ok();
                        item_clone.set_attribute("aria-selected", "false").ok();
                    } else {
                        item_clone.set_attribute("data-rs-state", "active").ok();
                        item_clone.set_attribute("aria-selected", "true").ok();
                    }
                }
            }) as Box<dyn FnMut(_)>);

            item.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
            closure.forget();
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
