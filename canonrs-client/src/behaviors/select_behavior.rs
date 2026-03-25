#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::MouseEvent;
#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_core::BehaviorResult;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-select", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {

        if root.get_attribute("data-rs-select-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-select-attached", "1").ok();

        let Ok(Some(trigger)) = root.query_selector("[data-rs-select-trigger]") else { return Ok(()); };
        let Ok(Some(content)) = root.query_selector("[data-rs-select-content]") else { return Ok(()); };

        // toggle open/close
        let content_toggle = content.clone();
        let trigger_toggle = trigger.clone();
        let root_toggle = root.clone();
        let cb_toggle = Closure::wrap(Box::new(move |e: MouseEvent| {
            e.stop_propagation();
            let is_open = root_toggle.get_attribute("data-rs-state").as_deref() == Some("open");
            if is_open {
                root_toggle.set_attribute("data-rs-state", "closed").ok();
                content_toggle.set_attribute("data-rs-state", "closed").ok();
                content_toggle.set_attribute("hidden", "").ok();
                trigger_toggle.set_attribute("aria-expanded", "false").ok();
            } else {
                // posicionar content
                if let Ok(trigger_html) = trigger_toggle.clone().dyn_into::<web_sys::HtmlElement>() {
                    if let Ok(content_html) = content_toggle.clone().dyn_into::<web_sys::HtmlElement>() {
                        let rect = trigger_html.get_bounding_client_rect();
                        content_html.style().set_property("left", &format!("{}px", rect.left())).ok();
                        content_html.style().set_property("top", &format!("{}px", rect.bottom() + 4.0)).ok();
                        content_html.style().set_property("min-width", &format!("{}px", rect.width())).ok();
                    }
                }
                root_toggle.set_attribute("data-rs-state", "open").ok();
                content_toggle.set_attribute("data-rs-state", "open").ok();
                content_toggle.remove_attribute("hidden").ok();
                trigger_toggle.set_attribute("aria-expanded", "true").ok();
            }
        }) as Box<dyn FnMut(_)>);
        trigger.add_event_listener_with_callback("click", cb_toggle.as_ref().unchecked_ref()).ok();
        cb_toggle.forget();

        // item selection
        if let Ok(items) = root.query_selector_all("[data-rs-select-item]") {
            for i in 0..items.length() {
                let Some(node) = items.item(i) else { continue };
                let Ok(item) = node.dyn_into::<web_sys::Element>() else { continue };

                let root_item = root.clone();
                let cb_item = Closure::wrap(Box::new(move |e: MouseEvent| {
                    e.stop_propagation();
                    let target = match e.current_target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) {
                        Some(el) => el,
                        None => return,
                    };
                    let text = target.text_content().unwrap_or_default();
                    let value = target.get_attribute("data-rs-value").unwrap_or_default();

                    // clear all selections
                    if let Ok(all) = root_item.query_selector_all("[data-rs-select-item]") {
                        for j in 0..all.length() {
                            if let Some(n) = all.item(j) {
                                if let Ok(el) = n.dyn_into::<web_sys::Element>() {
                                    el.set_attribute("data-rs-state", "unselected").ok();
                                    el.set_attribute("aria-selected", "false").ok();
                                }
                            }
                        }
                    }

                    // select current
                    target.set_attribute("data-rs-state", "selected").ok();
                    target.set_attribute("aria-selected", "true").ok();

                    // update trigger value display
                    if let Ok(Some(trigger)) = root_item.query_selector("[data-rs-select-trigger]") {
                        trigger.set_attribute("aria-expanded", "false").ok();
                        if let Ok(Some(value_el)) = trigger.query_selector("[data-rs-select-value]") {
                            value_el.set_text_content(Some(&text));
                        }
                    }

                    // update root value
                    root_item.set_attribute("data-rs-value", &value).ok();
                    root_item.set_attribute("data-rs-state", "closed").ok();

                    // close content
                    if let Ok(Some(content)) = root_item.query_selector("[data-rs-select-content]") {
                        content.set_attribute("data-rs-state", "closed").ok();
                        content.set_attribute("hidden", "").ok();
                    }
                }) as Box<dyn FnMut(_)>);
                item.add_event_listener_with_callback("click", cb_item.as_ref().unchecked_ref()).ok();
                cb_item.forget();
            }
        }

        // close outside
        let root_outside = root.clone();
        let cb_outside = Closure::wrap(Box::new(move |e: MouseEvent| {
            if let Some(target) = e.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok()) {
                if !root_outside.contains(Some(&target)) {
                    root_outside.set_attribute("data-rs-state", "closed").ok();
                    if let Ok(Some(c)) = root_outside.query_selector("[data-rs-select-content]") {
                        c.set_attribute("data-rs-state", "closed").ok();
                        c.set_attribute("hidden", "").ok();
                    }
                    if let Ok(Some(t)) = root_outside.query_selector("[data-rs-select-trigger]") {
                        t.set_attribute("aria-expanded", "false").ok();
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        web_sys::window().unwrap().document().unwrap()
            .add_event_listener_with_callback("click", cb_outside.as_ref().unchecked_ref()).ok();
        cb_outside.forget();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
