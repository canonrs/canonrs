//! Radio Init

use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::{state, query};
use canonrs_interactions_core::runtime::listeners;
use crate::runtime::focus;

pub fn init(root: Element) {
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

    if let Some(input) = query::first(&root, "[data-rs-radio-input]") {
        listeners::listen(&uid, &input, "change", {
            let root_c = root.clone();
            move |e: web_sys::Event| {
                let input = match e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                    Some(i) => i, None => return,
                };
                let name    = input.name();
                let checked = input.checked();
                if checked {
                    state::remove_state(&root_c, "unselected"); state::add_state(&root_c, "selected");
                } else {
                    state::remove_state(&root_c, "selected"); state::add_state(&root_c, "unselected");
                }
                if !name.is_empty() {
                    let selector = format!("[data-rs-radio-input][name='{}']", name);
                    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                        if let Ok(nodes) = doc.query_selector_all(&selector) {
                            for i in 0..nodes.length() {
                                if let Some(node) = nodes.item(i) {
                                    if let Ok(other) = node.dyn_into::<web_sys::HtmlInputElement>() {
                                        if !other.is_same_node(Some(input.as_ref())) {
                                            if let Ok(Some(label)) = other.closest("[data-rs-radio]") {
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
            }
        });
    }

    let in_group = root.closest("[data-rs-radio-group]").ok().flatten().is_some();
    if !in_group {
        listeners::listen(&uid, &root, "mouseenter", {
            let r = root.clone();
            move |_: web_sys::Event| {
                if r.get_attribute("data-rs-disabled").as_deref() == Some("true") { return; }
                state::add_state(&r, "hover");
            }
        });
        listeners::listen(&uid, &root, "mouseleave", {
            let r = root.clone();
            move |_: web_sys::Event| { state::remove_state(&r, "hover"); }
        });
    }

    listeners::listen(&uid, &root, "pointerdown", {
        let r = root.clone();
        move |_: web_sys::Event| {
            if r.get_attribute("data-rs-disabled").as_deref() == Some("true") { return; }
            state::add_state(&r, "active");
        }
    });
    listeners::listen(&uid, &root, "pointerup", {
        let r = root.clone();
        move |_: web_sys::Event| { state::remove_state(&r, "active"); }
    });

    focus::init_focus(&root);
}
