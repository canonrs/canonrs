use web_sys::HtmlElement;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::runtime::context;
use canonrs_interactions_core::dom::attrs;
use super::pagination::{set_page, update_pagination_ui};

pub fn init(table: &HtmlElement) {
    let heads = table.query_selector_all("[data-rs-datatable-head-cell]").ok();
    if let Some(list) = heads {
        for i in 0..list.length() {
            if let Some(node) = list.item(i) {
                if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
                        let Some(t) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
                        let col_idx = attrs::get_usize(&t, "data-rs-col-index", 0);
                        let Some(rc) = context::find_root(&t, "[data-rs-datatable]") else { return };
                        if let Ok(tbl) = rc.dyn_into::<HtmlElement>() {
                            handle_sort(&tbl, col_idx);
                            set_page(&tbl, 1);
                            update_pagination_ui(&tbl);
                        }
                    }));
                    let _ = el.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
                    cb.forget();
                }
            }
        }
    }
}

fn handle_sort(table: &HtmlElement, col_idx: usize) {
    let current_col = attrs::get_usize(table, "data-rs-sort-col", usize::MAX);
    let current_asc = table.get_attribute("data-rs-sort-asc").as_deref() == Some("true");
    let (new_col, new_asc) = if current_col == col_idx {
        if current_asc { (col_idx, false) } else { (usize::MAX, true) }
    } else { (col_idx, true) };
    let _ = table.set_attribute("data-rs-sort-col", &new_col.to_string());
    let _ = table.set_attribute("data-rs-sort-asc", if new_asc { "true" } else { "false" });
    let heads = table.query_selector_all("[data-rs-datatable-head-cell]").ok();
    if let Some(list) = heads {
        for i in 0..list.length() {
            if let Some(node) = list.item(i) {
                if let Ok(el) = node.dyn_into::<HtmlElement>() {
                    let idx = attrs::get_usize(&el, "data-rs-col-index", usize::MAX);
                    let icon = el.query_selector("[data-rs-datatable-sort-icon]").ok().flatten();
                    if let Some(icon_el) = icon {
                        icon_el.set_text_content(Some(if new_col == usize::MAX || idx != new_col { "↕" }
                            else if new_asc { "▲" } else { "▼" }));
                    }
                }
            }
        }
    }
    apply_sort(table, if new_col == usize::MAX { None } else { Some(new_col) }, new_asc);
}

fn apply_sort(table: &HtmlElement, col: Option<usize>, asc: bool) {
    let tbody = match table.query_selector("[data-rs-datatable-body]").ok().flatten() {
        Some(el) => el, None => return,
    };
    let rows = table.query_selector_all("[data-rs-datatable-row]").ok();
    if let Some(list) = rows {
        let mut indexed: Vec<(String, web_sys::Node)> = (0..list.length())
            .filter_map(|i| list.item(i))
            .filter_map(|node| {
                let el = node.clone().dyn_into::<HtmlElement>().ok()?;
                let val = if let Some(c) = col {
                    el.query_selector(&format!("[data-rs-col-index='{}']", c))
                        .ok().flatten()
                        .map(|td| td.text_content().unwrap_or_default())
                        .unwrap_or_default()
                } else { attrs::get_usize(&el, "data-rs-row-index", 0).to_string() };
                Some((val, node))
            }).collect();
        indexed.sort_by(|(a, _), (b, _)| { let ord = a.cmp(b); if asc { ord } else { ord.reverse() } });
        for (_, node) in indexed { let _ = tbody.append_child(&node); }
    }
}
