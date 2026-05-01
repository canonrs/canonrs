//! ContextMenu Interaction Engine
//! Posicionamento via coordenadas do contextmenu event (x/y absolutos)

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};
use crate::runtime::{lifecycle, state, query};

fn position_and_open(root: &Element, x: i32, y: i32) {
    // nao abre se modal esta aberto — CR-433
    if crate::runtime::stack::has_modal_open() { return; }
    let Ok(Some(content)) = root.query_selector("[data-rs-context-menu-content]") else { return };
    let Ok(_el) = content.clone().dyn_into::<HtmlElement>() else { return };
    if let Ok(root_html) = root.clone().dyn_into::<HtmlElement>() {
        let _ = root_html.style().set_property("--context-menu-x", &format!("{}px", x));
        let _ = root_html.style().set_property("--context-menu-y", &format!("{}px", y));
    }
    state::open(&content);
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    // contextmenu — abre no ponto do click direito
    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            e.prevent_default();
            let Some(current) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            // fechar outros context menus abertos
            query::each("[data-rs-context-menu][data-rs-initialized='true']", |node| {
                if state::is_open(&node) { state::close(&node); }
            });
            position_and_open(&current, e.client_x(), e.client_y());
        });
        let _ = root.add_event_listener_with_callback("contextmenu", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // click em item — fecha
    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(current) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(target)  = query::safe_target(&e) else { return };
            if query::closest(&target, "[data-rs-context-menu-item]") {
                let disabled = target.get_attribute("aria-disabled").as_deref() == Some("true");
                if !disabled { state::close(&current); }
            }
        });
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // outside click — fecha via document
    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
            query::each("[data-rs-context-menu][data-rs-initialized='true']", |node| {
                if state::is_open(&node) { state::close(&node); }
            });
        });
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let _ = doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        }
        cb.forget();
    }

    // Escape
    {
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
            if e.key() != "Escape" { return; }
            query::each("[data-rs-context-menu][data-rs-initialized='true']", |node| {
                if state::is_open(&node) { state::close(&node); }
            });
        });
        if let Some(win) = web_sys::window() {
            let _ = win.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        }
        cb.forget();
    }
}
