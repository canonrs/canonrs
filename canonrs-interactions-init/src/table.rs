//! Table Init — DOM micro-interactions para [data-rs-table]
//! hover row, clickable row, copy cell, truncate tooltip

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};
use crate::runtime::lifecycle;


pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    bind_clickable_rows(&root);
    bind_copy_cells(&root);
}

// ── Clickable Row ─────────────────────────────────────────────────────────────
fn bind_clickable_rows(root: &Element) {
    let Ok(rows) = root.query_selector_all("[data-rs-action='navigate']") else { return };
    for i in 0..rows.length() {
        let Some(row) = rows.item(i).and_then(|n| n.dyn_into::<Element>().ok()) else { continue };
        let href = row.get_attribute("data-rs-href").unwrap_or_default();
        if href.is_empty() { continue; }
        if let Ok(el) = row.clone().dyn_into::<HtmlElement>() {
            let _ = el.style().set_property("cursor", "pointer");
        }
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |_: web_sys::MouseEvent| {
            if let Some(win) = web_sys::window() {
                if let Ok(loc) = win.location().set_href(&href) { let _ = loc; }
            }
        }));
        let _ = row.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}

// ── Copy Cell ─────────────────────────────────────────────────────────────────
fn bind_copy_cells(root: &Element) {
    let Ok(cells) = root.query_selector_all("[data-rs-copyable]") else { return };
    for i in 0..cells.length() {
        let Some(cell) = cells.item(i).and_then(|n| n.dyn_into::<Element>().ok()) else { continue };
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(t) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let text = t.text_content().unwrap_or_default();
            if let Some(win) = web_sys::window() {
                let _ = win.navigator().clipboard().write_text(&text);
                let _ = t.set_attribute("data-rs-copied", "true");
                // remove após 1500ms
                let t_clone = t.clone();
                let cb2 = Closure::once(move || {
                    let _ = t_clone.remove_attribute("data-rs-copied");
                });
                let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
                    cb2.as_ref().unchecked_ref(), 1500
                );
                cb2.forget();
            }
        }));
        let _ = cell.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}
