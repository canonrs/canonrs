//! NavItem + NavGroup Init

use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::{state, query};
use canonrs_interactions_core::runtime::listeners;
use crate::runtime::keyboard;

pub fn init(root: Element) {
    if root.has_attribute("data-rs-nav-group") {
        let direction    = root.get_attribute("data-rs-direction").unwrap_or_default();
        let is_horizontal = direction == "horizontal";
        let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

        let all = query::all(&root, "[data-rs-nav-item]");
        let mut ssr_idx: Option<usize> = None;
        for (i, el) in all.iter().enumerate() {
            let s = el.get_attribute("data-rs-state").unwrap_or_default();
            let is_active = s.trim() == "active";
            let _ = el.set_attribute("data-rs-selected", if is_active { "true" } else { "false" });
            if is_active { ssr_idx = Some(i); }
        }

        let select_by_idx = {
            let root_c = root.clone();
            std::rc::Rc::new(move |idx: usize| {
                let all = query::all(&root_c, "[data-rs-nav-item]");
                for el in &all { let _ = el.set_attribute("data-rs-selected", "false"); }
                let enabled: Vec<&Element> = all.iter()
                    .filter(|el| el.get_attribute("data-rs-disabled").as_deref() != Some("true"))
                    .collect();
                if let Some(el) = enabled.get(idx) { let _ = el.set_attribute("data-rs-selected", "true"); }
            })
        };

        listeners::listen(&uid, &root, "mouseover", {
            let root_c = root.clone();
            move |e: web_sys::Event| {
                let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
                let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
                let Some(item) = target.closest("[data-rs-nav-item]").ok().flatten() else { return };
                for el in query::all(&root_c, "[data-rs-nav-item]") { state::remove_state(&el, "hover"); }
                if item.get_attribute("data-rs-disabled").as_deref() != Some("true") { state::add_state(&item, "hover"); }
            }
        });

        listeners::listen(&uid, &root, "mouseleave", {
            let root_c = root.clone();
            move |_: web_sys::Event| {
                for el in query::all(&root_c, "[data-rs-nav-item]") { state::remove_state(&el, "hover"); }
            }
        });

        let current_idx = keyboard::init_nav(
            &root,
            "[data-rs-nav-item]",
            keyboard::NavConfig {
                orientation: if is_horizontal { keyboard::Orientation::Horizontal } else { keyboard::Orientation::Vertical },
                element_type: keyboard::ElementType::Link,
                focus_state: "focused",
                wrap: false,
            },
            Some(Box::new({ let s = select_by_idx.clone(); move |idx, _| { s(idx); } })),
            None,
        );

        if let Some(idx) = ssr_idx { current_idx.set(Some(idx)); }

        listeners::listen(&uid, &root, "click", {
            let root_c = root.clone();
            let idx_click = current_idx.clone();
            let select_click = select_by_idx.clone();
            move |e: web_sys::Event| {
                let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
                let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
                let Some(item) = target.closest("[data-rs-nav-item]").ok().flatten() else { return };
                let href = item.get_attribute("href").unwrap_or_default();
                if href == "#" || href.is_empty() { e.prevent_default(); }
                let all = query::all(&root_c, "[data-rs-nav-item]");
                let enabled: Vec<Element> = all.into_iter()
                    .filter(|el| el.get_attribute("data-rs-disabled").as_deref() != Some("true"))
                    .collect();
                if let Some(idx) = keyboard::find_idx_by_uid(&enabled, &item) {
                    select_click(idx);
                    idx_click.set(Some(idx));
                }
            }
        });

    } else {
        let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
        listeners::listen(&uid, &root, "click", {
            let r = root.clone();
            move |e: web_sys::Event| {
                let href = r.get_attribute("href").unwrap_or_default();
                if href == "#" || href.is_empty() { e.prevent_default(); }
            }
        });
    }
}
