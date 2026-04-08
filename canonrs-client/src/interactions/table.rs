//! Table Interaction Engine — sort + row selection

use wasm_bindgen::prelude::*;
use crate::shared::{add_state, remove_state, is_initialized, mark_initialized};
use wasm_bindgen::JsCast;
use web_sys::Element;

pub fn init(root: Element) {
    if is_initialized(&root) { return; }
    mark_initialized(&root);

    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(th) = target.closest("[data-rs-table-head]").ok().flatten() else { return };
            let next_sort = match th.get_attribute("data-rs-sort").as_deref() {
                Some("none") | Some("desc") | None => "asc",
                Some("asc") => "desc",
                _ => "none",
            };
            if let Ok(heads) = root_cb.query_selector_all("[data-rs-table-head]") {
                for i in 0..heads.length() {
                    if let Some(n) = heads.item(i) {
                        if let Ok(el) = n.dyn_into::<Element>() { let _ = el.set_attribute("data-rs-sort", "none"); }
                    }
                }
            }
            let _ = th.set_attribute("data-rs-sort", next_sort);
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(row) = target.closest("[data-rs-table-row]").ok().flatten() else { return };
            if row.get_attribute("data-rs-state").map(|s| s.contains("selected")).unwrap_or(false) {
                remove_state(&row, "selected"); add_state(&row, "unselected");
            } else {
                remove_state(&row, "unselected"); add_state(&row, "selected");
            }
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-table]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init(el); }
        }
    }
}
