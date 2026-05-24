//! Sidebar Interaction Engine
//! Core: dom/{state, query} + behavior/keyboard::init_nav

use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::{state, query};
use canonrs_interactions_core::runtime::listeners;
use canonrs_interactions_core::behavior::keyboard::{init_nav, NavConfig, Orientation, ElementType};

fn is_pinned(root: &Element) -> bool {
    root.get_attribute("data-rs-pinned").as_deref() == Some("true")
}

pub fn init(root: Element) {
    let is_rail = root.get_attribute("data-rs-variant").as_deref() == Some("rail");

    if !state::has(&root, canonrs_interactions_core::dom::state::State::Expanded.as_str()) && !state::has(&root, canonrs_interactions_core::dom::state::State::Collapsed.as_str()) {
        state::add(&root, canonrs_interactions_core::dom::state::State::Collapsed.as_str());
    }

    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

    // toggle + pin + menu item + group toggle — single click listener
    listeners::listen(&uid, &root, "click", {
        let root_c = root.clone();
        move |e: web_sys::Event| {
            let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };

            // sidebar toggle
            if target.closest("[data-rs-sidebar-toggle]").ok().flatten().is_some() {
                if is_pinned(&root_c) { return; }
                if state::is_expanded(&root_c) {
                    state::collapse(&root_c);
                    for el in query::all(&root_c, "[data-rs-tooltip-content]") { state::close(&el); }
                } else {
                    state::expand(&root_c);
                    for el in query::all(&root_c, "[data-rs-tooltip-content]") { state::close(&el); }
                }
                return;
            }

            // pin toggle
            if target.closest("[data-rs-sidebar-pin-toggle]").ok().flatten().is_some() {
                let pinned = is_pinned(&root_c);
                let _ = root_c.set_attribute("data-rs-pinned", if pinned { "false" } else { "true" });
                if !pinned { state::expand(&root_c); }
                return;
            }

            // group toggle
            if let Some(toggle) = target.closest("[data-rs-sidebar-group-toggle]").ok().flatten() {
                e.stop_propagation();
                if let Some(group) = toggle.closest("[data-rs-sidebar-group]").ok().flatten() {
                    if state::is_expanded(&group) { state::collapse(&group); } else { state::expand(&group); }
                }
                return;
            }

            // menu item activate
            if let Some(item) = target.closest("[data-rs-sidebar-menu-item]").ok().flatten() {
                if state::has(&item, canonrs_interactions_core::dom::state::State::Disabled.as_str()) { return; }
                for el in query::all(&root_c, "[data-rs-sidebar-menu-item]") {
                    state::remove(&el, canonrs_interactions_core::dom::state::State::Active.as_str());
                    state::add(&el, canonrs_interactions_core::dom::state::State::Inactive.as_str());
                    let _ = el.remove_attribute("aria-current");
                }
                state::remove(&item, canonrs_interactions_core::dom::state::State::Inactive.as_str());
                state::add(&item, canonrs_interactions_core::dom::state::State::Active.as_str());
                let _ = item.set_attribute("aria-current", "page");
            }
        }
    });

    // keyboard nav
    init_nav(
        &root,
        "[data-rs-sidebar-menu-item]",
        NavConfig {
            orientation:  Orientation::Vertical,
            element_type: ElementType::Link,
            focus_state:  "focused",
            wrap:         false,
        },
        None,
        None,
    );

    // sidebar search
    if let Some(search) = query::first(&root, "[data-rs-sidebar-search]") {
        if let Some(list) = query::first(&search, "[data-rs-command-list]") {
            let _ = list.set_attribute("data-rs-hidden", "true");
            let style = list.get_attribute("style").unwrap_or_default();
            let _ = list.set_attribute("style", &format!("{}display:none;", style));
        }
        if let Some(input) = query::first(&search, "[data-rs-command-input]") {
            let uid_s  = format!("{}:search", uid);
            let input_el = input.clone().unchecked_into::<Element>();
            listeners::listen(&uid_s, &input_el, "input", {
                let search_c = search.clone();
                move |e: web_sys::Event| {
                    let value = e.target()
                        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                        .map(|i| i.value())
                        .unwrap_or_default();
                    if let Some(list) = query::first(&search_c, "[data-rs-command-list]") {
                        let _ = list.set_attribute("style", if value.is_empty() { "display:none;" } else { "display:block;" });
                    }
                }
            });
        }
    }

    // rail hover
    if is_rail {
        listeners::listen(&uid, &root, "mouseenter", {
            let root_c = root.clone();
            move |_: web_sys::Event| { state::expand(&root_c); }
        });
        listeners::listen(&uid, &root, "mouseleave", {
            let root_c = root.clone();
            move |_: web_sys::Event| { if !is_pinned(&root_c) { state::collapse(&root_c); } }
        });
    }
}
