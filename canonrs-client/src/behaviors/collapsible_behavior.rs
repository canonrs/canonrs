#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use wasm_bindgen::closure::Closure;
#[cfg(feature = "hydrate")]
use web_sys::HtmlElement;
#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_core::BehaviorResult;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-collapsible", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {

        if root.get_attribute("data-rs-collapsible-attached").as_deref() == Some("1") {
            return Ok(());
        }
        root.set_attribute("data-rs-collapsible-attached", "1").ok();

        let Ok(Some(trigger_el)) = root.query_selector("[data-rs-collapsible-trigger]") else { return Ok(()); };
        let Ok(Some(content_el)) = root.query_selector("[data-rs-collapsible-content]") else { return Ok(()); };

        let Ok(trigger) = trigger_el.dyn_into::<HtmlElement>() else { return Ok(()); };
        let Ok(content) = content_el.dyn_into::<HtmlElement>() else { return Ok(()); };

        let root_clone = root.clone();
        let content_clone = content.clone();
        let trigger_clone = trigger.clone();

        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let is_open = root_clone.get_attribute("data-rs-state").as_deref() == Some("open");

            if is_open {
                root_clone.set_attribute("data-rs-state", "closed").ok();
                content_clone.set_attribute("data-rs-state", "closed").ok();
                content_clone.set_attribute("aria-hidden", "true").ok();
                trigger_clone.set_attribute("aria-expanded", "false").ok();
            } else {
                root_clone.set_attribute("data-rs-state", "open").ok();
                content_clone.set_attribute("data-rs-state", "open").ok();
                content_clone.set_attribute("aria-hidden", "false").ok();
                trigger_clone.set_attribute("aria-expanded", "true").ok();
            }
        }) as Box<dyn FnMut(_)>);

        trigger.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
        closure.forget();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
