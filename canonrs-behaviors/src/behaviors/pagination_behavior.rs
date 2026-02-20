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
    register_behavior("data-pagination", Box::new(|id: &str, _state: &ComponentState| -> BehaviorResult<()> {
        use leptos::leptos_dom::helpers::document;

        let Some(root) = document().get_element_by_id(id) else { return Ok(()); };
        if root.get_attribute("data-pagination-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-pagination-attached", "1").ok();

        let buttons = root.query_selector_all("button, a[data-pagination-link], a[data-pagination-previous], a[data-pagination-next]")
            .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "query buttons".into() })?;

        for i in 0..buttons.length() {
            let node = match buttons.item(i) { Some(n) => n, None => continue };
            let btn = match node.dyn_into::<HtmlElement>() { Ok(el) => el, Err(_) => continue };

            let closure = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
                if let Some(target) = e.current_target().and_then(|t| t.dyn_into::<HtmlElement>().ok()) {
                    let _ = target.dispatch_event(&web_sys::Event::new("pagination:click").unwrap());
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

#[cfg(feature = "hydrate")]
pub fn init_pagination() { register(); }
#[cfg(not(feature = "hydrate"))]
pub fn init_pagination() {}
