//! Menu Init — single item selection + keyboard navigation + hover

use web_sys::Element;
use crate::runtime::{lifecycle, interactive, state, query, keyboard};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    // SSR bootstrap — item selected inicializa current_idx
    let all = query::all(&root, "[data-rs-menu-item]");
    let mut ssr_idx: Option<usize> = None;
    for (i, el) in all.iter().enumerate() {
        let s = el.get_attribute("data-rs-state").unwrap_or_default();
        if s.contains("selected") {
            ssr_idx = Some(i);
            state::add_state(el, "focused");
        }
        interactive::init(el);
    }

    // click — registrado no root
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(item) = target.closest("[data-rs-menu-item]").ok().flatten() else { return };
            if item.get_attribute("data-rs-disabled").as_deref() == Some("true") { return; }
            let all = query::all(&root_cb, "[data-rs-menu-item]");
            for el in &all {
                state::remove_state(el, "selected");
                state::remove_state(el, "focused");
            }
            state::add_state(&item, "selected");
            state::add_state(&item, "focused");
        });
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // keyboard via runtime
    let current_idx = keyboard::init_nav(
        &root,
        "[data-rs-menu-item]",
        keyboard::NavConfig {
            orientation: keyboard::Orientation::Vertical,
            element_type: keyboard::ElementType::Button,
            focus_state: "focused",
            wrap: false,
        },
        Some(Box::new({
            let root = root.clone();
            move |idx, items| {
                let all = query::all(&root, "[data-rs-menu-item]");
                for el in &all {
                    state::remove_state(el, "selected");
                    state::remove_state(el, "focused");
                }
                if let Some(el) = items.get(idx) {
                    state::add_state(el, "selected");
                    state::add_state(el, "focused");
                }
            }
        })),
        None,
    );

    // sincronizar SSR idx
    if let Some(idx) = ssr_idx {
        current_idx.set(Some(idx));
    }

    // sincronizar click com current_idx
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        let root_cb = root.clone();
        let idx_sync = current_idx.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(item) = target.closest("[data-rs-menu-item]").ok().flatten() else { return };
            if item.get_attribute("data-rs-disabled").as_deref() == Some("true") { return; }
            let all = query::all(&root_cb, "[data-rs-menu-item]");
            let enabled: Vec<&Element> = all.iter()
                .filter(|el| el.get_attribute("data-rs-disabled").as_deref() != Some("true"))
                .collect();
            if let Some(idx) = keyboard::find_idx_by_uid(&enabled.iter().map(|e| (*e).clone()).collect::<Vec<_>>(), &item) {
                idx_sync.set(Some(idx));
            }
        });
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}
