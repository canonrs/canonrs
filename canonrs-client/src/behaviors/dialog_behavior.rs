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
use leptos::prelude::Set;

#[cfg(feature = "hydrate")]
fn get_focusable(container: &web_sys::Element) -> Vec<web_sys::Element> {
    let sel = "a[href],button:not([disabled]),input:not([disabled]),select:not([disabled]),textarea:not([disabled]),[tabindex]:not([tabindex=\"-1\"])";
    let mut out = vec![];
    if let Ok(list) = container.query_selector_all(sel) {
        for i in 0..list.length() {
            if let Some(n) = list.item(i) {
                if let Ok(el) = n.dyn_into::<web_sys::Element>() { out.push(el); }
            }
        }
    }
    out
}

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-dialog", Box::new(|root: &web_sys::Element, state: &ComponentState| -> BehaviorResult<()> {

        if root.get_attribute("data-rs-dialog-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-dialog-attached", "1").ok();

        let open_signal = state.open;
        let doc = web_sys::window().unwrap().document().unwrap();

        if let Ok(Some(trigger)) = root.query_selector("[data-rs-trigger]") {
            let root_clone = root.clone();
            let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                let is_open = root_clone.get_attribute("data-rs-state").as_deref() == Some("open");
                if is_open {
                    root_clone.set_attribute("data-rs-state", "closed").ok();
                    open_signal.set(false);
                    if let Some(body) = web_sys::window().unwrap().document().unwrap().body() {
                        body.style().remove_property("overflow").ok();
                    }
                } else {
                    root_clone.set_attribute("data-rs-state", "open").ok();
                    open_signal.set(true);
                    if let Some(body) = web_sys::window().unwrap().document().unwrap().body() {
                        body.style().set_property("overflow", "hidden").ok();
                    }
                    let focusable = get_focusable(&root_clone);
                    if let Some(first) = focusable.first() {
                        if let Ok(el) = first.clone().dyn_into::<HtmlElement>() { el.focus().ok(); }
                    }
                }
            }) as Box<dyn FnMut(_)>);
            trigger.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        if let Ok(Some(overlay)) = root.query_selector("[data-rs-overlay]") {
            let root_clone = root.clone();
            let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                root_clone.set_attribute("data-rs-state", "closed").ok();
                open_signal.set(false);
                if let Some(body) = web_sys::window().unwrap().document().unwrap().body() {
                    body.style().remove_property("overflow").ok();
                }
            }) as Box<dyn FnMut(_)>);
            overlay.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        if let Ok(Some(close_btn)) = root.query_selector("[data-rs-close]") {
            let root_clone = root.clone();
            let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                root_clone.set_attribute("data-rs-state", "closed").ok();
                open_signal.set(false);
                if let Some(body) = web_sys::window().unwrap().document().unwrap().body() {
                    body.style().remove_property("overflow").ok();
                }
            }) as Box<dyn FnMut(_)>);
            close_btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        let root_esc = root.clone();
        let cb_esc = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            if e.key() == "Escape" && root_esc.get_attribute("data-rs-state").as_deref() == Some("open") {
                root_esc.set_attribute("data-rs-state", "closed").ok();
                open_signal.set(false);
                if let Some(body) = web_sys::window().unwrap().document().unwrap().body() {
                    body.style().remove_property("overflow").ok();
                }
            }
        }) as Box<dyn FnMut(_)>);
        doc.add_event_listener_with_callback("keydown", cb_esc.as_ref().unchecked_ref()).ok();
        cb_esc.forget();

        let root_trap = root.clone();
        let cb_trap = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            if root_trap.get_attribute("data-rs-state").as_deref() != Some("open") || e.key() != "Tab" { return; }
            let focusable = get_focusable(&root_trap);
            if focusable.is_empty() { return; }
            let first = focusable.first().unwrap().clone();
            let last = focusable.last().unwrap().clone();
            let active = web_sys::window().unwrap().document().unwrap().active_element();
            if e.shift_key() {
                if active.as_ref() == Some(&first) {
                    e.prevent_default();
                    if let Ok(el) = last.dyn_into::<HtmlElement>() { el.focus().ok(); }
                }
            } else {
                if active.as_ref() == Some(&last) {
                    e.prevent_default();
                    if let Ok(el) = first.dyn_into::<HtmlElement>() { el.focus().ok(); }
                }
            }
        }) as Box<dyn FnMut(_)>);
        doc.add_event_listener_with_callback("keydown", cb_trap.as_ref().unchecked_ref()).ok();
        cb_trap.forget();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
