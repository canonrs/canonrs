//! Markdown Toolbar Behavior
#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::HtmlElement;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-md-toolbar", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {

        if root.get_attribute("data-rs-md-toolbar-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-md-toolbar-attached", "1").ok();

        let buttons = root.query_selector_all("[data-rs-md-toolbar-item]")
            .map_err(|_| crate::BehaviorError::JsError { message: "query buttons".into() })?;

        for i in 0..buttons.length() {
            let node = match buttons.item(i) { Some(n) => n, None => continue };
            let btn = match node.dyn_into::<HtmlElement>() { Ok(el) => el, Err(_) => continue };

            let action = match btn.get_attribute("data-action") { Some(a) => a, None => continue };
            let target_id = match btn.get_attribute("data-target") { Some(t) => t, None => continue };
            let btn_clone = btn.clone();

            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                let doc = web_sys::window().unwrap().document().unwrap();
                let target = match doc.get_element_by_id(&target_id) { Some(t) => t, None => return };
                match action.as_str() {
                    "toggle-toc" => {
                        let current = target.get_attribute("data-hide-toc").unwrap_or_else(|| "false".to_string());
                        let new_val = if current == "true" { "false" } else { "true" };
                        target.set_attribute("data-hide-toc", new_val).ok();
                        btn_clone.set_attribute("data-active", if new_val == "false" { "true" } else { "false" }).ok();
                    }
                    "toggle-line-numbers" => {
                        let current = target.get_attribute("data-show-line-numbers").unwrap_or_else(|| "false".to_string());
                        let new_val = if current == "true" { "false" } else { "true" };
                        target.set_attribute("data-show-line-numbers", new_val).ok();
                        btn_clone.set_attribute("data-active", new_val).ok();
                    }
                    _ => {}
                }
            }) as Box<dyn FnMut(_)>);

            btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                .map_err(|_| crate::BehaviorError::JsError { message: "toolbar listener failed".into() })?;
            closure.forget();
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
