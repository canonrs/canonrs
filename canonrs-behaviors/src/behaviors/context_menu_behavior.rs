#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_shared::{BehaviorResult, BehaviorError};
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::{MouseEvent, HtmlElement};
#[cfg(feature = "hydrate")]
use leptos::prelude::Set;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-context-menu", Box::new(|id: &str, state: &ComponentState| -> BehaviorResult<()> {
        use leptos::leptos_dom::helpers::document;

        let Some(ctx) = document().get_element_by_id(id) else {
            return Err(BehaviorError::ElementNotFound { selector: id.into() });
        };
        if ctx.get_attribute("data-context-menu-attached").as_deref() == Some("1") { return Ok(()); }
        ctx.set_attribute("data-context-menu-attached", "1").ok();

        let open_signal = state.open;
        let ctx_id = id.to_string();

        if let Some(trigger) = document().query_selector(&format!("[data-context-menu-trigger='{}']", ctx_id)).ok().flatten() {
            let ctx_clone = ctx.clone();
            let cb_open = Closure::wrap(Box::new(move |e: MouseEvent| {
                e.prevent_default();
                if let Some(content) = ctx_clone.query_selector("[data-context-menu-content]").ok().flatten() {
                    if let Ok(c) = content.dyn_into::<HtmlElement>() {
                        c.style().set_property("left", &format!("{}px", e.client_x())).ok();
                        c.style().set_property("top", &format!("{}px", e.client_y())).ok();
                    }
                }
                open_signal.set(true);
                ctx_clone.set_attribute("data-state", "open").ok();
            }) as Box<dyn FnMut(_)>);
            trigger.add_event_listener_with_callback("contextmenu", cb_open.as_ref().unchecked_ref()).ok();
            cb_open.forget();
        }

        let ctx_clone = ctx.clone();
        let cb_close = Closure::wrap(Box::new(move |_: MouseEvent| {
            if ctx_clone.get_attribute("data-state").as_deref() != Some("open") { return; }
            open_signal.set(false);
            ctx_clone.set_attribute("data-state", "closed").ok();
        }) as Box<dyn FnMut(_)>);
        document().add_event_listener_with_callback("click", cb_close.as_ref().unchecked_ref()).ok();
        cb_close.forget();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
