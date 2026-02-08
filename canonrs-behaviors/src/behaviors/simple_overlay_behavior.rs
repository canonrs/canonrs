use leptos::prelude::*;
use leptos::leptos_dom::helpers::window;
use web_sys::{EventTarget, Element, Document};
use wasm_bindgen::{prelude::*, JsCast};
use super::behavior_registry::{register_behavior, ComponentState};
use canonrs_shared::BehaviorResult;

pub fn init_simple_overlay_behavior() {
    register_behavior("data-simple-overlay", Box::new(|id: &str, state: &ComponentState| -> BehaviorResult<()> {
        let overlay_id = id.to_string();
        let open_signal = state.open;

        let doc: Document = window().document().expect("document");
        let element: Element = doc.get_element_by_id(&overlay_id).expect("overlay element");

        let trigger_id = element.get_attribute("data-simple-overlay-trigger").unwrap_or_default();

        // Effect reativo
        let ov_id = overlay_id.clone();
        Effect::new(move |_| {
            let d: Document = window().document().expect("document");
            if let Some(overlay_elem) = d.get_element_by_id(&ov_id) {
                let state = if open_signal.get() { "open" } else { "closed" };
                let _ = overlay_elem.set_attribute("data-state", state);
            }
        });

        // Trigger
        if let Ok(Some(trigger)) = doc.query_selector(&format!("#{}", trigger_id)) {
            let closure = Closure::wrap(Box::new(move |_: leptos::web_sys::Event| {
                open_signal.update(|v| *v = !*v);
            }) as Box<dyn FnMut(_)>);
            let target: &EventTarget = trigger.as_ref();
            let _ = target.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
            closure.forget();
        }

        // Close button
        if let Ok(Some(close_btn)) = doc.query_selector(&format!("#{} [data-close]", overlay_id)) {
            let closure = Closure::wrap(Box::new(move |_: leptos::web_sys::Event| {
                open_signal.set(false);
            }) as Box<dyn FnMut(_)>);
            let target: &EventTarget = close_btn.as_ref();
            let _ = target.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
            closure.forget();
        }

        Ok(())
    }));
}
