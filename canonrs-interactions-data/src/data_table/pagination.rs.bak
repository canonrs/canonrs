//! Pagination — table pagination behavior
//! Core: dom/{state, attrs}
use canonrs_interactions_core::dom::lifecycle;
use web_sys::HtmlElement;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::runtime::context;

pub fn init(table: &HtmlElement) {
    if !lifecycle::init_guard(&table.clone().into()) { return; }
    let prev = table.query_selector("[data-rs-action='prev']").ok().flatten();
    let next = table.query_selector("[data-rs-action='next']").ok().flatten();

    if let Some(btn) = prev {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
            let Some(rc) = context::find_root(&t, "[data-rs-datatable]") else { return };
            if let Ok(tbl) = rc.dyn_into::<HtmlElement>() {
                let p = canonrs_interactions_core::dom::attrs::get_usize(&tbl, "data-rs-current-page", 1);
                if p > 1 { set_page(&tbl, p - 1); update_pagination_ui(&tbl); }
            }
        }));
        let _ = btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    if let Some(btn) = next {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
            let Some(rc) = context::find_root(&t, "[data-rs-datatable]") else { return };
            if let Ok(tbl) = rc.dyn_into::<HtmlElement>() {
                let p = canonrs_interactions_core::dom::attrs::get_usize(&tbl, "data-rs-current-page", 1);
                let tp = canonrs_interactions_core::dom::attrs::get_usize(&tbl, "data-rs-total-pages", 1);
                if p < tp { set_page(&tbl, p + 1); update_pagination_ui(&tbl); }
            }
        }));
        let _ = btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // init: esconde rows além da página 1
    let total = count_visible(table);
    let page_size = canonrs_interactions_core::dom::attrs::get_usize(table, "data-rs-page-size", 10);
    let total_pages = ((total as f64) / (page_size as f64)).ceil().max(1.0) as usize;
    let _ = table.set_attribute("data-rs-total-pages", &total_pages.to_string());
    set_page(table, 1);
    update_pagination_ui(table);
}

pub fn set_page(table: &HtmlElement, page: usize) {
    let _ = table.set_attribute("data-rs-current-page", &page.to_string());
    let page_size = canonrs_interactions_core::dom::attrs::get_usize(table, "data-rs-page-size", 10);
    let rows = table.query_selector_all("[data-rs-datatable-row]").ok();
    if let Some(list) = rows {
        for i in 0..list.length() {
            if let Some(node) = list.item(i) {
                if let Some(el) = node.dyn_into::<HtmlElement>().ok() {
                    let _ = el.set_attribute("hidden", "");
                }
            }
        }
        let visible_rows: Vec<HtmlElement> = (0..list.length())
            .filter_map(|i| list.item(i))
            .filter_map(|n| n.dyn_into::<HtmlElement>().ok())
            .filter(|el| el.get_attribute("data-rs-filtered").is_none())
            .collect();
        let start = (page - 1) * page_size;
        let end = start + page_size;
        for (i, el) in visible_rows.iter().enumerate() {
            if i >= start && i < end { let _ = el.remove_attribute("hidden"); }
        }
    }
}

pub fn update_pagination_ui(table: &HtmlElement) {
    let page = canonrs_interactions_core::dom::attrs::get_usize(table, "data-rs-current-page", 1);
    let total_pages = canonrs_interactions_core::dom::attrs::get_usize(table, "data-rs-total-pages", 1);
    if let Some(info) = table.query_selector("[data-rs-pagination-info]").ok().flatten() {
        info.set_text_content(Some(&format!("{} of {}", page, total_pages)));
    }
    if let Some(btn) = table.query_selector("[data-rs-action='prev']").ok().flatten()
        .and_then(|el| el.dyn_into::<web_sys::HtmlButtonElement>().ok()) { btn.set_disabled(page <= 1); }
    if let Some(btn) = table.query_selector("[data-rs-action='next']").ok().flatten()
        .and_then(|el| el.dyn_into::<web_sys::HtmlButtonElement>().ok()) { btn.set_disabled(page >= total_pages); }
}

pub fn count_visible(table: &HtmlElement) -> usize {
    table.query_selector_all("[data-rs-datatable-row]").ok()
        .map(|list| (0..list.length())
            .filter_map(|i| list.item(i))
            .filter_map(|n| n.dyn_into::<HtmlElement>().ok())
            .filter(|el| el.get_attribute("data-rs-filtered").is_none())
            .count())
        .unwrap_or(0)
}
