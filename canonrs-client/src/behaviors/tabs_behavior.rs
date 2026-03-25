#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_core::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::{HtmlElement, MouseEvent, KeyboardEvent};

#[cfg(feature = "hydrate")]
fn activate_tab(root: &web_sys::Element, value: &str) {
    if let Ok(triggers) = root.query_selector_all("[data-rs-tabs-trigger]") {
        for i in 0..triggers.length() {
            let Some(node) = triggers.item(i) else { continue };
            let Ok(el) = node.dyn_into::<web_sys::Element>() else { continue };
            let v = el.get_attribute("data-rs-value").unwrap_or_default();
            let active = v == value;
            el.set_attribute("data-rs-state", if active { "active" } else { "inactive" }).ok();
            el.set_attribute("aria-selected", if active { "true" } else { "false" }).ok();
        }
    }
    if let Ok(contents) = root.query_selector_all("[data-rs-tabs-content]") {
        for i in 0..contents.length() {
            let Some(node) = contents.item(i) else { continue };
            let Ok(el) = node.dyn_into::<web_sys::Element>() else { continue };
            let v = el.get_attribute("data-rs-value").unwrap_or_default();
            let active = v == value;
            el.set_attribute("data-rs-state", if active { "active" } else { "inactive" }).ok();
            if active {
                el.remove_attribute("hidden").ok();
            } else {
                el.set_attribute("hidden", "").ok();
            }
        }
    }
}

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-tabs", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {

        if root.get_attribute("data-rs-tabs-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-tabs-attached", "1").ok();

        // detectar tab ativa via trigger com data-rs-state="active"
        let default_val = root
            .query_selector("[data-rs-tabs-trigger][data-rs-state=\"active\"]")
            .ok()
            .flatten()
            .and_then(|el| el.get_attribute("data-rs-value"))
            .unwrap_or_default();

        if !default_val.is_empty() {
            activate_tab(root, &default_val);
        }

        let triggers = root.query_selector_all("[data-rs-tabs-trigger]")
            .map_err(|_| canonrs_core::BehaviorError::JsError { message: "query triggers".into() })?;

        for i in 0..triggers.length() {
            let Some(node) = triggers.item(i) else { continue };
            let Ok(btn) = node.dyn_into::<HtmlElement>() else { continue };

            if btn.get_attribute("data-rs-tabs-trigger-initialized").is_some() { continue; }
            btn.set_attribute("data-rs-tabs-trigger-initialized", "1").ok();

            let root_click = root.clone();
            let cb = Closure::wrap(Box::new(move |e: MouseEvent| {
                let target = e.current_target().and_then(|t| t.dyn_into::<web_sys::Element>().ok());
                let Some(target) = target else { return };
                let value = target.get_attribute("data-rs-value").unwrap_or_default();
                if value.is_empty() { return; }
                activate_tab(&root_click, &value);
            }) as Box<dyn FnMut(_)>);
            btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();

            let root_key = root.clone();
            let btn_key = btn.clone();
            let key_cb = Closure::wrap(Box::new(move |e: KeyboardEvent| {
                match e.key().as_str() {
                    "Enter" | " " => { e.prevent_default(); btn_key.click(); }
                    "ArrowRight" | "ArrowLeft" => {
                        e.prevent_default();
                        let win = match web_sys::window() { Some(w) => w, None => return };
                        let doc = match win.document() { Some(d) => d, None => return };
                        if let Ok(all) = root_key.query_selector_all("[data-rs-tabs-trigger]") {
                            let len = all.length();
                            let mut cur: Option<u32> = None;
                            for j in 0..len {
                                if let Some(n) = all.item(j) {
                                    if let Ok(el) = n.dyn_into::<HtmlElement>() {
                                        if doc.active_element().map(|a| a == *el.as_ref()) == Some(true) {
                                            cur = Some(j); break;
                                        }
                                    }
                                }
                            }
                            if let Some(idx) = cur {
                                let next = if e.key() == "ArrowRight" { (idx + 1) % len } else { (idx + len - 1) % len };
                                if let Some(n) = all.item(next) {
                                    if let Ok(el) = n.dyn_into::<HtmlElement>() { el.focus().ok(); el.click(); }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }) as Box<dyn FnMut(_)>);
            btn.add_event_listener_with_callback("keydown", key_cb.as_ref().unchecked_ref()).ok();
            key_cb.forget();
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
