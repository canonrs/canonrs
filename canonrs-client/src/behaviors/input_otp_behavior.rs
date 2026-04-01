#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::{HtmlInputElement, InputEvent, KeyboardEvent, MouseEvent};

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-input-otp", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {
        if root.get_attribute("data-rs-otp-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-otp-attached", "1").ok();

        let input_el: HtmlInputElement = match root.clone().dyn_into() {
            Ok(el) => el,
            Err(_) => return Ok(()),
        };

        // container é o div.input-otp-container (avô do input)
        let container = match root.parent_element().and_then(|p| p.parent_element()) {
            Some(c) => c,
            None => return Ok(()),
        };

        // clicar em qualquer slot foca o input
        let input_for_click = input_el.clone();
        let cb_click = Closure::wrap(Box::new(move |_: MouseEvent| {
            input_for_click.focus().ok();
        }) as Box<dyn FnMut(_)>);
        container.add_event_listener_with_callback("click", cb_click.as_ref().unchecked_ref()).ok();
        cb_click.forget();

        // atualizar slots ao digitar
        let container_for_input = container.clone();
        let input_for_input = input_el.clone();
        let update_slots = move || {
            let value = input_for_input.value();
            let slots: web_sys::NodeList = match container_for_input.query_selector_all("[data-rs-input-otp-slot]") {
                Ok(s) => s,
                Err(_) => return,
            };
            for i in 0..slots.length() {
                if let Some(node) = slots.item(i) {
                    if let Ok(slot) = node.dyn_into::<web_sys::Element>() {
                        let ch = value.chars().nth(i as usize)
                            .map(|c| c.to_string())
                            .unwrap_or_default();
                        // atualizar span interno
                        if let Ok(Some(inner)) = slot.query_selector("[data-rs-slot-inner]") {
                            inner.set_text_content(Some(&ch));
                        }
                        let is_active = i == value.len() as u32;
                        slot.set_attribute("data-rs-state", if is_active { "active" } else { "inactive" }).ok();
                    }
                }
            }
        };

        let update_slots_clone = update_slots.clone();
        let cb_input = Closure::wrap(Box::new(move |_: InputEvent| {
            update_slots_clone();
        }) as Box<dyn FnMut(_)>);
        input_el.add_event_listener_with_callback("input", cb_input.as_ref().unchecked_ref()).ok();
        cb_input.forget();

        // backspace
        let input_for_key = input_el.clone();
        let cb_key = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            if e.key() == "Backspace" {
                let value = input_for_key.value();
                if !value.is_empty() {
                    let new_val = value[..value.len()-1].to_string();
                    let _ = js_sys::Reflect::set(
                        &input_for_key,
                        &"value".into(),
                        &new_val.into(),
                    );
                    if let Ok(event) = web_sys::Event::new("input") {
                        input_for_key.dispatch_event(&event).ok();
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        input_el.add_event_listener_with_callback("keydown", cb_key.as_ref().unchecked_ref()).ok();
        cb_key.forget();

        // inicializar slots
        update_slots();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
