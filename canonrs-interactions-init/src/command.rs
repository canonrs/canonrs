//! Command Init

use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::{state, query};
use canonrs_interactions_core::runtime::listeners;
use crate::runtime::keyboard;

pub fn init(root: Element) {
    if let (Some(input), Some(list)) = (
        query::first(&root, "[data-rs-command-input]"),
        query::first(&root, "[data-rs-command-list]")
    ) {
        if let Some(list_id) = list.get_attribute("id") {
            let _ = input.set_attribute("aria-controls", &list_id);
            let _ = input.set_attribute("aria-expanded", "true");
        }
    }

    let first_items = query::all(&root, "[data-rs-command-item]");
    if let Some(first) = first_items.iter().find(|el| {
        el.get_attribute("data-rs-disabled").as_deref() != Some("true")
    }) {
        state::add_state(first, "active");
        let _ = first.set_attribute("aria-selected", "true");
        if let Some(input) = query::first(&root, "[data-rs-command-input]") {
            let _ = input.set_attribute("aria-activedescendant", &first.get_attribute("id").unwrap_or_default());
        }
    }

    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

    if let Some(input) = query::first(&root, "[data-rs-command-input]") {
        listeners::listen(&uid, &input, "input", {
            let root_c = root.clone();
            move |e: web_sys::Event| {
                let query_str = e.target()
                    .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                    .map(|i| i.value().to_lowercase()).unwrap_or_default();
                let items = query::all(&root_c, "[data-rs-command-item]");
                let mut visible = 0;
                for item in &items {
                    let value = item.get_attribute("data-rs-value").unwrap_or_default().to_lowercase();
                    let text  = item.text_content().unwrap_or_default().to_lowercase();
                    if query_str.is_empty() || value.contains(&query_str) || text.contains(&query_str) {
                        state::remove_state(item, "hidden"); visible += 1;
                    } else {
                        state::add_state(item, "hidden");
                    }
                }
                if let Some(empty) = query::first(&root_c, "[data-rs-command-empty]") {
                    if visible == 0 { state::remove_state(&empty, "hidden"); }
                    else            { state::add_state(&empty, "hidden"); }
                }
                let all = query::all(&root_c, "[data-rs-command-item]");
                for el in &all { state::remove_state(el, "active"); let _ = el.set_attribute("aria-selected", "false"); }
                if let Some(first) = all.iter().find(|el| {
                    !el.get_attribute("data-rs-state").map(|s| s.contains("hidden")).unwrap_or(false) &&
                    el.get_attribute("data-rs-disabled").as_deref() != Some("true")
                }) {
                    state::add_state(first, "active");
                    let _ = first.set_attribute("aria-selected", "true");
                    if let Some(input) = query::first(&root_c, "[data-rs-command-input]") {
                        let _ = input.set_attribute("aria-activedescendant", &first.get_attribute("id").unwrap_or_default());
                    }
                }
            }
        });
    }

    listeners::listen(&uid, &root, "mouseover", {
        let root_c = root.clone();
        move |e: web_sys::Event| {
            let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(item) = target.closest("[data-rs-command-item]").ok().flatten() else { return };
            if item.get_attribute("data-rs-disabled").as_deref() == Some("true") { return; }
            for el in query::all(&root_c, "[data-rs-command-item]") {
                state::remove_state(&el, "active"); let _ = el.set_attribute("aria-selected", "false");
            }
            state::add_state(&item, "active"); let _ = item.set_attribute("aria-selected", "true");
        }
    });

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
