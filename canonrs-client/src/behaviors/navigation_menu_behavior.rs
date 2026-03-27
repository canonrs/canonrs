//! NavigationMenu Behavior
//! CSS governa hover — JS governa keyboard + click para acessibilidade
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::{HtmlElement, KeyboardEvent};
#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;

#[cfg(feature = "hydrate")]
fn open_item(item: &web_sys::Element) {
    if let Ok(Some(trigger)) = item.query_selector("[data-rs-navigation-menu-trigger]") {
        trigger.set_attribute("data-rs-state", "open").ok();
        trigger.set_attribute("aria-expanded", "true").ok();
    }
    if let Ok(Some(content)) = item.query_selector("[data-rs-navigation-menu-content]") {
        content.set_attribute("data-rs-state", "open").ok();
    }
}

#[cfg(feature = "hydrate")]
fn close_item(item: &web_sys::Element) {
    if let Ok(Some(trigger)) = item.query_selector("[data-rs-navigation-menu-trigger]") {
        trigger.set_attribute("data-rs-state", "closed").ok();
        trigger.set_attribute("aria-expanded", "false").ok();
    }
    if let Ok(Some(content)) = item.query_selector("[data-rs-navigation-menu-content]") {
        content.set_attribute("data-rs-state", "closed").ok();
    }
}

#[cfg(feature = "hydrate")]
fn close_all(root: &web_sys::Element) {
    if let Ok(items) = root.query_selector_all("[data-rs-navigation-menu-item]") {
        for i in 0..items.length() {
            if let Some(node) = items.item(i) {
                if let Ok(item) = node.dyn_into::<web_sys::Element>() {
                    close_item(&item);
                }
            }
        }
    }
}

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-navigation-menu", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {
        if root.get_attribute("data-rs-navigation-menu-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-navigation-menu-attached", "1").ok();

        // keyboard navigation entre triggers
        let root_kb = root.clone();
        let cb_kb = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            let key = e.key();
            if !["ArrowLeft", "ArrowRight", "Escape", "Enter", " "].contains(&key.as_str()) { return; }

            let triggers: Vec<web_sys::Element> = {
                let Ok(list) = root_kb.query_selector_all("[data-rs-navigation-menu-trigger]") else { return };
                (0..list.length()).filter_map(|i| {
                    list.item(i)?.dyn_into::<web_sys::Element>().ok()
                }).collect()
            };

            let doc = web_sys::window().unwrap().document().unwrap();
            let active = doc.active_element();

            let current_idx = triggers.iter().position(|t| Some(t) == active.as_ref());

            match key.as_str() {
                "Escape" => close_all(&root_kb),
                "ArrowRight" => {
                    if let Some(idx) = current_idx {
                        let next = (idx + 1) % triggers.len();
                        if let Ok(el) = triggers[next].clone().dyn_into::<HtmlElement>() {
                            el.focus().ok();
                        }
                    }
                }
                "ArrowLeft" => {
                    if let Some(idx) = current_idx {
                        let prev = if idx == 0 { triggers.len() - 1 } else { idx - 1 };
                        if let Ok(el) = triggers[prev].clone().dyn_into::<HtmlElement>() {
                            el.focus().ok();
                        }
                    }
                }
                "Enter" | " " => {
                    if let Some(idx) = current_idx {
                        let trigger = &triggers[idx];
                        let is_open = trigger.get_attribute("data-rs-state").as_deref() == Some("open");
                        if let Ok(Some(item)) = trigger.closest("[data-rs-navigation-menu-item]") {
                            if is_open { close_item(&item); } else {
                                close_all(&root_kb);
                                open_item(&item);
                            }
                        }
                    }
                }
                _ => {}
            }
        }) as Box<dyn FnMut(_)>);
        root.add_event_listener_with_callback("keydown", cb_kb.as_ref().unchecked_ref()).ok();
        cb_kb.forget();

        // ESC global fecha tudo
        let root_esc = root.clone();
        let cb_esc = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            if e.key() == "Escape" { close_all(&root_esc); }
        }) as Box<dyn FnMut(_)>);
        web_sys::window().unwrap()
            .add_event_listener_with_callback("keydown", cb_esc.as_ref().unchecked_ref()).ok();
        cb_esc.forget();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
