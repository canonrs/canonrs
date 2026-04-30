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
    bind_selection(&table);
    bind_bulk_bar(&table);
    bind_context_menu(&table);
    bind_row_actions(&table);
    // inicializa paginação — esconde rows além da página 1
    let total = count_visible(&table);
    let page_size = crate::runtime::attrs::get_usize_html(&table, "data-rs-page-size", 10);
    let total_pages = ((total as f64) / (page_size as f64)).ceil().max(1.0) as usize;
    let _ = table.set_attribute("data-rs-total-pages", &total_pages.to_string());
    set_page(&table, 1);
    update_pagination_ui(&table);
    // garante bulk bar oculta no init (zero seleção)
    let root: web_sys::Element = table.clone().into();
    update_bulk_bar(&root);
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

fn sync_density_state(table: &HtmlElement) {
    let current = table.get_attribute("data-rs-density").unwrap_or_else(|| "comfortable".to_string());
    let root: web_sys::Element = table.clone().into();
    if let Ok(btns) = root.query_selector_all("[data-rs-density-btn]") {
        for i in 0..btns.length() {
            if let Some(b) = btns.item(i).and_then(|n: web_sys::Node| n.dyn_into::<web_sys::Element>().ok()) {
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
    let _table_clone = table.clone();

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
    let _root: web_sys::Element = table.clone().into(); // não usado diretamente
    let prev: Option<web_sys::Element> = table.query_selector("[data-rs-action='prev']").ok().flatten();
    let next: Option<web_sys::Element> = table.query_selector("[data-rs-action='next']").ok().flatten();

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

// ─── Selection ───────────────────────────────────────────────────────────────

fn set_row_selected(row: &web_sys::Element, selected: bool) {
    if selected {
        state::remove(row, "unselected");
        state::add(row, "selected");
    } else {
        state::remove(row, "selected");
        state::add(row, "unselected");
    }
    // sincroniza checkbox da row
    if let Some(cb) = row.query_selector("[data-rs-datatable-select-row]").ok().flatten()
        .and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok())
    {
        cb.set_checked(selected);
    }
}

fn get_selected_count(table: &web_sys::Element) -> (usize, usize) {
    let Ok(rows) = table.query_selector_all("[data-rs-datatable-row]:not([hidden])") else { return (0, 0) };
    let total = rows.length() as usize;
    let selected = (0..rows.length())
        .filter_map(|i| rows.item(i))
        .filter_map(|n| n.dyn_into::<web_sys::Element>().ok())
        .filter(|el| state::has(el, "selected"))
        .count();
    (selected, total)
}

fn sync_select_all(table: &web_sys::Element) {
    let (selected, total) = get_selected_count(table);
    if let Some(cb) = table.query_selector("[data-rs-datatable-select-all]").ok().flatten()
        .and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok())
    {
        cb.set_checked(selected == total && total > 0);
        cb.set_indeterminate(selected > 0 && selected < total);
    }
}








// ─── Selection — DOM como SSOT (padrão select/radio) ────────────────────────

fn get_row_id(row: &web_sys::Element) -> String {
    row.get_attribute("data-rs-row-id").unwrap_or_default()
}

fn get_visible_rows(rc: &web_sys::Element) -> Vec<web_sys::Element> {
    rc.query_selector_all("[data-rs-datatable-row]").ok()
        .map(|list| (0..list.length())
            .filter_map(|i| list.item(i))
            .filter_map(|n| n.dyn_into::<web_sys::Element>().ok())
            .filter(|el| el.get_attribute("hidden").is_none())
            .collect())
        .unwrap_or_default()
}

fn get_ordered_ids(rows: &[web_sys::Element]) -> Vec<String> {
    let mut indexed: Vec<(usize, String)> = rows.iter().filter_map(|r| {
        let idx = r.get_attribute("data-rs-row-index").and_then(|s| s.parse::<usize>().ok())?;
        Some((idx, get_row_id(r)))
    }).collect();
    if indexed.is_empty() { return rows.iter().map(|r| get_row_id(r)).collect(); }
    indexed.sort_by_key(|(i, _)| *i);
    indexed.into_iter().map(|(_, id)| id).collect()
}

// ── estado no DOM do root ────────────────────────────────────────────────────
fn sel_ids(root: &web_sys::Element) -> Vec<String> {
    root.get_attribute("data-rs-selected-ids")
        .unwrap_or_default()
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn sel_is(root: &web_sys::Element, id: &str) -> bool {
    root.get_attribute("data-rs-selected-ids")
        .unwrap_or_default()
        .split(',')
        .any(|s| s == id)
}

fn sel_count(root: &web_sys::Element) -> usize {
    let v = root.get_attribute("data-rs-selected-ids").unwrap_or_default();
    if v.is_empty() { 0 } else { v.split(',').filter(|s| !s.is_empty()).count() }
}

fn sel_last(root: &web_sys::Element) -> String {
    root.get_attribute("data-rs-selection-last").unwrap_or_default()
}

fn sel_set(root: &web_sys::Element, ids: Vec<String>, last: &str) {
    let _ = root.set_attribute("data-rs-selected-ids", &ids.join(","));
    let _ = root.set_attribute("data-rs-selection-last", last);
}

fn sel_add(root: &web_sys::Element, id: &str) {
    let mut ids = sel_ids(root);
    if !ids.iter().any(|s| s == id) { ids.push(id.to_string()); }
    let _ = root.set_attribute("data-rs-selected-ids", &ids.join(","));
    let _ = root.set_attribute("data-rs-selection-last", id);
}

fn sel_remove(root: &web_sys::Element, id: &str) {
    let ids: Vec<String> = sel_ids(root).into_iter().filter(|s| s != id).collect();
    let _ = root.set_attribute("data-rs-selected-ids", &ids.join(","));
}

fn sel_toggle(root: &web_sys::Element, id: &str) {
    if sel_is(root, id) { sel_remove(root, id); } else { sel_add(root, id); }
    let _ = root.set_attribute("data-rs-selection-last", id);
}

fn sel_range(root: &web_sys::Element, ordered_ids: &[String], anchor: &str, to: &str) {
    let from = ordered_ids.iter().position(|id| id == anchor);
    let to_pos = ordered_ids.iter().position(|id| id == to);
    if let (Some(f), Some(t)) = (from, to_pos) {
        let (s, e) = if f <= t { (f, t) } else { (t, f) };
        let mut ids = sel_ids(root);
        for id in &ordered_ids[s..=e] {
            if !ids.iter().any(|s| s == id) { ids.push(id.clone()); }
        }
        let _ = root.set_attribute("data-rs-selected-ids", &ids.join(","));
        let _ = root.set_attribute("data-rs-selection-last", to);
    }
}

fn sel_clear(root: &web_sys::Element) {
    let _ = root.set_attribute("data-rs-selected-ids", "");
}

fn render_selection(root: &web_sys::Element, rows: &[web_sys::Element]) {
    for row in rows {
        let id = get_row_id(row);
        set_row_selected(row, sel_is(root, &id));
    }
}

fn emit_sel_change(root: &web_sys::Element, action: &str, source: &str) {
    let ids = sel_ids(root);
    let detail = js_sys::Object::new();
    let arr = js_sys::Array::new();
    for id in &ids { arr.push(&wasm_bindgen::JsValue::from_str(id)); }
    let _ = js_sys::Reflect::set(&detail, &"selected".into(), &arr);
    let _ = js_sys::Reflect::set(&detail, &"action".into(), &wasm_bindgen::JsValue::from_str(action));
    let _ = js_sys::Reflect::set(&detail, &"source".into(), &wasm_bindgen::JsValue::from_str(source));
    let last = sel_last(root);
    if !last.is_empty() {
        let _ = js_sys::Reflect::set(&detail, &"last".into(), &wasm_bindgen::JsValue::from_str(&last));
    }
    let event_init: web_sys::CustomEventInit = web_sys::CustomEventInit::new();
    event_init.set_detail(&detail);
    event_init.set_bubbles(true);
    if let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict("rs-selection-change", &event_init) {
        let _ = root.dispatch_event(&event);
    }
}

fn bind_selection(table: &HtmlElement) {
    if table.get_attribute("data-rs-selectable").as_deref() != Some("true") { return; }
    let root: web_sys::Element = table.clone().into();

    // select-all
    if let Some(select_all) = root.query_selector("[data-rs-datatable-select-all]").ok().flatten()
        .and_then(|el: web_sys::Element| Some(el))
    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) else { return };
            let Some(rc) = context::find_root(&t.clone().into(), "[data-rs-datatable]") else { return };
            let rows = get_visible_rows(&rc);
            if t.checked() {
                let ids: Vec<String> = rows.iter().map(|r| get_row_id(r)).collect();
                let last = ids.last().cloned().unwrap_or_default();
                sel_set(&rc, ids, &last);
            } else {
                sel_clear(&rc);
            }
            render_selection(&rc, &rows);
            sync_select_all(&rc);
            emit_sel_change(&rc, "select-all", "mouse");
        }));
        let _ = select_all.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    let tbody: web_sys::Element = match root.query_selector("[data-rs-datatable-body]").ok().flatten() {
        Some(el) => el,
        None => return,
    };

    // keyboard
    {
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
            let Some(rc) = context::find_root(&t, "[data-rs-datatable]") else { return };
            let rows = get_visible_rows(&rc);
            if rows.is_empty() { return; }
            let ordered_ids = get_ordered_ids(&rows);
            let focused_id = sel_last(&rc);
            let cur_idx = if focused_id.is_empty() { None }
                else { ordered_ids.iter().position(|id| id == &focused_id) }
                .or_else(|| {
                    web_sys::window().and_then(|w| w.document())
                        .and_then(|d| d.active_element())
                        .and_then(|a| rows.iter().position(|r| r.contains(Some(&a))))
                });
            match e.key().as_str() {
                "ArrowDown" | "ArrowUp" => {
                    e.prevent_default();
                    let next = match (e.key().as_str(), cur_idx) {
                        ("ArrowDown", None) => 0,
                        ("ArrowDown", Some(i)) => (i + 1).min(rows.len() - 1),
                        ("ArrowUp", None) => rows.len() - 1,
                        ("ArrowUp", Some(i)) => if i == 0 { 0 } else { i - 1 },
                        _ => 0,
                    };
                    let next_id = ordered_ids[next].clone();
                    if e.shift_key() {
                        let anchor = sel_last(&rc);
                        let anchor = if anchor.is_empty() { next_id.clone() } else { anchor };
                        sel_range(&rc, &ordered_ids, &anchor, &next_id);
                        render_selection(&rc, &rows);
                        sync_select_all(&rc);
                        emit_sel_change(&rc, "range", "keyboard");
                    } else {
                        let _ = rc.set_attribute("data-rs-selection-last", &next_id);
                        if let Ok(el) = rows[next].clone().dyn_into::<web_sys::HtmlElement>() { let _ = el.focus(); }
                    }
                }
                " " => {
                    e.prevent_default();
                    if let Some(idx) = cur_idx {
                        let id = ordered_ids[idx].clone();
                        sel_toggle(&rc, &id);
                        render_selection(&rc, &rows);
                        sync_select_all(&rc);
                        emit_sel_change(&rc, "toggle", "keyboard");
                    }
                }
                "Escape" => {
                    sel_clear(&rc);
                    render_selection(&rc, &rows);
                    sync_select_all(&rc);
                    emit_sel_change(&rc, "clear", "keyboard");
                }
                _ => {}
            }
        }));
        let _ = tbody.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // click
    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
            if t.has_attribute("data-rs-datatable-select-all") { return; }
            if t.closest("[data-rs-datatable-head-cell]").ok().flatten().is_some() { return; }
            if t.closest("[data-rs-action]").ok().flatten().is_some() { return; }
            if t.closest("[data-rs-density-btn]").ok().flatten().is_some() { return; }
            if t.closest("[data-rs-dropdown-menu]").ok().flatten().is_some() { return; }

            if t.has_attribute("data-rs-datatable-select-row") {
                e.stop_propagation();
                let Some(row) = t.closest("[data-rs-datatable-row]").ok().flatten() else { web_sys::console::log_1(&"[ctx] no row found".into()); return };
                let Some(rc) = context::find_root(&row, "[data-rs-datatable]") else { return };
                let rows = get_visible_rows(&rc);
                let id = get_row_id(&row);
                let checked = t.clone().dyn_into::<web_sys::HtmlInputElement>().ok()
                    .map(|cb| cb.checked()).unwrap_or(false);
                if checked { sel_add(&rc, &id); } else { sel_remove(&rc, &id); }
                render_selection(&rc, &rows);
                sync_select_all(&rc);
                emit_sel_change(&rc, "toggle", "checkbox");
                return;
            }

            let Some(row) = t.closest("[data-rs-datatable-row]").ok().flatten() else { web_sys::console::log_1(&"[ctx] no row found".into()); return };
            let Some(rc) = context::find_root(&row, "[data-rs-datatable]") else { return };
            let rows = get_visible_rows(&rc);
            let ordered_ids = get_ordered_ids(&rows);
            let id = get_row_id(&row);
            if id.is_empty() { return; }

            let is_single = rc.get_attribute("data-rs-select-mode").as_deref() == Some("single");

            if is_single {
                let was_only = sel_is(&rc, &id) && sel_count(&rc) == 1;
                sel_clear(&rc);
                if !was_only { sel_add(&rc, &id); }
            } else if e.shift_key() {
                let anchor = sel_last(&rc);
                let anchor = if anchor.is_empty() { id.clone() } else { anchor };
                sel_range(&rc, &ordered_ids, &anchor, &id);
            } else if e.ctrl_key() || e.meta_key() {
                sel_toggle(&rc, &id);
            } else {
                let was_only = sel_is(&rc, &id) && sel_count(&rc) == 1;
                sel_clear(&rc);
                if !was_only { sel_add(&rc, &id); }
            }

            render_selection(&rc, &rows);
            sync_select_all(&rc);
            emit_sel_change(&rc, "click", "mouse");
        }));
        let _ = tbody.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}



#[allow(dead_code)]
fn root_el(table: &HtmlElement) -> web_sys::Element {
    table.clone().into()
}












// ─── Row Actions ─────────────────────────────────────────────────────────────

fn bind_row_actions(table: &HtmlElement) {
    let _root: web_sys::Element = table.clone().into();
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
        let Some(t) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
        // encontra o elemento com data-rs-datatable-action (pode ser o span interno)
        let action_el = if t.has_attribute("data-rs-datatable-action") {
            Some(t.clone())
        } else {
            t.closest("[data-rs-datatable-action]").ok().flatten()
        };
        let Some(action_el) = action_el else { return };
        let Some(action) = action_el.get_attribute("data-rs-datatable-action") else { return };
        let row_id = action_el.get_attribute("data-rs-row-id").unwrap_or_default();
        // encontra o datatable root a partir do action_el ou via document
        let rc = context::find_root(&action_el, "[data-rs-datatable]")
            .or_else(|| {
                web_sys::window().and_then(|w| w.document())
                    .and_then(|d| d.query_selector("[data-rs-datatable]").ok().flatten())
            });
        let Some(rc) = rc else { return };
        // busca label na row pelo row_id
        let row_label = rc.query_selector(&format!("[data-rs-datatable-row][data-rs-row-id='{}']", row_id))
            .ok().flatten()
            .and_then(|row| row.get_attribute("data-rs-row-label"))
            .unwrap_or_default();
        // propaga contexto no root — DOM como fonte de verdade
        let _ = rc.set_attribute("data-rs-current-action", &action);
        let _ = rc.set_attribute("data-rs-current-row", &row_id);
        let _ = rc.set_attribute("data-rs-current-label", &row_label);
        // dispara evento para a página ouvir
        let detail = js_sys::Object::new();
        let _ = js_sys::Reflect::set(&detail, &"action".into(), &wasm_bindgen::JsValue::from_str(&action));
        let _ = js_sys::Reflect::set(&detail, &"rowId".into(), &wasm_bindgen::JsValue::from_str(&row_id));
        let event_init = web_sys::CustomEventInit::new();
        event_init.set_detail(&detail);
        event_init.set_bubbles(true);
        if let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict("rs-datatable-action", &event_init) {
            let _ = rc.dispatch_event(&event);
        }
    }));
    // usa capture=true para interceptar antes do stop_propagation do DropdownMenu
    use wasm_bindgen::JsValue;
    let opts = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&opts, &JsValue::from_str("capture"), &JsValue::from_bool(true));
    let _ = doc.add_event_listener_with_callback_and_bool(
        "click",
        cb.as_ref().unchecked_ref(),
        true,
    );
    cb.forget();
}

// ─── Context Menu ────────────────────────────────────────────────────────────

fn bind_context_menu(_table: &HtmlElement) {
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };

    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
            let Some(row) = t.closest("[data-rs-datatable-row]").ok().flatten() else { return };
            let Some(rc) = context::find_root(&row, "[data-rs-datatable]") else { return };
            let row_id = row.get_attribute("data-rs-row-id").unwrap_or_default();
            let selector = format!("[data-rs-datatable-row-context][data-rs-row-id='{}']", row_id);
            let Some(ctx_root) = rc.query_selector(&selector).ok().flatten() else { return };
            let Some(content) = ctx_root.query_selector("[data-rs-context-menu-content]").ok().flatten() else { return };

            e.prevent_default();

            // fecha outros context menus abertos
            let Some(win) = web_sys::window() else { return };
            let Some(d) = win.document() else { return };
            if let Ok(list) = d.query_selector_all("[data-rs-context-menu][data-rs-state~='open']") {
                for i in 0..list.length() {
                    if let Some(el) = list.item(i).and_then(|n| n.dyn_into::<web_sys::Element>().ok()) {
                        state::remove(&el, "open");
                        state::add(&el, "closed");
                    }
                }
            }

            // posiciona via CSS variables no ctx_root
            let x = e.client_x();
            let y = e.client_y();
            if let Ok(el) = ctx_root.clone().dyn_into::<web_sys::HtmlElement>() {
                let _ = el.style().set_property("--context-menu-x", &format!("{}px", x));
                let _ = el.style().set_property("--context-menu-y", &format!("{}px", y));
            }

            // abre
            state::remove(&ctx_root, "closed");
            state::add(&ctx_root, "open");
            let _ = content.remove_attribute("hidden");
        }));
        let _ = doc.add_event_listener_with_callback("contextmenu", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // click fora — fecha
    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let Some(win) = web_sys::window() else { return };
            let Some(d) = win.document() else { return };
            let Ok(list) = d.query_selector_all("[data-rs-context-menu][data-rs-state~='open']") else { return };
            for i in 0..list.length() {
                if let Some(el) = list.item(i).and_then(|n| n.dyn_into::<web_sys::Element>().ok()) {
                    state::remove(&el, "open");
                    state::add(&el, "closed");
                }
            }
        }));
        let _ = doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}




// ─── Bulk Action Bar ─────────────────────────────────────────────────────────

fn update_bulk_bar(root: &web_sys::Element) {
    let ids = root.get_attribute("data-rs-selected-ids").unwrap_or_default();
    let count = ids.split(',').filter(|s| !s.is_empty()).count();
    let bar = root.query_selector("[data-rs-datatable-bulk-bar]").ok().flatten();
    if let Some(bar) = bar {
        if count > 0 {
            let _ = bar.remove_attribute("hidden");
        } else {
            let _ = bar.set_attribute("hidden", "");
        }
        if let Some(counter) = bar.query_selector("[data-rs-datatable-bulk-count]").ok().flatten() {
            counter.set_text_content(Some(&format!("{} selected", count)));
        }
    }
}

fn bind_bulk_bar(table: &HtmlElement) {
    let root: web_sys::Element = table.clone().into();

    // listener no rs-selection-change — usa target para encontrar root correto
    {
        let cb = Closure::<dyn Fn(web_sys::Event)>::wrap(Box::new(move |e: web_sys::Event| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
            update_bulk_bar(&target);
        }));
        let _ = root.add_event_listener_with_callback("rs-selection-change", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // bulk clear button
    if let Some(clear_btn) = root.query_selector("[data-rs-datatable-bulk-clear]").ok().flatten()
        .and_then(|el: web_sys::Element| Some(el))
    {
        let root_c = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let _ = root_c.set_attribute("data-rs-selected-ids", "");
            let Ok(rows) = root_c.query_selector_all("[data-rs-datatable-row]") else { return };
            for i in 0..rows.length() {
                if let Some(row) = rows.item(i).and_then(|n: web_sys::Node| n.dyn_into::<web_sys::Element>().ok()) {
                    set_row_selected(&row, false);
                }
            }
            sync_select_all(&root_c);
            update_bulk_bar(&root_c);
            if let Ok(event) = web_sys::CustomEvent::new("rs-selection-change") {
                let _ = root_c.dispatch_event(&event);
            }
        }));
        let _ = clear_btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
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
    web_sys::console::log_1(&"NEW VERSION LOADED".into());
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
