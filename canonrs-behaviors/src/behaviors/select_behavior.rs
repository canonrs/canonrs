#[cfg(feature = "hydrate")]
use leptos::leptos_dom::helpers::document;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use super::register_behavior;
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-select", Box::new(|element_id, _state| -> BehaviorResult<()> {
        let doc = document();
        if let Some(select_el) = doc.get_element_by_id(element_id) {
            let trigger = select_el.query_selector("[data-select-trigger]").ok().flatten();
            let content = select_el.query_selector("[data-select-content]").ok().flatten();

            if let (Some(trigger_el), Some(content_el)) = (trigger, content) {
                // Toggle open/close on trigger click
                let content_clone = content_el.clone();
                let trigger_clone = trigger_el.clone();
                let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |e: leptos::web_sys::Event| {
                    e.stop_propagation();
                    let is_open = content_clone.get_attribute("data-state").unwrap_or_default() == "open";
                    if is_open {
                        content_clone.set_attribute("data-state", "closed").ok();
                        trigger_clone.set_attribute("aria-expanded", "false").ok();
                    } else {
                        content_clone.set_attribute("data-state", "open").ok();
                        trigger_clone.set_attribute("aria-expanded", "true").ok();
                    }
                }) as Box<dyn FnMut(_)>);
                trigger_el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
                closure.forget();

                // Close on click outside
                let select_clone = select_el.clone();
                let doc_clone = document();
                let outside_closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |e: leptos::web_sys::Event| {
                    if let Some(target) = e.target() {
                        let node: leptos::web_sys::Node = target.unchecked_into();
                        if !select_clone.contains(Some(&node)) {
                            if let Some(c) = select_clone.query_selector("[data-select-content]").ok().flatten() {
                                c.set_attribute("data-state", "closed").ok();
                            }
                            if let Some(t) = select_clone.query_selector("[data-select-trigger]").ok().flatten() {
                                t.set_attribute("aria-expanded", "false").ok();
                            }
                        }
                    }
                }) as Box<dyn FnMut(_)>);
                doc_clone.add_event_listener_with_callback("click", outside_closure.as_ref().unchecked_ref()).ok();
                outside_closure.forget();
            }
        }
        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
