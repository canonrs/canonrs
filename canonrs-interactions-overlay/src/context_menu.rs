//! ContextMenu Interaction Engine
//! Core: dom/{lifecycle, state, query} + Overlay: stack

use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};
use canonrs_interactions_core::dom::{lifecycle, state, query};
use canonrs_interactions_core::runtime::listeners;

fn position_and_open(root: &Element, x: i32, y: i32) {
    if crate::runtime::stack::has_modal_open() { return; }
    let Ok(Some(content)) = root.query_selector("[data-rs-context-menu-content]") else { return };
    if let Ok(root_html) = root.clone().dyn_into::<HtmlElement>() {
        let _ = root_html.style().set_property("--context-menu-x", &format!("{}px", x));
        let _ = root_html.style().set_property("--context-menu-y", &format!("{}px", y));
    }
    state::open(&content);
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

    {
        listeners::listen(&uid, &root, "contextmenu", move |e: web_sys::Event| {
            let Some(me) = e.dyn_into::<web_sys::MouseEvent>().ok() else { return };
            me.prevent_default();
            let Some(current) = me.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            query::each("[data-rs-context-menu][data-rs-initialized='true']", |node| {
                if state::is_open(&node) { state::close(&node); }
            });
            position_and_open(&current, me.client_x(), me.client_y());
        });
    }
    {
        let uid2 = uid.clone();
        listeners::listen(&uid2, &root, "click", move |e: web_sys::Event| {
            let Some(me) = e.dyn_into::<web_sys::MouseEvent>().ok() else { return };
            let Some(current) = me.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(target)  = me.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if query::closest(&target, "[data-rs-context-menu-item]") {
                let disabled = target.get_attribute("aria-disabled").as_deref() == Some("true");
                if !disabled { state::close(&current); }
            }
        });
    }
    {
        let uid2 = uid.clone();
        listeners::listen_document(&uid2, "click", move |_: web_sys::Event| {
            query::each("[data-rs-context-menu][data-rs-initialized='true']", |node| {
                if state::is_open(&node) { state::close(&node); }
            });
        });
    }
    {
        let uid2 = uid.clone();
        listeners::listen_window(&uid2, "keydown", move |e: web_sys::Event| {
            let Some(ke) = e.dyn_into::<web_sys::KeyboardEvent>().ok() else { return };
            if ke.key() != "Escape" { return; }
            query::each("[data-rs-context-menu][data-rs-initialized='true']", |node| {
                if state::is_open(&node) { state::close(&node); }
            });
        });
    }
}
