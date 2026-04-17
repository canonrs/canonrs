//! Pagination Interaction Engine

use wasm_bindgen::prelude::*;
use crate::runtime::{lifecycle, state};
use wasm_bindgen::JsCast;
use web_sys::Element;

fn get_current_page(root: &Element) -> usize {
    root.get_attribute("data-rs-current-page")
        .and_then(|s| s.parse().ok())
        .unwrap_or(1)
}

fn get_total_pages(root: &Element) -> usize {
    root.get_attribute("data-rs-total-pages")
        .and_then(|s| s.parse().ok())
        .unwrap_or(1)
}

fn is_disabled(el: &Element) -> bool {
    el.get_attribute("data-rs-state")
        .map(|s| s.contains("disabled"))
        .unwrap_or(false)
        || el.get_attribute("data-rs-disabled").as_deref() == Some("true")
}

fn set_page(root: &Element, page: usize) {
    let total = get_total_pages(root);
    let page = page.max(1).min(total);
    let _ = root.set_attribute("data-rs-current-page", &page.to_string());

    let Ok(links) = root.query_selector_all("[data-rs-pagination-link]") else { return };
    for i in 0..links.length() {
        if let Some(n) = links.item(i) {
            if let Ok(el) = n.dyn_into::<Element>() {
                let link_page = el.get_attribute("data-rs-page")
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(0);
                state::remove_state(&el, "active");
                state::remove_state(&el, "inactive");
                if link_page == page {
                    state::add_state(&el, "active");
                    let _ = el.set_attribute("aria-current", "page");
                } else {
                    state::add_state(&el, "inactive");
                    let _ = el.remove_attribute("aria-current");
                }
            }
        }
    }

    if let Ok(Some(prev)) = root.query_selector("[data-rs-pagination-previous]") {
        state::remove_state(&prev, "disabled");
        state::remove_state(&prev, "inactive");
        if page <= 1 {
            state::add_state(&prev, "disabled");
            let _ = prev.set_attribute("aria-disabled", "true");
            let _ = prev.set_attribute("tabindex", "-1");
        } else {
            state::add_state(&prev, "inactive");
            let _ = prev.set_attribute("aria-disabled", "false");
            let _ = prev.set_attribute("tabindex", "0");
        }
    }
    if let Ok(Some(next)) = root.query_selector("[data-rs-pagination-next]") {
        state::remove_state(&next, "disabled");
        state::remove_state(&next, "inactive");
        if page >= total {
            state::add_state(&next, "disabled");
            let _ = next.set_attribute("aria-disabled", "true");
            let _ = next.set_attribute("tabindex", "-1");
        } else {
            state::add_state(&next, "inactive");
            let _ = next.set_attribute("aria-disabled", "false");
            let _ = next.set_attribute("tabindex", "0");
        }
    }
}

fn focus_page_link(root: &Element, page: usize) {
    use wasm_bindgen::JsCast;
    let Ok(links) = root.query_selector_all("[data-rs-pagination-link]") else { return };
    for i in 0..links.length() {
        if let Some(n) = links.item(i) {
            if let Ok(el) = n.dyn_into::<Element>() {
                let p = el.get_attribute("data-rs-page").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                if p == page {
                    if let Ok(h) = el.dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); }
                    return;
                }
            }
        }
    }
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    let root_cb = root.clone();
    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
        let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };

        if let Ok(Some(prev)) = target.closest("[data-rs-pagination-previous]") {
            e.prevent_default();
            if !is_disabled(&prev) {
                let cur = get_current_page(&root_cb);
                if cur > 1 { set_page(&root_cb, cur - 1); }
            }
            return;
        }
        if let Ok(Some(next)) = target.closest("[data-rs-pagination-next]") {
            e.prevent_default();
            if !is_disabled(&next) {
                let cur = get_current_page(&root_cb);
                set_page(&root_cb, cur + 1);
            }
            return;
        }
        if let Ok(Some(link)) = target.closest("[data-rs-pagination-link]") {
            e.prevent_default();
            if let Some(p) = link.get_attribute("data-rs-page").and_then(|s| s.parse::<usize>().ok()) {
                set_page(&root_cb, p);
            }
        }
    }));
    let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
    cb.forget();

    // keyboard nav
    let root_kb = root.clone();
    let kb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
        let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        let on_link = target.closest("[data-rs-pagination-link]").ok().flatten().is_some();
        let on_prev = target.closest("[data-rs-pagination-previous]").ok().flatten().is_some();
        let on_next = target.closest("[data-rs-pagination-next]").ok().flatten().is_some();
        if !on_link && !on_prev && !on_next { return; }

        let cur = get_current_page(&root_kb);
        let total = get_total_pages(&root_kb);

        match e.key().as_str() {
            "ArrowRight" | "ArrowDown" => {
                e.prevent_default();
                if cur < total { set_page(&root_kb, cur + 1); }
                // move foco para o próximo link
                focus_page_link(&root_kb, cur + 1);
            }
            "ArrowLeft" | "ArrowUp" => {
                e.prevent_default();
                if cur > 1 { set_page(&root_kb, cur - 1); }
                focus_page_link(&root_kb, cur - 1);
            }
            "Home" => {
                e.prevent_default();
                set_page(&root_kb, 1);
                focus_page_link(&root_kb, 1);
            }
            "End" => {
                e.prevent_default();
                set_page(&root_kb, total);
                focus_page_link(&root_kb, total);
            }
            _ => {}
        }
    }));
    let _ = root.add_event_listener_with_callback("keydown", kb.as_ref().unchecked_ref());
    kb.forget();
}
