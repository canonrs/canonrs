//! @canon-level: strict
//! Alert Behavior — fecha alert via data-rs-state

#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::{HtmlElement, MouseEvent};

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior(
        "data-rs-alert-close",
        Box::new(|el: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {
            let el_ref = el.clone();
            let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                if let Some(alert) = el_ref
                    .closest("[data-rs-alert]")
                    .ok()
                    .flatten()
                    .and_then(|n| n.dyn_into::<HtmlElement>().ok())
                {
                    let _ = alert.dataset().set("rsState", "closed");
                }
            }) as Box<dyn FnMut(MouseEvent)>);
            el.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
            Ok(())
        }),
    );
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
