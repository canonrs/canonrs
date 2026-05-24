//! Menu Init

use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::{state, query};
use canonrs_interactions_core::runtime::listeners;
use crate::runtime::{interactive, keyboard};

pub fn init(root: Element) {
    let all = query::all(&root, "[data-rs-menu-item]");
    let mut ssr_idx: Option<usize> = None;
    for (i, el) in all.iter().enumerate() {
        let s = el.get_attribute("data-rs-state").unwrap_or_default();
        if s.contains("selected") { ssr_idx = Some(i); state::add_state(el, "focused"); }
        interactive::init(el);
    }

    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

    listeners::listen(&uid, &root, "click", {
        let root_c = root.clone();
        move |e: web_sys::Event| {
            let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(item) = target.closest("[data-rs-menu-item]").ok().flatten() else { return };
            if item.get_attribute("data-rs-disabled").as_deref() == Some("true") { return; }
            for el in query::all(&root_c, "[data-rs-menu-item]") {
                state::remove_state(&el, "selected"); state::remove_state(&el, "focused");
            }
            state::add_state(&item, "selected"); state::add_state(&item, "focused");
        }
    });

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
            let root_c = root.clone();
            move |idx, items| {
                for el in query::all(&root_c, "[data-rs-menu-item]") {
                    state::remove_state(&el, "selected"); state::remove_state(&el, "focused");
                }
                if let Some(el) = items.get(idx) {
                    state::add_state(el, "selected"); state::add_state(el, "focused");
                }
            }
        })),
        None,
    );

    if let Some(idx) = ssr_idx { current_idx.set(Some(idx)); }

    listeners::listen(&uid, &root, "click", {
        let root_c = root.clone();
        let idx_sync = current_idx.clone();
        move |e: web_sys::Event| {
            let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(item) = target.closest("[data-rs-menu-item]").ok().flatten() else { return };
            if item.get_attribute("data-rs-disabled").as_deref() == Some("true") { return; }
            let all = query::all(&root_c, "[data-rs-menu-item]");
            let enabled: Vec<Element> = all.into_iter()
                .filter(|el| el.get_attribute("data-rs-disabled").as_deref() != Some("true"))
                .collect();
            if let Some(idx) = keyboard::find_idx_by_uid(&enabled, &item) { idx_sync.set(Some(idx)); }
        }
    });
}
