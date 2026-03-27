#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;
#[cfg(feature = "hydrate")]
use web_sys::{Event, HtmlInputElement};
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-command", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {

        if root.get_attribute("data-rs-command-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-command-attached", "1").ok();

        let Ok(Some(input_el)) = root.query_selector("[data-rs-command-input]") else { return Ok(()); };

        let root_clone = root.clone();
        let cb = Closure::wrap(Box::new(move |_: Event| {
            let search = root_clone
                .query_selector("[data-rs-command-input]")
                .ok()
                .flatten()
                .and_then(|el| el.dyn_into::<HtmlInputElement>().ok())
                .map(|el| el.value().to_lowercase())
                .unwrap_or_default();

            if let Ok(items) = root_clone.query_selector_all("[data-rs-command-item]") {
                for i in 0..items.length() {
                    if let Some(item) = items.item(i).and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok()) {
                        let text = item.text_content().unwrap_or_default().to_lowercase();
                        let matches = search.is_empty() || text.contains(&search);
                        item.style().set_property("display", if matches { "flex" } else { "none" }).ok();
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        input_el.add_event_listener_with_callback("input", cb.as_ref().unchecked_ref()).ok();
        cb.forget();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
