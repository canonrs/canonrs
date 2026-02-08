//! Toggle Behavior - Auto-discovery via [data-toggle]
//! State lives in Signal. DOM reacts via Effect.

#[cfg(feature = "hydrate")]
use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use leptos::web_sys::{EventTarget, Window, Element};
#[cfg(feature = "hydrate")]
use wasm_bindgen::{prelude::*, JsCast};
#[cfg(feature = "hydrate")]
use super::behavior_registry::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-toggle", Box::new(|id: &str, state: &ComponentState| -> BehaviorResult<()> {
        let element_id = id.to_string();
        let open_signal = state.open;

        Effect::new(move |_| {
            let win: Window = leptos::leptos_dom::helpers::window();
            let doc = win.document().expect("Document not available");
            
            if let Some(el) = doc.get_element_by_id(&element_id) {
                let elem: Element = el;
                let value = if open_signal.get() { "on" } else { "off" };
                let _ = elem.set_attribute("data-state", value);
            }
        });

        let win: Window = leptos::leptos_dom::helpers::window();
        let doc = win.document().expect("Document not available");
        
        if let Some(el) = doc.get_element_by_id(id) {
            let elem: Element = el;
            let closure = Closure::wrap(Box::new(move |_: leptos::web_sys::Event| {
                open_signal.set(!open_signal.get());
            }) as Box<dyn FnMut(_)>);

            let target: &EventTarget = elem.as_ref();
            target.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
            closure.forget();
        }
        
        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
