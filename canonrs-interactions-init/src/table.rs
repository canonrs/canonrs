//! Table Init

use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::{state, query};
use canonrs_interactions_core::runtime::{listeners, timers};
use crate::runtime::keyboard;

fn get_rows(root: &Element) -> Vec<Element> {
    query::all(root, "[data-rs-table-body] [data-rs-table-row]")
}

fn select_row(row: &Element) {
    state::remove_state(row, "unselected"); state::add_state(row, "selected");
    let _ = row.set_attribute("aria-selected", "true");
}

fn deselect_row(row: &Element) {
    state::remove_state(row, "selected"); state::add_state(row, "unselected");
    let _ = row.set_attribute("aria-selected", "false");
}

fn deselect_all(root: &Element) {
    for row in get_rows(root) { deselect_row(&row); }
}

fn apply_odd_even(rows: &[Element]) {
    for (i, row) in rows.iter().enumerate() {
        if i % 2 == 0 { state::remove_state(row, "odd");  state::add_state(row, "even"); }
        else           { state::remove_state(row, "even"); state::add_state(row, "odd"); }
    }
}

fn sort_rows(root: &Element, col_index: usize, direction: &str) {
    let tbody = match query::first(root, "[data-rs-table-body]") { Some(el) => el, None => return };
    let mut rows = get_rows(root);
    rows.sort_by(|a, b| {
        let val = |row: &Element| -> String {
            query::all(row, "[data-rs-table-cell]").get(col_index)
                .map(|cell| cell.text_content().unwrap_or_default().trim().to_string())
                .unwrap_or_default()
        };
        let (va, vb) = (val(a), val(b));
        let ord = va.parse::<f64>().ok().zip(vb.parse::<f64>().ok())
            .map(|(na, nb)| na.partial_cmp(&nb).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or_else(|| va.cmp(&vb));
        if direction == "descending" { ord.reverse() } else { ord }
    });
    for row in &rows { let _ = tbody.append_child(row); }
    apply_odd_even(&rows);
}

pub fn init(root: Element) {
    if root.has_attribute("data-rs-table-context") { crate::table_row_sheet_preview::init(root.clone()); }
    apply_odd_even(&get_rows(&root));
    for cell in query::all(&root, "[data-rs-truncate]") {
        let text = cell.text_content().unwrap_or_default().trim().to_string();
        if !text.is_empty() { let _ = cell.set_attribute("title", &text); }
    }

    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

    // hover — truncate cells
    for (i, cell) in query::all(&root, "[data-rs-truncate]").iter().enumerate() {
        let uid_c = format!("{}:trunc:{}", uid, i);
        listeners::listen(&uid_c, cell, "mouseenter", {
            let c = cell.clone();
            move |_: web_sys::Event| {
                state::add_state(&c, "hover");
                if let Some(row) = c.closest("[data-rs-table-row]").ok().flatten() { state::add_state(&row, "hover"); }
            }
        });
        listeners::listen(&uid_c, cell, "mouseleave", {
            let c = cell.clone();
            move |_: web_sys::Event| {
                state::remove_state(&c, "hover");
                if let Some(row) = c.closest("[data-rs-table-row]").ok().flatten() { state::remove_state(&row, "hover"); }
            }
        });
    }

    // hover — copyable cells
    for (i, cell) in query::all(&root, "[data-rs-copyable]").iter().enumerate() {
        let uid_c = format!("{}:copy:{}", uid, i);
        listeners::listen(&uid_c, cell, "mouseenter", {
            let c = cell.clone();
            move |_: web_sys::Event| { state::add_state(&c, "hover"); }
        });
        listeners::listen(&uid_c, cell, "mouseleave", {
            let c = cell.clone();
            move |_: web_sys::Event| { state::remove_state(&c, "hover"); }
        });
        // copy click
        listeners::listen(&uid_c, cell, "click", {
            let c = cell.clone();
            move |e: web_sys::Event| {
                e.stop_propagation();
                let text = c.text_content().unwrap_or_default().trim().to_string();
                if let Some(win) = web_sys::window() {
                    let _ = win.navigator().clipboard().write_text(&text);
                    state::add_state(&c, "copied");
                    let _ = c.set_attribute("data-rs-copy-label", "Copied!");
                    let c2 = c.clone();
                    timers::timeout(1500, move || {
                        state::remove_state(&c2, "copied");
                        let _ = c2.remove_attribute("data-rs-copy-label");
                    });
                }
            }
        });
    }

    // click — sort + select
    listeners::listen(&uid, &root, "click", {
        move |e: web_sys::Event| {
            let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if t.closest("[data-rs-copyable]").ok().flatten().is_some() { return; }
            let Some(rc) = t.closest("[data-rs-table]").ok().flatten() else { return };

            if let Some(th) = t.closest("[data-rs-table-head]").ok().flatten() {
                let next = match th.get_attribute("data-rs-sort").as_deref() {
                    Some("ascending") => "descending", _ => "ascending",
                };
                for n in query::all(&rc, "[data-rs-table-head]") {
                    let _ = n.set_attribute("data-rs-sort", "none");
                    let _ = n.set_attribute("aria-sort", "none");
                }
                let _ = th.set_attribute("data-rs-sort", next);
                let _ = th.set_attribute("aria-sort", next);
                let heads = query::all(&rc, "[data-rs-table-head]");
                let col_index = heads.iter().position(|h| *h == th).unwrap_or(0);
                sort_rows(&rc, col_index, next);
                return;
            }

            if let Some(row) = t.closest("[data-rs-table-row]").ok().flatten() {
                if row.get_attribute("data-rs-state").unwrap_or_default().contains("disabled") { return; }
                if row.get_attribute("data-rs-action").as_deref() == Some("navigate") {
                    if let Some(href) = row.get_attribute("data-rs-href") {
                        if let Some(win) = web_sys::window() { let _ = win.location().set_href(&href); }
                    }
                    return;
                }
                let rows = get_rows(&rc);
                if e.shift_key() {
                    let focused = rows.iter().position(|r| {
                        r.owner_document().and_then(|d| d.active_element()).map(|ae| ae == *r).unwrap_or(false)
                    });
                    let clicked = rows.iter().position(|r| *r == row);
                    if let (Some(from), Some(to)) = (focused, clicked) {
                        let (start, end) = if from <= to { (from, to) } else { (to, from) };
                        for r in &rows[start..=end] { select_row(r); }
                    }
                } else if e.ctrl_key() || e.meta_key() {
                    if row.get_attribute("data-rs-state").unwrap_or_default().contains("selected") { deselect_row(&row); }
                    else { select_row(&row); }
                } else {
                    deselect_all(&rc); select_row(&row);
                }
            }
        }
    });

    // capture arrow keys
    listeners::listen_opts(
        &uid, &root.clone().unchecked_into::<web_sys::EventTarget>(),
        "keydown",
        canonrs_interactions_core::runtime::listeners::ListenOpts { capture: true, passive: false },
        move |e: web_sys::Event| {
            let e = e.dyn_into::<web_sys::KeyboardEvent>().unwrap();
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if t.closest("[data-rs-table-row]").ok().flatten().is_none() { return; }
            match e.key().as_str() {
                "ArrowUp" | "ArrowDown" | "Home" | "End" | "Enter" | " " => { e.prevent_default(); }
                _ => {}
            }
        }
    );

    // keyboard nav
    let root_kb = root.clone();
    let current_idx = keyboard::init_nav(
        &root_kb,
        "[data-rs-table-body] [data-rs-table-row]",
        keyboard::NavConfig {
            orientation: keyboard::Orientation::Vertical,
            element_type: keyboard::ElementType::Button,
            focus_state: "focused",
            wrap: false,
        },
        Some(Box::new(move |idx, items| {
            if let Some(row) = items.get(idx) {
                if row.get_attribute("data-rs-action").as_deref() == Some("navigate") {
                    if let Some(href) = row.get_attribute("data-rs-href") {
                        if let Some(win) = web_sys::window() { let _ = win.location().set_href(&href); }
                    }
                } else if row.get_attribute("data-rs-action").as_deref() == Some("open-sheet") {
                    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                        if let Ok(event) = doc.create_event("MouseEvents") {
                            let _ = event.init_event_with_bubbles_and_cancelable("click", true, true);
                            let _ = row.dispatch_event(&event);
                        }
                    }
                } else {
                    select_row(row);
                }
            }
        })),
        Some(Box::new(|| {})),
    );

    for (i, row) in get_rows(&root).iter().enumerate() {
        let uid_row = format!("{}:row:{}", uid, i);
        let idx_sync = current_idx.clone();
        listeners::listen(&uid_row, row, "focus", move |_: web_sys::Event| {
            idx_sync.set(Some(i));
        });
    }
}
