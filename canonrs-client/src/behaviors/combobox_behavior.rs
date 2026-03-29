#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::{HtmlElement, MouseEvent, Element};

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-combobox", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {

        if root.get_attribute("data-rs-combobox-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-combobox-attached", "1").ok();

        let Ok(Some(trigger_el)) = root.query_selector("[data-rs-combobox-trigger]") else { return Ok(()); };
        let Ok(trigger) = trigger_el.dyn_into::<HtmlElement>() else { return Ok(()); };

        // toggle open/close
        let root_toggle = root.clone();
        let cb_toggle = Closure::wrap(Box::new(move |_: MouseEvent| {
            let is_open = root_toggle.get_attribute("data-rs-state").as_deref() == Some("open");
            if is_open {
                root_toggle.set_attribute("data-rs-state", "closed").ok();
                root_toggle.set_attribute("aria-expanded", "false").ok();
            } else {
                // posicionar lista via floating-x/y
                if let Ok(Some(list_el)) = root_toggle.query_selector("[data-rs-combobox-list]") {
                    if let Ok(list) = list_el.dyn_into::<HtmlElement>() {
                        let rect = root_toggle.get_bounding_client_rect();
                        let x = rect.left();
                        let y = rect.bottom() + 4.0;
                        let w = rect.width();
                        list.style().set_property("left", &format!("{}px", x)).ok();
                        list.style().set_property("top", &format!("{}px", y)).ok();
                        list.style().set_property("width", &format!("{}px", w)).ok();
                    }
                }
                root_toggle.set_attribute("data-rs-state", "open").ok();
                root_toggle.set_attribute("aria-expanded", "true").ok();
            }
        }) as Box<dyn FnMut(_)>);
        trigger.add_event_listener_with_callback("click", cb_toggle.as_ref().unchecked_ref()).ok();
        cb_toggle.forget();

        // fechar ao clicar fora
        let root_outside = root.clone();
        let cb_outside = Closure::wrap(Box::new(move |e: MouseEvent| {
            if let Some(target) = e.target() {
                if let Some(el) = target.dyn_ref::<Element>() {
                    let inside_list    = el.closest("[data-rs-combobox-list]").ok().flatten().is_some();
                    let inside_trigger = el.closest("[data-rs-combobox-trigger]").ok().flatten().is_some();
                    let inside_root    = el.closest("[data-rs-combobox]").ok().flatten().is_some();
                    if !inside_list && !inside_trigger && !inside_root {
                        root_outside.set_attribute("data-rs-state", "closed").ok();
                        root_outside.set_attribute("aria-expanded", "false").ok();
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        web_sys::window().unwrap().document().unwrap()
            .add_event_listener_with_callback("click", cb_outside.as_ref().unchecked_ref()).ok();
        cb_outside.forget();

        // ESC fecha
        let root_esc = root.clone();
        let cb_esc = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            if e.key() == "Escape" && root_esc.get_attribute("data-rs-state").as_deref() == Some("open") {
                root_esc.set_attribute("data-rs-state", "closed").ok();
                root_esc.set_attribute("aria-expanded", "false").ok();
            }
        }) as Box<dyn FnMut(_)>);
        web_sys::window().unwrap()
            .add_event_listener_with_callback("keydown", cb_esc.as_ref().unchecked_ref()).ok();
        cb_esc.forget();

        // item selection + rs-change
        if let Ok(items) = root.query_selector_all("[data-rs-combobox-item]") {
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
                    let value = target.get_attribute("data-rs-value").unwrap_or_default();
                    let text  = target.text_content().unwrap_or_default();

                    // clear all
                    if let Ok(all) = root_item.query_selector_all("[data-rs-combobox-item]") {
                        for j in 0..all.length() {
                            if let Some(n) = all.item(j) {
                                if let Ok(el) = n.dyn_into::<web_sys::Element>() {
                                    el.set_attribute("data-rs-state", "unselected").ok();
                                    el.set_attribute("aria-selected", "false").ok();
                                }
                            }
                        }
                    }
                    target.set_attribute("data-rs-state", "selected").ok();
                    target.set_attribute("aria-selected", "true").ok();

                    // update trigger display
                    if let Ok(Some(trigger)) = root_item.query_selector("[data-rs-combobox-trigger]") {
                        trigger.set_text_content(Some(&text));
                    }

                    root_item.set_attribute("data-rs-value", &value).ok();
                    root_item.set_attribute("data-rs-state", "closed").ok();
                    root_item.set_attribute("aria-expanded", "false").ok();

                    // dispatch rs-change
                    if let Ok(event) = web_sys::CustomEvent::new("rs-change") {
                        root_item.dispatch_event(&event).ok();
                    }
                }) as Box<dyn FnMut(_)>);
                item.add_event_listener_with_callback("click", cb_item.as_ref().unchecked_ref()).ok();
                cb_item.forget();
            }
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
