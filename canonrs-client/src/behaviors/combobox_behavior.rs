#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_core::BehaviorResult;
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

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
