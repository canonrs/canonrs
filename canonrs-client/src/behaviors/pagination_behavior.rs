#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::HtmlElement;
#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-pagination", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {
        if root.get_attribute("data-rs-pagination-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-pagination-attached", "1").ok();

        let buttons = root.query_selector_all("[data-rs-pagination-link], [data-rs-pagination-previous], [data-rs-pagination-next]")
            .map_err(|_| crate::BehaviorError::JsError { message: "query buttons".into() })?;

        for i in 0..buttons.length() {
            let node = match buttons.item(i) { Some(n) => n, None => continue };
            let btn = match node.dyn_into::<HtmlElement>() { Ok(el) => el, Err(_) => continue };

            let closure = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
                if let Some(target) = e.current_target().and_then(|t| t.dyn_into::<HtmlElement>().ok()) {
                    if target.get_attribute("data-rs-state").as_deref() == Some("disabled") {
                        e.prevent_default();
                    }
                }
            }) as Box<dyn FnMut(_)>);

            btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
            closure.forget();
        }
        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
