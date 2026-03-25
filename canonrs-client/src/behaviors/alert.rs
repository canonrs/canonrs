//! @canon-level: strict
//! Alert Behavior — fecha alert via data-rs-state

#[cfg(feature = "hydrate")]
pub fn register() {
    use crate::behavior_registry::register_behavior;
    use wasm_bindgen::JsCast;
    use web_sys::HtmlElement;

    register_behavior("data-rs-alert-close", |el| {
        let el = el.clone();
        let closure = wasm_bindgen::closure::Closure::<dyn Fn(web_sys::MouseEvent)>::new(
            move |_e: web_sys::MouseEvent| {
                if let Some(alert) = el
                    .closest("[data-rs-alert]")
                    .ok()
                    .flatten()
                    .and_then(|n| n.dyn_into::<HtmlElement>().ok())
                {
                    let _ = alert.dataset().set("rsState", "closed");
                }
            },
        );
        el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .ok();
        closure.forget();
    });
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
