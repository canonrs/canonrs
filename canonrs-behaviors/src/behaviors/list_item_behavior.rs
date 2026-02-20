#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::MouseEvent;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-list-item", Box::new(|id: &str, _state: &ComponentState| -> BehaviorResult<()> {
        use leptos::leptos_dom::helpers::document;

        let Some(item) = document().get_element_by_id(id) else { return Ok(()); };
        if item.get_attribute("data-list-item-attached").as_deref() == Some("1") { return Ok(()); }
        item.set_attribute("data-list-item-attached", "1").ok();

        let item_id = id.to_string();
        let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
            // Deselect siblings within parent list
            let Some(current) = document().get_element_by_id(&item_id) else { return; };
            let Some(parent) = current.parent_element() else { return; };
            if let Ok(siblings) = parent.query_selector_all("[data-list-item]") {
                for i in 0..siblings.length() {
                    if let Some(node) = siblings.item(i) {
                        if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                            el.set_attribute("data-selected", "false").ok();
                            el.set_attribute("aria-selected", "false").ok();
                        }
                    }
                }
            }
            current.set_attribute("data-selected", "true").ok();
            current.set_attribute("aria-selected", "true").ok();
        }) as Box<dyn FnMut(_)>);
        item.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
        cb.forget();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
