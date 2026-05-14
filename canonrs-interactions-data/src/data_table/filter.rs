use web_sys::{HtmlElement, HtmlInputElement};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::runtime::context;
use super::pagination::{set_page, update_pagination_ui, count_visible};

pub fn get_filter_input(root: &web_sys::Element) -> Option<HtmlInputElement> {
    root.query_selector("[data-rs-datatable-filter]").ok().flatten()
        .and_then(|el| el.dyn_into::<HtmlInputElement>().ok())
}

pub fn init(table: &HtmlElement) {
    let root: web_sys::Element = table.clone().into();
    let Some(input) = get_filter_input(&root) else { return };

    let cb = Closure::<dyn Fn(web_sys::Event)>::wrap(Box::new(move |e: web_sys::Event| {
        let Some(t) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
        let Some(rc) = context::find_root(&t, "[data-rs-datatable]") else { return };
        let q = get_filter_input(&rc).map(|i| i.value().to_lowercase()).unwrap_or_default();
        if let Ok(t) = rc.dyn_into::<HtmlElement>() {
            apply_filter(&t, &q);
            set_page(&t, 1);
            update_pagination_ui(&t);
        }
    }));
    let _ = input.add_event_listener_with_callback("input", cb.as_ref().unchecked_ref());
    cb.forget();
}

pub fn apply_filter(table: &HtmlElement, q: &str) {
    let rows = table.query_selector_all("[data-rs-datatable-row]").ok();
    if let Some(list) = rows {
        let mut visible = 0usize;
        for i in 0..list.length() {
            if let Some(node) = list.item(i) {
                if let Ok(el) = node.dyn_into::<HtmlElement>() {
                    let text = el.inner_text().to_lowercase();
                    let show = q.is_empty() || text.contains(q);
                    if show { let _ = el.remove_attribute("data-rs-filtered"); }
                    else { let _ = el.set_attribute("data-rs-filtered", "hidden"); }
                    if show { visible += 1; }
                }
            }
        }
        let empty = table.query_selector("[data-rs-datatable-empty]").ok().flatten();
        if let Some(el) = empty {
            if visible == 0 { let _ = el.remove_attribute("hidden"); }
            else { let _ = el.set_attribute("hidden", ""); }
        }
    }
    let page_size = canonrs_interactions_core::dom::attrs::get_usize(table, "data-rs-page-size", 10);
    let total = count_visible(table);
    let total_pages = ((total as f64) / (page_size as f64)).ceil().max(1.0) as usize;
    let _ = table.set_attribute("data-rs-total-pages", &total_pages.to_string());
}
