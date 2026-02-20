#[cfg(feature = "hydrate")]
use leptos::leptos_dom::helpers::document;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use wasm_bindgen::closure::Closure;
#[cfg(feature = "hydrate")]
use web_sys::HtmlElement;
#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-collapsible", Box::new(|id: &str, _state: &ComponentState| -> BehaviorResult<()> {
        let Some(root) = document().get_element_by_id(id) else { return Ok(()); };

        if root.get_attribute("data-collapsible-attached").as_deref() == Some("1") {
            return Ok(());
        }
        root.set_attribute("data-collapsible-attached", "1").ok();

        let Ok(Some(trigger_el)) = root.query_selector("[data-collapsible-trigger]") else { return Ok(()); };
        let Ok(Some(content_el)) = root.query_selector("[data-collapsible-content]") else { return Ok(()); };

        let Ok(trigger) = trigger_el.dyn_into::<HtmlElement>() else { return Ok(()); };
        let Ok(content) = content_el.dyn_into::<HtmlElement>() else { return Ok(()); };

        let root_clone = root.clone();
        let content_clone = content.clone();
        let trigger_clone = trigger.clone();

        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let is_open = root_clone.get_attribute("data-state").as_deref() == Some("open");

            if is_open {
                root_clone.set_attribute("data-state", "closed").ok();
                content_clone.set_attribute("hidden", "").ok();
                content_clone.set_attribute("data-state", "closed").ok();
                content_clone.set_attribute("aria-hidden", "true").ok();
                trigger_clone.set_attribute("aria-expanded", "false").ok();
            } else {
                root_clone.set_attribute("data-state", "open").ok();
                content_clone.remove_attribute("hidden").ok();
                content_clone.set_attribute("data-state", "open").ok();
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
