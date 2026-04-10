//! DataTable Interaction — Canon Rule #342
//! Toda a lógica vive aqui. Island apenas chama init_all().
//! DOM é a fonte de verdade via data-rs-* attributes.

use web_sys::{HtmlInputElement, HtmlElement};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::runtime::{lifecycle, state, attrs, context};

// ─── Entry point ─────────────────────────────────────────────────────────────

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-datatable]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<web_sys::Element>() { init(el); }
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
    sync_density_state(&table);
    sync_col_toggle_state(&table);
}

fn sync_col_toggle_state(table: &HtmlElement) {
    let root: web_sys::Element = table.clone().into();
    if let Ok(items) = root.query_selector_all("[data-rs-dropdown-menu-checkbox-item]") {
        for i in 0..items.length() {
            if let Some(el) = items.item(i).and_then(|n| n.dyn_into::<web_sys::Element>().ok()) {
                if !state::has(&el, "checked") && !state::has(&el, "unchecked") {
                    state::add(&el, "checked");
                    let _ = el.set_attribute("aria-checked", "true");
                }
            }
        }
    }
}

fn sync_density_state(table: &HtmlElement) {
    let current = table.get_attribute("data-rs-density").unwrap_or_else(|| "comfortable".to_string());
    let root: web_sys::Element = table.clone().into();
    if let Ok(btns) = root.query_selector_all("[data-rs-density-btn]") {
        for i in 0..btns.length() {
            if let Some(b) = btns.item(i).and_then(|n| n.dyn_into::<web_sys::Element>().ok()) {
                let is_active = b.get_attribute("data-rs-density-btn").as_deref() == Some(current.as_str());
                let _ = b.set_attribute("data-active", if is_active { "true" } else { "false" });
            }
        }
    }
}

// ─── Filter ──────────────────────────────────────────────────────────────────

fn get_filter_input(root: &web_sys::Element) -> Option<HtmlInputElement> {
    root.query_selector("[data-rs-datatable-filter]").ok().flatten()
        .and_then(|el| el.dyn_into::<HtmlInputElement>().ok())
}

fn bind_filter(table: &HtmlElement) {
    let root: web_sys::Element = table.clone().into();
    let Some(input) = get_filter_input(&root) else { return };
    let table_clone = table.clone();

    let cb = Closure::<dyn Fn(web_sys::Event)>::wrap(Box::new(move |e: web_sys::Event| {
        let Some(t) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
        let Some(rc) = context::find_root(&t, "[data-rs-datatable]") else { return };
        let q = get_filter_input(&rc).map(|i| i.value().to_lowercase()).unwrap_or_default();
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("[datatable] filter q='{}'", q)));
        if let Ok(t) = rc.dyn_into::<HtmlElement>() {
            apply_filter(&t, &q);
            set_page(&t, 1);
            update_pagination_ui(&t);
        }
    }));

    let _ = input.add_event_listener_with_callback("input", cb.as_ref().unchecked_ref());
    cb.forget();
}

fn apply_filter(table: &HtmlElement, q: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("[datatable] apply_filter q='{}' connected={}", q, table.is_connected())));
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
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("[datatable] handle_sort col={} connected={}", col_idx, table.is_connected())));
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
    let root: web_sys::Element = table.clone().into();
    let prev = table.query_selector("[data-rs-action='prev']").ok().flatten()
        .and_then(|el| el.dyn_into::<web_sys::Element>().ok());
    let next = table.query_selector("[data-rs-action='next']").ok().flatten()
        .and_then(|el| el.dyn_into::<web_sys::Element>().ok());

    if let Some(btn) = prev {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
            let Some(rc) = context::find_root(&t, "[data-rs-datatable]") else { return };
            if let Ok(tbl) = rc.dyn_into::<HtmlElement>() {
                let p = attrs::get_usize_html(&tbl, "data-rs-current-page", 1);
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
                let p = attrs::get_usize_html(&tbl, "data-rs-current-page", 1);
                let tp = attrs::get_usize_html(&tbl, "data-rs-total-pages", 1);
                if p < tp { set_page(&tbl, p + 1); update_pagination_ui(&tbl); }
            }
        }));
        let _ = btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
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
                if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
                        let Some(t) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
                        let Some(btn) = t.closest("[data-rs-density-btn]").ok().flatten() else { return };
                        let Some(rc) = context::find_root(&btn, "[data-rs-datatable]") else { return };
                        let Some(d) = btn.get_attribute("data-rs-density-btn") else { return };
                        let _ = rc.set_attribute("data-rs-density", &d);
                        if let Ok(all) = rc.query_selector_all("[data-rs-density-btn]") {
                            for j in 0..all.length() {
                                if let Some(b) = all.item(j).and_then(|n| n.dyn_into::<web_sys::Element>().ok()) {
                                    let is_active = b.get_attribute("data-rs-density-btn").as_deref() == Some(d.as_str());
                                    let _ = b.set_attribute("data-active", if is_active { "true" } else { "false" });
                                }
                            }
                        }
                    }));
                    let _ = el.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
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
                if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
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
                    }));
                    let _ = el.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
                    cb.forget();
                }
            }
        }
    }
}


fn toggle_column(table: &HtmlElement, col_idx: usize) {
    // seleciona apenas células e headers da tabela, não checkbox items do dropdown
    let selector = format!(
        "[data-rs-datatable-head-cell][data-rs-col-index='{}'], [data-rs-datatable-cell][data-rs-col-index='{}']",
        col_idx, col_idx
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
    let uid = root.get_attribute("data-rs-uid");
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("[datatable] init called uid={:?}", uid)));
    if !lifecycle::init_guard(&root) { 
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("[datatable] BLOCKED by init_guard"));
        return; 
    }
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("[datatable] init OK uid={}", uid.unwrap_or_default())));
    use wasm_bindgen::JsCast;
    if let Ok(el) = root.dyn_into::<web_sys::HtmlElement>() {
        init_table(el);
    }
}
