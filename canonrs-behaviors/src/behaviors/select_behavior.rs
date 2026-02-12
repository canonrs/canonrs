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
                        content_clone.set_attribute("hidden", "").ok();
                        trigger_clone.set_attribute("aria-expanded", "false").ok();
                    } else {
                        content_clone.set_attribute("data-state", "open").ok();
                        content_clone.remove_attribute("hidden").ok();
                        trigger_clone.set_attribute("aria-expanded", "true").ok();
                    }
                }) as Box<dyn FnMut(_)>);
                trigger_el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
                closure.forget();

                // Handle item selection
                let items = select_el.query_selector_all("[data-select-item]").ok();
                if let Some(items_list) = items {
                    for i in 0..items_list.length() {
                        if let Some(item) = items_list.get(i) {
                            let item_el: leptos::web_sys::Element = item.unchecked_into();
                            let select_clone = select_el.clone();
                            
                            let item_closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |e: leptos::web_sys::Event| {
                                e.stop_propagation();
                                let target = e.current_target().unwrap();
                                let current_el: &leptos::web_sys::Element = target.unchecked_ref();
                                let text = current_el.text_content().unwrap_or_default();

                                // Remove previous selection
                                if let Some(all_items) = select_clone.query_selector_all("[data-select-item]").ok() {
                                    for j in 0..all_items.length() {
                                        if let Some(other) = all_items.get(j) {
                                            let other_el: leptos::web_sys::Element = other.unchecked_into();
                                            other_el.set_attribute("aria-selected", "false").ok();
                                            other_el.set_attribute("data-state", "unselected").ok();
                                        }
                                    }
                                }

                                // Mark current as selected
                                current_el.set_attribute("aria-selected", "true").ok();
                                current_el.set_attribute("data-state", "selected").ok();

                                // Update trigger text
                                if let Some(trigger) = select_clone.query_selector("[data-select-trigger]").ok().flatten() {
                                    trigger.set_attribute("data-value-text", &text).ok();
                                    trigger.set_attribute("aria-expanded", "false").ok();
                                    
                                    // Update SelectValue text
                                    if let Some(value_el) = trigger.query_selector("[data-select-value]").ok().flatten() {
                                        value_el.set_text_content(Some(&text));
                                    }
                                }

                                // Close dropdown
                                if let Some(content) = select_clone.query_selector("[data-select-content]").ok().flatten() {
                                    content.set_attribute("data-state", "closed").ok();
                                    content.set_attribute("hidden", "").ok();
                                }
                            }) as Box<dyn FnMut(_)>);
                            
                            item_el.add_event_listener_with_callback("click", item_closure.as_ref().unchecked_ref()).ok();
                            item_closure.forget();
                        }
                    }
                }

                // Close on click outside
                let select_clone = select_el.clone();
                let outside_closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |e: leptos::web_sys::Event| {
                    if let Some(target) = e.target() {
                        let node: leptos::web_sys::Node = target.unchecked_into();
                        if !select_clone.contains(Some(&node)) {
                            if let Some(c) = select_clone.query_selector("[data-select-content]").ok().flatten() {
                                c.set_attribute("data-state", "closed").ok();
                                c.set_attribute("hidden", "").ok();
                            }
                            if let Some(t) = select_clone.query_selector("[data-select-trigger]").ok().flatten() {
                                t.set_attribute("aria-expanded", "false").ok();
                            }
                        }
                    }
                }) as Box<dyn FnMut(_)>);
                doc.add_event_listener_with_callback("click", outside_closure.as_ref().unchecked_ref()).ok();
                outside_closure.forget();
            }
        }
        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
