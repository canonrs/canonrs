//! OverlayBehavior - Auto-discovery via [data-overlay]
//! Configuração via data-attributes no HTML:
//!   data-overlay                        — marca o overlay
//!   data-overlay-trigger="id"           — id do elemento trigger
//!   data-overlay-trigger-type="click|hover"
//!   data-overlay-close-on-outside="true|false"
//! Estado vive no Signal. DOM reage via Effect.
//! POSICIONAMENTO é calculado via CSS/data-attributes (não aqui).

use leptos::prelude::*;
use leptos::leptos_dom::helpers::window;
use web_sys::{EventTarget, Element, Document};
use wasm_bindgen::{prelude::*, JsCast};
use super::behavior_registry::{register_behavior, ComponentState};
use canonrs_shared::BehaviorResult;

#[derive(Clone, Copy)]
pub enum TriggerType {
    Click,
    Hover,
}

fn parse_trigger_type(s: &str) -> TriggerType {
    match s {
        "click" => TriggerType::Click,
        "hover" => TriggerType::Hover,
        _ => TriggerType::Click,
    }
}

pub fn init_overlay_behavior() {
    register_behavior("data-overlay", Box::new(|id: &str, state: &ComponentState| -> BehaviorResult<()> {
        let overlay_id = id.to_string();
        let open_signal = state.open;

        let doc: Document = window().document().expect("document");
        let element: Element = doc.get_element_by_id(&overlay_id).expect("overlay element");

        let trigger_id = element.get_attribute("data-overlay-trigger").unwrap_or_default();
        let trigger_type = parse_trigger_type(
            &element.get_attribute("data-overlay-trigger-type").unwrap_or("click".to_string())
        );
        let close_on_outside = element
            .get_attribute("data-overlay-close-on-outside")
            .unwrap_or("false".to_string()) == "true";

        // Effect reativo: Signal → DOM
        let ov_id = overlay_id.clone();
        Effect::new(move |_| {
            let d: Document = window().document().expect("document");
            if let Some(overlay_elem) = d.get_element_by_id(&ov_id) {
                let state = if open_signal.get() { "open" } else { "closed" };
                let _ = overlay_elem.set_attribute("data-state", state);
            }
        });

        // Trigger events
        if let Ok(Some(trigger)) = doc.query_selector(&format!("#{}", trigger_id)) {
            match trigger_type {
                TriggerType::Click => {
                    let closure = Closure::wrap(Box::new(move |_: leptos::web_sys::Event| {
                        open_signal.update(|v| *v = !*v);
                    }) as Box<dyn FnMut(_)>);
                    let target: &EventTarget = trigger.as_ref();
                    let _ = target.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
                    closure.forget();
                }
                TriggerType::Hover => {
                    let open_sig = open_signal;
                    let enter = Closure::wrap(Box::new(move |_: leptos::web_sys::Event| {
                        open_sig.set(true);
                    }) as Box<dyn FnMut(_)>);
                    let leave = Closure::wrap(Box::new(move |_: leptos::web_sys::Event| {
                        open_signal.set(false);
                    }) as Box<dyn FnMut(_)>);
                    let target: &EventTarget = trigger.as_ref();
                    let _ = target.add_event_listener_with_callback("mouseenter", enter.as_ref().unchecked_ref());
                    let _ = target.add_event_listener_with_callback("mouseleave", leave.as_ref().unchecked_ref());
                    enter.forget();
                    leave.forget();
                }
            }
        }

        // Close on outside click
        if close_on_outside {
            let body: web_sys::HtmlElement = doc.body().expect("body");
            let closure = Closure::wrap(Box::new(move |e: leptos::web_sys::Event| {
                let target_et: EventTarget = match e.target() {
                    Some(t) => t,
                    None => return,
                };
                let target_el: Element = match target_et.dyn_into() {
                    Ok(el) => el,
                    Err(_) => return,
                };
                let d: Document = window().document().expect("document");
                if let Some(overlay_elem) = d.get_element_by_id(&overlay_id) {
                    if !overlay_elem.contains(Some(&target_el)) {
                        open_signal.set(false);
                    }
                }
            }) as Box<dyn FnMut(_)>);
            let target: &EventTarget = body.as_ref();
            let _ = target.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
            closure.forget();
        }

        Ok(())
    }));
}
