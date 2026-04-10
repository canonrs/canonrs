//! Table Interaction Engine — sort + row selection + keyboard navigation

use wasm_bindgen::prelude::*;
use crate::runtime::{lifecycle, state, context};
use wasm_bindgen::JsCast;
use web_sys::Element;

fn get_rows(root: &Element) -> Vec<Element> {
    let Ok(nodes) = root.query_selector_all("[data-rs-table-row]") else { return vec![] };
    (0..nodes.length())
        .filter_map(|i| nodes.item(i))
        .filter_map(|n| n.dyn_into::<Element>().ok())
        .collect()
}

fn select_row(row: &Element) {
    state::remove(row, "unselected");
    state::add(row, "selected");
    let _ = row.set_attribute("aria-selected", "true");
}

fn deselect_row(row: &Element) {
    state::remove(row, "selected");
    state::add(row, "unselected");
    let _ = row.set_attribute("aria-selected", "false");
}

fn toggle_row(row: &Element) {
    if state::has(row, "selected") { deselect_row(row); } else { select_row(row); }
}

fn deselect_all(root: &Element) {
    for row in get_rows(root) { deselect_row(&row); }
}

fn focused_row_index(rows: &[Element]) -> Option<usize> {
    rows.iter().position(|el| {
        el.owner_document()
            .and_then(|d| d.active_element())
            .map(|ae| ae == *el)
            .unwrap_or(false)
    })
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    context::propagate_owner(&root);

    // click — sort on head, select on row
    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(rc) = context::find_root(&t, "[data-rs-table]") else { return };

            // sort — clique no header
            if let Ok(Some(th)) = t.closest("[data-rs-table-head]") {
                let next = match th.get_attribute("data-rs-sort").as_deref() {
                    Some("ascending") => "descending",
                    Some("descending") | Some("none") | None => "ascending",
                    _ => "ascending",
                };
                // reset todos os heads
                if let Ok(heads) = rc.query_selector_all("[data-rs-table-head]") {
                    for i in 0..heads.length() {
                        if let Some(n) = heads.item(i).and_then(|n| n.dyn_into::<Element>().ok()) {
                            let _ = n.set_attribute("data-rs-sort", "none");
                            let _ = n.set_attribute("aria-sort", "none");
                        }
                    }
                }
                let _ = th.set_attribute("data-rs-sort", next);
                let _ = th.set_attribute("aria-sort", next);
                return;
            }

            // select — clique na row
            if let Ok(Some(row)) = t.closest("[data-rs-table-row]") {
                if state::has(&row, "disabled") { return; }
                // shift+click — seleção múltipla
                if e.shift_key() {
                    let rows = get_rows(&rc);
                    let focused = focused_row_index(&rows);
                    let clicked = rows.iter().position(|r| *r == row);
                    if let (Some(from), Some(to)) = (focused, clicked) {
                        let (start, end) = if from <= to { (from, to) } else { (to, from) };
                        for r in &rows[start..=end] { select_row(r); }
                    } else {
                        toggle_row(&row);
                    }
                } else if e.ctrl_key() || e.meta_key() {
                    toggle_row(&row);
                } else {
                    deselect_all(&rc);
                    select_row(&row);
                }
            }
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // keydown — ArrowUp/Down navega, Enter/Space seleciona, Escape deseleciona
    {
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(rc) = context::find_root(&t, "[data-rs-table]") else { return };
            if t.closest("[data-rs-table-row]").ok().flatten().is_none() { return; }
            let rows = get_rows(&rc);
            let len = rows.len();
            if len == 0 { return; }
            let pos = focused_row_index(&rows);
            match e.key().as_str() {
                "ArrowDown" => {
                    e.prevent_default();
                    if let Some(p) = pos {
                        let next = (p + 1).min(len - 1);
                        if let Ok(el) = rows[next].clone().dyn_into::<web_sys::HtmlElement>() { let _ = el.focus(); }
                    }
                }
                "ArrowUp" => {
                    e.prevent_default();
                    if let Some(p) = pos {
                        let prev = if p == 0 { 0 } else { p - 1 };
                        if let Ok(el) = rows[prev].clone().dyn_into::<web_sys::HtmlElement>() { let _ = el.focus(); }
                    }
                }
                "Enter" | " " => {
                    e.prevent_default();
                    if let Some(p) = pos {
                        if !state::has(&rows[p], "disabled") {
                            deselect_all(&rc);
                            select_row(&rows[p]);
                        }
                    }
                }
                "Escape" => {
                    e.prevent_default();
                    deselect_all(&rc);
                }
                _ => {}
            }
        }));
        let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
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
