//! Pagination Interaction Engine

use wasm_bindgen::prelude::*;
use crate::shared::{remove_state, is_initialized, mark_initialized};
use wasm_bindgen::JsCast;
use web_sys::Element;

fn add_state(el: &Element, state: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    if !current.split_whitespace().any(|s| s == state) {
        let next = if current.is_empty() { state.to_string() } else { format!("{} {}", current, state) };
        el.set_attribute("data-rs-state", &next).ok();
    }
}


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

fn set_page(root: &Element, page: usize) {
    let total = get_total_pages(root);
    let page = page.max(1).min(total);
    let _ = root.set_attribute("data-rs-current-page", &page.to_string());

    // update link states
    let Ok(links) = root.query_selector_all("[data-rs-pagination-link]") else { return };
    for i in 0..links.length() {
        if let Some(n) = links.item(i) {
            if let Ok(el) = n.dyn_into::<Element>() {
                let link_page = el.get_attribute("data-rs-page")
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(0);
                remove_state(&el, "active");
                remove_state(&el, "inactive");
                if link_page == page {
                    add_state(&el, "active");
                    let _ = el.set_attribute("aria-current", "page");
                } else {
                    add_state(&el, "inactive");
                    let _ = el.set_attribute("aria-current", "false");
                }
            }
        }
    }

    // update prev/next disabled state
    if let Ok(Some(prev)) = root.query_selector("[data-rs-pagination-previous]") {
        remove_state(&prev, "disabled"); remove_state(&prev, "inactive");
        if page <= 1 { add_state(&prev, "disabled"); let _ = prev.set_attribute("aria-disabled", "true"); }
        else { add_state(&prev, "inactive"); let _ = prev.set_attribute("aria-disabled", "false"); }
    }
    if let Ok(Some(next)) = root.query_selector("[data-rs-pagination-next]") {
        remove_state(&next, "disabled"); remove_state(&next, "inactive");
        if page >= total { add_state(&next, "disabled"); let _ = next.set_attribute("aria-disabled", "true"); }
        else { add_state(&next, "inactive"); let _ = next.set_attribute("aria-disabled", "false"); }
    }
}

pub fn init(root: Element) {
    if is_initialized(&root) { return; }
    mark_initialized(&root);
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };

            if target.closest("[data-rs-pagination-previous]").ok().flatten().is_some() {
                e.prevent_default();
                let cur = get_current_page(&root_cb);
                if cur > 1 { set_page(&root_cb, cur - 1); }
                return;
            }
            if target.closest("[data-rs-pagination-next]").ok().flatten().is_some() {
                e.prevent_default();
                let cur = get_current_page(&root_cb);
                set_page(&root_cb, cur + 1);
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
    }
}

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-pagination-link]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init(el); }
        }
    }
}
