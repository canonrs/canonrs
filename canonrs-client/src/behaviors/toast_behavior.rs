//! @canon-level: strict
//! Toast Behavior — fecha toast via data-rs-state

#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_core::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::{HtmlElement, MouseEvent};

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior(
        "data-rs-toast-close",
        Box::new(|el: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {
            let el_ref = el.clone();
            let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                if let Some(toast) = el_ref
                    .closest("[data-rs-toast]")
                    .ok()
                    .flatten()
                    .and_then(|n| n.dyn_into::<HtmlElement>().ok())
                {
                    let _ = toast.dataset().set("rsState", "closed");
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
