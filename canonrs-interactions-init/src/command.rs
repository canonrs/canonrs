//! Command Init — search + keyboard navigation

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state, query, keyboard};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    // conectar aria-controls do input ao listbox
    if let (Some(input), Some(list)) = (
        query::first(&root, "[data-rs-command-input]"),
        query::first(&root, "[data-rs-command-list]")
    ) {
        if let Some(list_id) = list.get_attribute("id") {
            let _ = input.set_attribute("aria-controls", &list_id);
            let _ = input.set_attribute("aria-expanded", "true");
        }
    }

    // ativar primeiro item visivel automaticamente
    let first_items = query::all(&root, "[data-rs-command-item]");
    if let Some(first) = first_items.iter().find(|el| {
        !el.get_attribute("data-rs-disabled").map(|s| s == "true").unwrap_or(false)
    }) {
        state::add_state(first, "active");
        let _ = first.set_attribute("aria-selected", "true");
        if let Some(input) = query::first(&root, "[data-rs-command-input]") {
            let item_id = first.get_attribute("id").unwrap_or_default();
            let _ = input.set_attribute("aria-activedescendant", &item_id);
        }
    }

    // search/filter
    let root_input = root.clone();
    if let Some(input) = query::first(&root, "[data-rs-command-input]") {
        let cb = Closure::<dyn Fn(web_sys::Event)>::new(move |e: web_sys::Event| {
            let query_str = e.target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                .map(|i| i.value().to_lowercase())
                .unwrap_or_default();
            let items = query::all(&root_input, "[data-rs-command-item]");
            let mut visible = 0;
            for item in &items {
                let value = item.get_attribute("data-rs-value").unwrap_or_default().to_lowercase();
                let text = item.text_content().unwrap_or_default().to_lowercase();
                if query_str.is_empty() || value.contains(&query_str) || text.contains(&query_str) {
                    state::remove_state(item, "hidden");
                    visible += 1;
                } else {
                    state::add_state(item, "hidden");
                }
            }
            if let Some(empty) = query::first(&root_input, "[data-rs-command-empty]") {
                if visible == 0 { state::remove_state(&empty, "hidden"); }
                else { state::add_state(&empty, "hidden"); }
            }
            // atualizar item ativo para primeiro visível
            let all_items = query::all(&root_input, "[data-rs-command-item]");
            for el in &all_items {
                state::remove_state(el, "active");
                let _ = el.set_attribute("aria-selected", "false");
            }
            if let Some(first) = all_items.iter().find(|el| {
                !el.get_attribute("data-rs-state").map(|s| s.contains("hidden")).unwrap_or(false) &&
                el.get_attribute("data-rs-disabled").as_deref() != Some("true")
            }) {
                state::add_state(first, "active");
                let _ = first.set_attribute("aria-selected", "true");
                if let Some(input) = query::first(&root_input, "[data-rs-command-input]") {
                    let _ = input.set_attribute("aria-activedescendant", &first.get_attribute("id").unwrap_or_default());
                }
            }
        });
        let _ = input.add_event_listener_with_callback("input", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // hover via delegation
    let root_hover = root.clone();
    let enter_cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
        let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        let Some(item) = target.closest("[data-rs-command-item]").ok().flatten() else { return };
        if item.get_attribute("data-rs-disabled").as_deref() == Some("true") { return; }
        for el in query::all(&root_hover, "[data-rs-command-item]") {
            state::remove_state(&el, "active");
            let _ = el.set_attribute("aria-selected", "false");
        }
        state::add_state(&item, "active");
        let _ = item.set_attribute("aria-selected", "true");
    });
    let _ = root.add_event_listener_with_callback("mouseover", enter_cb.as_ref().unchecked_ref());
    enter_cb.forget();

    // keyboard via runtime module
    let root_close = root.clone();
    keyboard::init_nav(
        &root,
        "[data-rs-command-item]",
        keyboard::NavConfig {
            orientation: keyboard::Orientation::Vertical,
            element_type: keyboard::ElementType::Button,
            focus_state: "active",
            wrap: false,
        },
        None,
        Some(Box::new(move || { state::close(&root_close); })),
    );
}
