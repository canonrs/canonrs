#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::HtmlElement;
#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-navigation-menu", Box::new(|id: &str, _state: &ComponentState| -> BehaviorResult<()> {
        use leptos::leptos_dom::helpers::document;

        let Some(root) = document().get_element_by_id(id) else { return Ok(()); };
        if root.get_attribute("data-navigation-menu-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-navigation-menu-attached", "1").ok();

        let triggers = root.query_selector_all("[data-nav-trigger]")
            .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "query triggers".into() })?;

        for i in 0..triggers.length() {
            let node = match triggers.item(i) { Some(n) => n, None => continue };
            let trigger = match node.dyn_into::<HtmlElement>() { Ok(el) => el, Err(_) => continue };

            if trigger.get_attribute("data-nav-initialized").is_some() { continue; }
            trigger.set_attribute("data-nav-initialized", "1").ok();

            let trigger_id = trigger.get_attribute("data-nav-trigger").unwrap_or_default();
            let doc = document();

            let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
                let Some(content) = doc.query_selector(&format!("[data-nav-content='{}']", trigger_id)).ok().flatten() else { return; };
                let Ok(content_el) = content.dyn_into::<HtmlElement>() else { return; };
                if content_el.has_attribute("hidden") {
                    content_el.remove_attribute("hidden").ok();
                    content_el.set_attribute("aria-hidden", "false").ok();
                } else {
                    content_el.set_attribute("hidden", "").ok();
                    content_el.set_attribute("aria-hidden", "true").ok();
                }
            }) as Box<dyn FnMut(_)>);

            trigger.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
            closure.forget();
        }
        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}

#[cfg(feature = "hydrate")]
pub fn init_navigation_menu() { register(); }
#[cfg(not(feature = "hydrate"))]
pub fn init_navigation_menu() {}
