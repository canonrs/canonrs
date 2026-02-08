#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;

#[cfg(feature = "hydrate")]
pub fn register() {
    use leptos::prelude::*;
    use leptos::leptos_dom::helpers::document;
    use wasm_bindgen::JsCast;

    register_behavior("data-combobox", Box::new(|element_id, state| -> BehaviorResult<()> {
        if let Some(el) = document().get_element_by_id(element_id) {
            if let Ok(Some(trigger)) = el.query_selector("[data-combobox-trigger]") {
                let state = state.clone();
                let id = element_id.to_string();

                let cb = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: leptos::web_sys::Event| {
                    let open = state.open.get();
                    state.open.set(!open);

                    if let Some(el) = document().get_element_by_id(&id) {
                        el.set_attribute("aria-expanded", if open { "false" } else { "true" }).ok();
                    }
                }) as Box<dyn FnMut(_)>);

                trigger.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
                cb.forget();
            }
        }
        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
