//! ColToggle — show/hide column behavior
//! Core: dom/{state, attrs}
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use crate::runtime::context;
use crate::runtime::listeners;
use canonrs_interactions_core::dom::state;

pub fn init(table: &HtmlElement) {
    let uid = table.get_attribute("data-rs-uid").unwrap_or_default();
    sync_col_toggle_state(table);
    let items = table.query_selector_all("[data-rs-dropdown-menu-checkbox-item]").ok();
    if let Some(list) = items {
        for i in 0..list.length() {
            if let Some(node) = list.item(i) {
                if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                    let uid2 = uid.clone();
                    listeners::listen(&uid2, &el, "click", move |e: web_sys::Event| {
                        use wasm_bindgen::JsCast;
                        let e = match e.dyn_into::<web_sys::MouseEvent>() { Ok(e) => e, Err(_) => return };
                        let Some(t) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
                        let Some(item) = t.closest("[data-rs-dropdown-menu-checkbox-item]").ok().flatten() else { return };
                        e.stop_propagation();
                        let Some(rc) = context::find_root(&item, "[data-rs-datatable]") else { return };
                        if let Some(idx_str) = item.get_attribute("data-rs-col-index") {
                            if let Ok(idx) = idx_str.parse::<usize>() {
                                if let Ok(tbl) = rc.clone().dyn_into::<HtmlElement>() {
                                    toggle_column(&tbl, idx);
                                }
                                let checked = state::has(&item, "checked");
                                if checked {
                                    state::remove(&item, "checked");
                                    state::add(&item, "unchecked");
                                    let _ = item.set_attribute("aria-checked", "false");
                                } else {
                                    state::remove(&item, "unchecked");
                                    state::add(&item, "checked");
                                    let _ = item.set_attribute("aria-checked", "true");
                                }
                            }
                        }
                    });
                }
            }
        }
    }
}

fn sync_col_toggle_state(table: &HtmlElement) {
    let root: web_sys::Element = table.clone().into();
    if let Ok(items) = root.query_selector_all("[data-rs-dropdown-menu-checkbox-item]") {
        for i in 0..items.length() {
            if let Some(el) = items.item(i).and_then(|n: web_sys::Node| n.dyn_into::<web_sys::Element>().ok()) {
                if !state::has(&el, "checked") && !state::has(&el, "unchecked") {
                    state::add(&el, "checked");
                    let _ = el.set_attribute("aria-checked", "true");
                }
            }
        }
    }
}

fn toggle_column(table: &HtmlElement, col_idx: usize) {
    let selector = format!(
        "[data-rs-datatable-head-cell][data-rs-col-index='{0}'], [data-rs-datatable-cell][data-rs-col-index='{0}']",
        col_idx
    );
    let cells = table.query_selector_all(&selector).ok();
    if let Some(list) = cells {
        for i in 0..list.length() {
            if let Some(node) = list.item(i) {
                if let Ok(el) = node.dyn_into::<HtmlElement>() {
                    el.set_hidden(!el.hidden());
                }
            }
        }
    }
}
