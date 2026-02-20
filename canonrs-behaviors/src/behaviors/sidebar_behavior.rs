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
    register_behavior("data-sidebar", Box::new(|id: &str, _state: &ComponentState| -> BehaviorResult<()> {
        use leptos::leptos_dom::helpers::document;

        let Some(root) = document().get_element_by_id(id) else { return Ok(()); };
        if root.get_attribute("data-sidebar-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-sidebar-attached", "1").ok();

        let toggles = root.query_selector_all("[data-sidebar-toggle]")
            .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "query toggles".into() })?;

        for i in 0..toggles.length() {
            let node = match toggles.item(i) { Some(n) => n, None => continue };
            let toggle = match node.dyn_into::<HtmlElement>() { Ok(el) => el, Err(_) => continue };

            let sidebar_id = id.to_string();
            let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
                let Some(sidebar) = document().get_element_by_id(&sidebar_id)
                    .and_then(|el| el.dyn_into::<HtmlElement>().ok()) else { return; };
                if sidebar.has_attribute("data-collapsed") {
                    sidebar.remove_attribute("data-collapsed").ok();
                    sidebar.set_attribute("aria-expanded", "true").ok();
                } else {
                    sidebar.set_attribute("data-collapsed", "true").ok();
                    sidebar.set_attribute("aria-expanded", "false").ok();
                }
            }) as Box<dyn FnMut(_)>);

            toggle.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
            closure.forget();
        }
        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}

#[cfg(feature = "hydrate")]
pub fn init_sidebar() { register(); }
#[cfg(not(feature = "hydrate"))]
pub fn init_sidebar() {}
