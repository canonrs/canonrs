//! Table Interaction Engine — sort + row selection

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;

fn add_state(el: &Element, state: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    if !current.split_whitespace().any(|s| s == state) {
        let next = if current.is_empty() { state.to_string() } else { format!("{} {}", current, state) };
        el.set_attribute("data-rs-state", &next).ok();
    }
}

fn remove_state(el: &Element, state: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    let next: Vec<&str> = current.split_whitespace().filter(|s| *s != state).collect();
    el.set_attribute("data-rs-state", &next.join(" ")).ok();
}

pub fn init(root: Element) {
    // click th → sort
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(th) = target.closest("[data-rs-table-head]").ok().flatten() else { return };
            let current_sort = th.get_attribute("data-rs-sort").unwrap_or_else(|| "none".to_string());
            let next_sort = match current_sort.as_str() {
                "none" | "desc" => "asc",
                "asc" => "desc",
                _ => "none",
            };
            // reset all other headers
            if let Ok(heads) = root_cb.query_selector_all("[data-rs-table-head]") {
                for i in 0..heads.length() {
                    if let Some(n) = heads.item(i) {
                        if let Ok(el) = n.dyn_into::<Element>() {
                            let _ = el.set_attribute("data-rs-sort", "none");
                        }
                    }
                }
            }
            let _ = th.set_attribute("data-rs-sort", next_sort);
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // click row → select
    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(row) = target.closest("[data-rs-table-row]").ok().flatten() else { return };
            let is_selected = row.get_attribute("data-rs-state").map(|s| s.contains("selected")).unwrap_or(false);
            if is_selected { remove_state(&row, "selected"); add_state(&row, "unselected"); }
            else { remove_state(&row, "unselected"); add_state(&row, "selected"); }
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-table-head]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<web_sys::Element>() { init(el); }
        }
    }
}