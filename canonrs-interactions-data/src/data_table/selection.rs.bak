//! Selection — row selection behavior
//! Core: dom/{state}
use canonrs_interactions_core::dom::lifecycle;
use web_sys::{HtmlElement};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::runtime::context;
use canonrs_interactions_core::dom::state;

pub fn init(table: &HtmlElement) {
    if !lifecycle::init_guard(&table.clone().into()) { return; }
    if table.get_attribute("data-rs-selectable").as_deref() != Some("true") { return; }
    let root: web_sys::Element = table.clone().into();

    // previne seleção de texto com shift+click
    {
        let root_c = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            if e.shift_key() {
                if let Some(t) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) {
                    if t.closest("[data-rs-datatable-row]").ok().flatten().is_some() {
                        e.prevent_default();
                    }
                }
            }
        }));
        let _ = root_c.add_event_listener_with_callback("mousedown", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // select-all
    if let Some(select_all) = root.query_selector("[data-rs-datatable-select-all]").ok().flatten() {
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

    let tbody = match root.query_selector("[data-rs-datatable-body]").ok().flatten() {
        Some(el) => el, None => return,
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
            if t.closest("[data-rs-datatable-action]").ok().flatten().is_some() { return; }
            if t.closest("[data-rs-datatable-actions-cell]").ok().flatten().is_some() { return; }
            let Some(row) = t.closest("[data-rs-datatable-row]").ok().flatten() else { return };
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

    // init bulk bar
    let root: web_sys::Element = table.clone().into();
    if root.get_attribute("data-rs-selected-ids").is_none() {
        let _ = root.set_attribute("data-rs-selected-ids", "");
    }
}

pub fn set_row_selected(row: &web_sys::Element, selected: bool) {
    if selected {
        state::remove(row, "unselected");
        state::set(row, canonrs_interactions_core::dom::state::State::Selected);
    } else {
        state::unset(row, canonrs_interactions_core::dom::state::State::Selected);
        state::add(row, "unselected");
    }
    if let Some(cb) = row.query_selector("[data-rs-datatable-select-row]").ok().flatten()
        .and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok())
    {
        cb.set_checked(selected);
    }
}

pub fn get_row_id(row: &web_sys::Element) -> String {
    row.get_attribute("data-rs-row-id").unwrap_or_default()
}

pub fn get_visible_rows(rc: &web_sys::Element) -> Vec<web_sys::Element> {
    rc.query_selector_all("[data-rs-datatable-row]").ok()
        .map(|list| (0..list.length())
            .filter_map(|i| list.item(i))
            .filter_map(|n| n.dyn_into::<web_sys::Element>().ok())
            .filter(|el| el.get_attribute("hidden").is_none())
            .collect())
        .unwrap_or_default()
}

pub fn get_ordered_ids(rows: &[web_sys::Element]) -> Vec<String> {
    let mut indexed: Vec<(usize, String)> = rows.iter().filter_map(|r| {
        let idx = r.get_attribute("data-rs-row-index").and_then(|s| s.parse::<usize>().ok())?;
        Some((idx, get_row_id(r)))
    }).collect();
    if indexed.is_empty() { return rows.iter().map(|r| get_row_id(r)).collect(); }
    indexed.sort_by_key(|(i, _)| *i);
    indexed.into_iter().map(|(_, id)| id).collect()
}

pub fn sel_ids(root: &web_sys::Element) -> Vec<String> {
    let raw = root.get_attribute("data-rs-selected-ids").unwrap_or_default();
    if raw.starts_with('[') {
        raw.trim_matches(|c| c == '[' || c == ']')
            .split(',')
            .map(|s| s.trim().trim_matches('"').to_string())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        raw.split(',').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect()
    }
}

pub fn sel_is(root: &web_sys::Element, id: &str) -> bool {
    root.get_attribute("data-rs-selected-ids")
        .unwrap_or_default()
        .split(',')
        .any(|s| s == id)
}

pub fn sel_count(root: &web_sys::Element) -> usize {
    sel_ids(root).len()
}

pub fn sel_last(root: &web_sys::Element) -> String {
    root.get_attribute("data-rs-selection-last").unwrap_or_default()
}

pub fn sel_set(root: &web_sys::Element, ids: Vec<String>, last: &str) {
    let quoted: Vec<String> = ids.iter().map(|id| format!("\"{}\"", id)).collect();
    let json = format!("[{}]", quoted.join(","));
    let _ = root.set_attribute("data-rs-selected-ids", &json);
    let _ = root.set_attribute("data-rs-selection-last", last);
}

pub fn sel_add(root: &web_sys::Element, id: &str) {
    let mut ids = sel_ids(root);
    if !ids.iter().any(|s| s == id) { ids.push(id.to_string()); }
    let _ = root.set_attribute("data-rs-selected-ids", &ids.join(","));
    let _ = root.set_attribute("data-rs-selection-last", id);
}

pub fn sel_remove(root: &web_sys::Element, id: &str) {
    let ids: Vec<String> = sel_ids(root).into_iter().filter(|s| s != id).collect();
    let _ = root.set_attribute("data-rs-selected-ids", &ids.join(","));
}

pub fn sel_toggle(root: &web_sys::Element, id: &str) {
    if sel_is(root, id) { sel_remove(root, id); } else { sel_add(root, id); }
    let _ = root.set_attribute("data-rs-selection-last", id);
}

pub fn sel_range(root: &web_sys::Element, ordered_ids: &[String], anchor: &str, to: &str) {
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

pub fn sel_clear(root: &web_sys::Element) {
    let _ = root.set_attribute("data-rs-selected-ids", "");
}

pub fn render_selection(root: &web_sys::Element, rows: &[web_sys::Element]) {
    for row in rows {
        let id = get_row_id(row);
        set_row_selected(row, sel_is(root, &id));
    }
}

pub fn sync_select_all(table: &web_sys::Element) {
    let Ok(rows) = table.query_selector_all("[data-rs-datatable-row]:not([hidden])") else { return };
    let total = rows.length() as usize;
    let selected = (0..rows.length())
        .filter_map(|i| rows.item(i))
        .filter_map(|n| n.dyn_into::<web_sys::Element>().ok())
        .filter(|el| state::has(el, "selected"))
        .count();
    if let Some(cb) = table.query_selector("[data-rs-datatable-select-all]").ok().flatten()
        .and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok())
    {
        cb.set_checked(selected == total && total > 0);
        cb.set_indeterminate(selected > 0 && selected < total);
    }
}

pub fn emit_sel_change(root: &web_sys::Element, action: &str, source: &str) {
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
    let event_init = web_sys::CustomEventInit::new();
    event_init.set_detail(&detail);
    event_init.set_bubbles(true);
    if let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict("rs-selection-change", &event_init) {
        let _ = root.dispatch_event(&event);
    }
    sync_hidden_input(root);
}

pub fn sync_hidden_input(root: &web_sys::Element) {
    let name = match root.get_attribute("data-rs-name") {
        Some(n) if !n.is_empty() => n,
        _ => return,
    };
    let doc = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d, None => return,
    };
    let value = root.get_attribute("data-rs-selected-ids").unwrap_or_default();
    if let Ok(Some(el)) = root.query_selector("input[data-rs-hidden]") {
        if let Ok(input) = el.dyn_into::<web_sys::HtmlInputElement>() {
            input.set_value(&value);
            return;
        }
    }
    if let Ok(el) = doc.create_element("input") {
        if let Ok(input) = el.dyn_into::<web_sys::HtmlInputElement>() {
            let _ = input.set_attribute("type", "hidden");
            let _ = input.set_attribute("data-rs-hidden", "");
            let _ = input.set_attribute("name", &name);
            input.set_value(&value);
            let _ = root.append_child(input.unchecked_ref());
        }
    }
}
