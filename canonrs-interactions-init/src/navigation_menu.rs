//! NavigationMenu Init — open/close + keyboard navigation

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state, query, interactive, keyboard};

fn close_all(root: &Element) {
    for item in query::all(root, "[data-rs-navigation-menu-item]") {
        state::close(&item);
        if let Some(content) = query::first(&item, "[data-rs-navigation-menu-content]") {
            let _ = content.set_attribute("aria-hidden", "true");
        }
    }
}

fn open_item(item: &Element) {
    state::open(item);
    if let Some(content) = query::first(item, "[data-rs-navigation-menu-content]") {
        let _ = content.set_attribute("aria-hidden", "false");
    }
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    // hover/active nos triggers e links
    for el in query::all(&root, "[data-rs-navigation-menu-trigger]") { interactive::init(&el); }
    for el in query::all(&root, "[data-rs-navigation-menu-link]") { interactive::init(&el); }

    // bootstrap — garantir tudo fechado
    close_all(&root);

    // click trigger
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            // e.stop_propagation(); — removido para permitir sync do triggers_idx
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(_trigger) = target.closest("[data-rs-navigation-menu-trigger]").ok().flatten() else { return };
            let Some(item) = target.closest("[data-rs-navigation-menu-item]").ok().flatten() else { return };
            let is_open = state::is_open(&item);
            close_all(&root_cb);
            if !is_open { open_item(&item); }
        });
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // sincronizar click com current_idx dos triggers
    let triggers_idx = keyboard::init_nav(
        &root,
        "[data-rs-navigation-menu-trigger]",
        keyboard::NavConfig {
            orientation: keyboard::Orientation::Horizontal,
            element_type: keyboard::ElementType::Button,
            focus_state: "focused",
            wrap: false,
        },
        Some(Box::new({
            move |_idx, items| {
                if let Some(trigger) = items.get(_idx) {
                    if let Some(item) = trigger.closest("[data-rs-navigation-menu-item]").ok().flatten() {
                        let is_open = state::is_open(&item);
                        if !is_open { open_item(&item); }
                        let links = query::all(&item, "[data-rs-navigation-menu-link]");
                        if let Some(first) = links.into_iter().next() {
                            if let Ok(el) = first.dyn_into::<web_sys::HtmlElement>() { let _ = el.focus(); }
                        }
                    }
                }
            }
        })),
        None,
    );

    // click trigger — sincronizar current_idx por posição
    {
        let root_cb = root.clone();
        let idx_sync = triggers_idx.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(trigger) = target.closest("[data-rs-navigation-menu-trigger]").ok().flatten() else { return };
            let triggers = query::all(&root_cb, "[data-rs-navigation-menu-trigger]");
            if let Some(idx) = triggers.iter().position(|el| el.contains(Some(trigger.as_ref()))) {
                idx_sync.set(Some(idx));
            }
        });
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }



    // keyboard links — keydown no root, links são <a>
    {
        let _root_kb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let in_content = target.closest("[data-rs-navigation-menu-content]").ok().flatten().is_some();
            if !in_content { return; }
            let Some(item) = target.closest("[data-rs-navigation-menu-item]").ok().flatten() else { return };
            let links: Vec<Element> = query::all(&item, "[data-rs-navigation-menu-link]");
            let current = links.iter().position(|el| el.contains(Some(target.as_ref())));
            match e.key().as_str() {
                "ArrowDown" => {
                    e.prevent_default();
                    let next = current.map(|i| (i + 1).min(links.len() - 1)).unwrap_or(0);
                    if let Ok(el) = links[next].clone().dyn_into::<web_sys::HtmlElement>() { let _ = el.focus(); }
                }
                "ArrowUp" => {
                    e.prevent_default();
                    let prev = current.map(|i| if i == 0 { 0 } else { i - 1 }).unwrap_or(0);
                    if let Ok(el) = links[prev].clone().dyn_into::<web_sys::HtmlElement>() { let _ = el.focus(); }
                }
                "Escape" => {
                    e.prevent_default();
                    e.stop_propagation();
                    // focar trigger ANTES de fechar — CSS clip não bloqueia foco no trigger
                    if let Some(trigger) = query::first(&item, "[data-rs-navigation-menu-trigger]") {
                        if let Ok(el) = trigger.clone().dyn_into::<web_sys::HtmlElement>() {
                            let _ = el.focus();
                        }
                    }
                    // fechar depois do foco
                    state::close(&item);
                }
                _ => {}
            }
        });
        let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // click outside
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if root_cb.contains(Some(&target)) { return; }
            close_all(&root_cb);
        });
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let _ = doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        }
        cb.forget();
    }
}
