//! DataTable Interaction — Canon Rule #342
//! Toda a lógica vive aqui. Island apenas chama init_all().
//! DOM é a fonte de verdade via data-rs-* attributes.

use web_sys::{HtmlInputElement, HtmlElement};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::runtime::{lifecycle, state, attrs};

// ─── Entry point ─────────────────────────────────────────────────────────────

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-data-table]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<HtmlElement>() { init_table(el); }
        }
    }
}

// ─── Per-table bootstrap ─────────────────────────────────────────────────────

fn init_table(table: HtmlElement) {
    bind_filter(&table);
    bind_sort(&table);
    bind_pagination(&table);
    bind_density(&table);
    bind_col_toggle(&table);
    bind_col_dropdown(&table);
}

// ─── Filter ──────────────────────────────────────────────────────────────────

fn bind_filter(table: &HtmlElement) {
    let input = match table
        .query_selector("[data-rs-datatable-filter]")
        .ok()
        .flatten()
        .and_then(|el| el.dyn_into::<HtmlInputElement>().ok())
    {
        Some(el) => el,
        None => return,
    };

    let table_clone = table.clone();
    let input_clone = input.clone();
    let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::Event| {
        let q = input_clone.value().to_lowercase();
        apply_filter(&table_clone, &q);
        set_page(&table_clone, 1);
        update_pagination_ui(&table_clone);
    }));

    let _ = input
        .dyn_ref::<web_sys::EventTarget>()
        .map(|et| et.add_event_listener_with_callback("input", cb.as_ref().unchecked_ref()));

    cb.forget();
}

fn apply_filter(table: &HtmlElement, q: &str) {
    let rows = table.query_selector_all("[data-rs-datatable-row]").ok();
    if let Some(list) = rows {
        let mut visible = 0usize;
        for i in 0..list.length() {
            if let Some(node) = list.item(i) {
                if let Ok(el) = node.dyn_into::<HtmlElement>() {
                    let text = el.inner_text().to_lowercase();
                    let show = q.is_empty() || text.contains(q);
                    if show {
                        let _ = el.remove_attribute("data-rs-filtered");
                    } else {
                        let _ = el.set_attribute("data-rs-filtered", "hidden");
                    }
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
    let page_size = attrs::get_usize_html(table, "data-rs-page-size", 10);
    let total = count_visible(table);
    let total_pages = ((total as f64) / (page_size as f64)).ceil().max(1.0) as usize;
    let _ = table.set_attribute("data-rs-total-pages", &total_pages.to_string());
}

// ─── Sort ─────────────────────────────────────────────────────────────────────

fn bind_sort(table: &HtmlElement) {
    let heads = table.query_selector_all("[data-rs-datatable-head-cell]").ok();
    if let Some(list) = heads {
        for i in 0..list.length() {
            if let Some(node) = list.item(i) {
                if let Ok(el) = node.dyn_into::<HtmlElement>() {
                    let table_clone = table.clone();
                    let el_clone = el.clone();
                    let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::MouseEvent| {
                        let col_idx = attrs::get_usize_html(&el_clone, "data-rs-col-index", 0);
                        handle_sort(&table_clone, col_idx);
                        set_page(&table_clone, 1);
                        update_pagination_ui(&table_clone);
                    }));
                    let _ = el.dyn_ref::<web_sys::EventTarget>()
                        .map(|et| et.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()));
                    cb.forget();
                }
            }
        }
    }
}

fn handle_sort(table: &HtmlElement, col_idx: usize) {
    let current_col = attrs::get_usize_html(table, "data-rs-sort-col", usize::MAX);
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
                    let idx = attrs::get_usize_html(&el, "data-rs-col-index", usize::MAX);
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
                } else { attrs::get_usize_html(&el, "data-rs-row-index", 0).to_string() };
                Some((val, node))
            }).collect();
        indexed.sort_by(|(a, _), (b, _)| { let ord = a.cmp(b); if asc { ord } else { ord.reverse() } });
        for (_, node) in indexed { let _ = tbody.append_child(&node); }
    }
}

// ─── Pagination ───────────────────────────────────────────────────────────────

fn bind_pagination(table: &HtmlElement) {
    let prev = table.query_selector("[data-rs-action='prev']").ok().flatten();
    let next = table.query_selector("[data-rs-action='next']").ok().flatten();
    if let Some(btn) = prev {
        let table_clone = table.clone();
        let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let p = attrs::get_usize_html(&table_clone, "data-rs-current-page", 1);
            if p > 1 { set_page(&table_clone, p - 1); update_pagination_ui(&table_clone); }
        }));
        let _ = btn.dyn_ref::<web_sys::EventTarget>()
            .map(|et| et.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()));
        cb.forget();
    }
    if let Some(btn) = next {
        let table_clone = table.clone();
        let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let p = attrs::get_usize_html(&table_clone, "data-rs-current-page", 1);
            let tp = attrs::get_usize_html(&table_clone, "data-rs-total-pages", 1);
            if p < tp { set_page(&table_clone, p + 1); update_pagination_ui(&table_clone); }
        }));
        let _ = btn.dyn_ref::<web_sys::EventTarget>()
            .map(|et| et.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()));
        cb.forget();
    }
}

// FIX #342: set_page agora esconde TODAS as rows primeiro,
// depois mostra apenas as visíveis (sem data-rs-filtered) da página atual.
fn set_page(table: &HtmlElement, page: usize) {
    let _ = table.set_attribute("data-rs-current-page", &page.to_string());
    let page_size = attrs::get_usize_html(table, "data-rs-page-size", 10);
    let rows = table.query_selector_all("[data-rs-datatable-row]").ok();
    if let Some(list) = rows {
        // Passo 1: esconde todas
        for i in 0..list.length() {
            if let Some(node) = list.item(i) {
                if let Some(el) = node.dyn_into::<HtmlElement>().ok() {
                    let _ = el.set_attribute("hidden", "");
                }
            }
        }
        // Passo 2: mostra só as não-filtradas da página
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

fn update_pagination_ui(table: &HtmlElement) {
    let page = attrs::get_usize_html(table, "data-rs-current-page", 1);
    let total_pages = attrs::get_usize_html(table, "data-rs-total-pages", 1);
    if let Some(info) = table.query_selector("[data-rs-pagination-info]").ok().flatten() {
        info.set_text_content(Some(&format!("{} of {}", page, total_pages)));
    }
    if let Some(btn) = table.query_selector("[data-rs-action='prev']").ok().flatten()
        .and_then(|el| el.dyn_into::<web_sys::HtmlButtonElement>().ok()) { btn.set_disabled(page <= 1); }
    if let Some(btn) = table.query_selector("[data-rs-action='next']").ok().flatten()
        .and_then(|el| el.dyn_into::<web_sys::HtmlButtonElement>().ok()) { btn.set_disabled(page >= total_pages); }
}

// ─── Density ──────────────────────────────────────────────────────────────────

fn bind_density(table: &HtmlElement) {
    let btns = table.query_selector_all("[data-rs-density-btn]").ok();
    if let Some(list) = btns {
        for i in 0..list.length() {
            if let Some(node) = list.item(i) {
                if let Ok(el) = node.dyn_into::<HtmlElement>() {
                    let table_clone = table.clone();
                    let el_clone = el.clone();
                    let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::MouseEvent| {
                        if let Some(d) = el_clone.get_attribute("data-rs-density-btn") {
                            let _ = table_clone.set_attribute("data-rs-density", &d);
                            if let Ok(all) = table_clone.query_selector_all("[data-rs-density-btn]") {
                                for j in 0..all.length() {
                                    if let Some(btn_node) = all.item(j) {
                                        if let Ok(btn) = btn_node.dyn_into::<HtmlElement>() {
                                            let is_active = btn.get_attribute("data-rs-density-btn").as_deref() == Some(&d);
                                            if is_active { state::remove(&btn.clone().into(), "inactive"); state::add(&btn.clone().into(), "active"); } else { state::remove(&btn.clone().into(), "active"); state::add(&btn.clone().into(), "inactive"); }
                                        }
                                    }
                                }
                            }
                        }
                    }));
                    let _ = el.dyn_ref::<web_sys::EventTarget>()
                        .map(|et| et.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()));
                    cb.forget();
                }
            }
        }
    }
}

// ─── Column toggle ────────────────────────────────────────────────────────────

fn bind_col_toggle(table: &HtmlElement) {
    let items = table.query_selector_all("[data-rs-dropdown-menu-checkbox-item]").ok();
    if let Some(list) = items {
        for i in 0..list.length() {
            if let Some(node) = list.item(i) {
                if let Ok(el) = node.dyn_into::<HtmlElement>() {
                    let table_clone = table.clone();
                    let el_clone = el.clone();
                    let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::MouseEvent| {
                        if let Some(idx_str) = el_clone.get_attribute("data-rs-col-index") {
                            if let Ok(idx) = idx_str.parse::<usize>() {
                                toggle_column(&table_clone, idx);
                                let checked = el_clone.get_attribute("aria-checked").as_deref() == Some("true");
                                let _ = el_clone.set_attribute("aria-checked", if checked { "false" } else { "true" });
                            }
                        }
                    }));
                    let _ = el.dyn_ref::<web_sys::EventTarget>()
                        .map(|et| et.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()));
                    cb.forget();
                }
            }
        }
    }
}

fn toggle_column(table: &HtmlElement, col_idx: usize) {
    let selector = format!("[data-rs-col-index='{}']", col_idx);
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

// ─── Column dropdown ──────────────────────────────────────────────────────────

fn bind_col_dropdown(table: &HtmlElement) {
    let trigger = table.query_selector("[data-rs-dropdown-menu-trigger]").ok().flatten();
    if let Some(btn) = trigger {
        let table_clone = table.clone();
        let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            e.stop_propagation();
            let menu = table_clone.query_selector("[data-rs-dropdown-menu]").ok().flatten();
            if let Some(m) = menu {
                let open = m.get_attribute("data-rs-state").as_deref() == Some("open");
                let _ = m.set_attribute("data-rs-state", if open { "closed" } else { "open" });
                let content = table_clone.query_selector("[data-rs-dropdown-menu-content]").ok().flatten();
                if let Some(el) = content.and_then(|el| el.dyn_into::<HtmlElement>().ok()) {
                    el.set_hidden(open);
                }
            }
        }));
        let _ = btn.dyn_ref::<web_sys::EventTarget>()
            .map(|et| et.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()));
        cb.forget();
    }
}

// ─── Helpers ──────────────────────────────────────────────────────────────────



fn count_visible(table: &HtmlElement) -> usize {
    table.query_selector_all("[data-rs-datatable-row]").ok()
        .map(|list| (0..list.length())
            .filter_map(|i| list.item(i))
            .filter_map(|n| n.dyn_into::<HtmlElement>().ok())
            .filter(|el| el.get_attribute("data-rs-filtered").is_none())
            .count())
        .unwrap_or(0)
}

pub fn init(root: web_sys::Element) {
    if !lifecycle::init_guard(&root) { return; }
    use wasm_bindgen::JsCast;
    if let Ok(el) = root.dyn_into::<web_sys::HtmlElement>() {
        init_table(el);
    }
}
