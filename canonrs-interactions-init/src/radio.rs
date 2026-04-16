//! Radio Init — sync native checked state + focus + active

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state, query, focus};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    // sync estado nativo — ao mudar, sincronizar todos do mesmo name
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::Event)>::new(move |e: web_sys::Event| {
            let input = match e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                Some(i) => i,
                None => return,
            };
            let name = input.name();
            let checked = input.checked();

            // atualizar este radio
            if checked {
                state::remove_state(&root_cb, "unselected");
                state::add_state(&root_cb, "selected");
            } else {
                state::remove_state(&root_cb, "selected");
                state::add_state(&root_cb, "unselected");
            }

            // sincronizar todos os outros radios do mesmo name no documento
            if !name.is_empty() {
                let selector = format!("[data-rs-radio-input][name='{}']", name);
                if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                    if let Ok(nodes) = doc.query_selector_all(&selector) {
                        for i in 0..nodes.length() {
                            if let Some(node) = nodes.item(i) {
                                if let Ok(other_input) = node.dyn_into::<web_sys::HtmlInputElement>() {
                                    if !other_input.is_same_node(Some(input.as_ref())) {
                                        if let Ok(Some(label)) = other_input.closest("[data-rs-radio]") {
                                            state::remove_state(&label, "selected");
                                            state::add_state(&label, "unselected");
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
        if let Some(input) = query::first(&root, "[data-rs-radio-input]") {
            let _ = input.add_event_listener_with_callback("change", cb.as_ref().unchecked_ref());
        }
        cb.forget();
    }

    // hover — só se não estiver dentro de radio_group (que gerencia hover)
    {
        let r = root.clone();
        let in_group = root.closest("[data-rs-radio-group]").ok().flatten().is_some();
        if !in_group {
            let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
                if r.get_attribute("data-rs-disabled").as_deref() == Some("true") { return; }
                state::add_state(&r, "hover");
            });
            let _ = root.add_event_listener_with_callback("mouseenter", cb.as_ref().unchecked_ref());
            cb.forget();
        }
    }
    {
        let r = root.clone();
        let in_group = root.closest("[data-rs-radio-group]").ok().flatten().is_some();
        if !in_group {
            let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
                state::remove_state(&r, "hover");
            });
            let _ = root.add_event_listener_with_callback("mouseleave", cb.as_ref().unchecked_ref());
            cb.forget();
        }
    }

    // active (press)
    {
        let r = root.clone();
        let cb = Closure::<dyn Fn(web_sys::PointerEvent)>::new(move |_: web_sys::PointerEvent| {
            if r.get_attribute("data-rs-disabled").as_deref() == Some("true") { return; }
            state::add_state(&r, "active");
        });
        let _ = root.add_event_listener_with_callback("pointerdown", cb.as_ref().unchecked_ref());
        cb.forget();
    }
    {
        let r = root.clone();
        let cb = Closure::<dyn Fn(web_sys::PointerEvent)>::new(move |_: web_sys::PointerEvent| {
            state::remove_state(&r, "active");
        });
        let _ = root.add_event_listener_with_callback("pointerup", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // focus
    focus::init_focus(&root);
}
