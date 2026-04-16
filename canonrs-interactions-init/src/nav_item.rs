//! NavItem + NavGroup Init

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state, query, keyboard};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    if root.has_attribute("data-rs-nav-group") {
        let direction = root.get_attribute("data-rs-direction").unwrap_or_default();
        let is_horizontal = direction == "horizontal";

        // SSR bootstrap
        let all = query::all(&root, "[data-rs-nav-item]");
        let mut ssr_idx: Option<usize> = None;
        for (i, el) in all.iter().enumerate() {
            let s = el.get_attribute("data-rs-state").unwrap_or_default();
            let is_active = s.trim() == "active";
            let _ = el.set_attribute("data-rs-selected", if is_active { "true" } else { "false" });
            if is_active { ssr_idx = Some(i); }
        }

        // select por idx — opera nos não-disabled
        let select_by_idx = {
            let root = root.clone();
            move |idx: usize| {
                let all = query::all(&root, "[data-rs-nav-item]");
                for el in &all { let _ = el.set_attribute("data-rs-selected", "false"); }
                let enabled: Vec<&Element> = all.iter()
                    .filter(|el| el.get_attribute("data-rs-disabled").as_deref() != Some("true"))
                    .collect();
                if let Some(el) = enabled.get(idx) {
                    let _ = el.set_attribute("data-rs-selected", "true");
                }
            }
        };

        // hover
        {
            let root_hover = root.clone();
            let hover_cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
                let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
                let Some(item) = target.closest("[data-rs-nav-item]").ok().flatten() else { return };
                for el in query::all(&root_hover, "[data-rs-nav-item]") { state::remove_state(&el, "hover"); }
                if item.get_attribute("data-rs-disabled").as_deref() != Some("true") {
                    state::add_state(&item, "hover");
                }
            });
            let root_leave = root.clone();
            let leave_cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
                for el in query::all(&root_leave, "[data-rs-nav-item]") { state::remove_state(&el, "hover"); }
            });
            let _ = root.add_event_listener_with_callback("mouseover", hover_cb.as_ref().unchecked_ref());
            let _ = root.add_event_listener_with_callback("mouseleave", leave_cb.as_ref().unchecked_ref());
            hover_cb.forget();
            leave_cb.forget();
        }

        // click
        let current_idx = keyboard::init_nav(
            &root,
            "[data-rs-nav-item]",
            keyboard::NavConfig {
                orientation: if is_horizontal { keyboard::Orientation::Horizontal } else { keyboard::Orientation::Vertical },
                element_type: keyboard::ElementType::Link,
                focus_state: "focused",
                wrap: false,
            },
            Some(Box::new({
                let select = select_by_idx.clone();
                move |idx, _items| { select(idx); }
            })),
            None,
        );

        if let Some(idx) = ssr_idx { current_idx.set(Some(idx)); }

        // click — sincronizar current_idx e selected
        {
            let root_cb = root.clone();
            let idx_click = current_idx.clone();
            let select_click = select_by_idx.clone();
            let click_cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
                let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
                let Some(item) = target.closest("[data-rs-nav-item]").ok().flatten() else { return };
                let href = item.get_attribute("href").unwrap_or_default();
                if href == "#" || href.is_empty() { e.prevent_default(); }
                let all = query::all(&root_cb, "[data-rs-nav-item]");
                let enabled: Vec<&Element> = all.iter()
                    .filter(|el| el.get_attribute("data-rs-disabled").as_deref() != Some("true"))
                    .collect();
                if let Some(idx) = keyboard::find_idx_by_uid(
                    &enabled.iter().map(|e| (*e).clone()).collect::<Vec<_>>(), &item
                ) {
                    select_click(idx);
                    idx_click.set(Some(idx));
                }
            });
            let _ = root.add_event_listener_with_callback("click", click_cb.as_ref().unchecked_ref());
            click_cb.forget();
        }

    } else {
        let r = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let href = r.get_attribute("href").unwrap_or_default();
            if href == "#" || href.is_empty() { e.prevent_default(); }
        });
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}
