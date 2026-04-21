//! Table Init — hover, copy, truncate, odd/even, sort, select, keyboard

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state, query, keyboard};

fn get_rows(root: &Element) -> Vec<Element> {
    query::all(root, "[data-rs-table-body] [data-rs-table-row]")
}

fn select_row(row: &Element) {
    state::remove_state(row, "unselected");
    state::add_state(row, "selected");
    let _ = row.set_attribute("aria-selected", "true");
}

fn deselect_row(row: &Element) {
    state::remove_state(row, "selected");
    state::add_state(row, "unselected");
    let _ = row.set_attribute("aria-selected", "false");
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

fn apply_odd_even(rows: &[Element]) {
    for (i, row) in rows.iter().enumerate() {
        if i % 2 == 0 {
            state::remove_state(row, "odd");
            state::add_state(row, "even");
        } else {
            state::remove_state(row, "even");
            state::add_state(row, "odd");
        }
    }
}

fn sort_rows(root: &Element, col_index: usize, direction: &str) {
    let tbody = match query::first(root, "[data-rs-table-body]") {
        Some(el) => el,
        None => return,
    };
    let mut rows = get_rows(root);
    rows.sort_by(|a, b| {
        let val = |row: &Element| -> String {
            query::all(row, "[data-rs-table-cell]")
                .get(col_index)
                .map(|cell| cell.text_content().unwrap_or_default().trim().to_string())
                .unwrap_or_default()
        };
        let va = val(a);
        let vb = val(b);
        let numeric = va.parse::<f64>().ok().zip(vb.parse::<f64>().ok());
        let ord = if let Some((na, nb)) = numeric {
            na.partial_cmp(&nb).unwrap_or(std::cmp::Ordering::Equal)
        } else {
            va.cmp(&vb)
        };
        if direction == "descending" { ord.reverse() } else { ord }
    });
    for row in &rows {
        let _ = tbody.append_child(row);
    }
    apply_odd_even(&rows);
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    // sheet context
    if root.has_attribute("data-rs-table-context") {
        crate::table_row_sheet_preview::init(root.clone());
    }

    // odd/even inicial
    apply_odd_even(&get_rows(&root));

    // truncate tooltip
    for cell in query::all(&root, "[data-rs-truncate]") {
        let text = cell.text_content().unwrap_or_default().trim().to_string();
        if !text.is_empty() { let _ = cell.set_attribute("title", &text); }
    }

    // hover — cell truncate ativa o hover da ROW e da CELL
    for cell in query::all(&root, "[data-rs-truncate]") {
        let c_enter = cell.clone();
        let enter = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
            state::add_state(&c_enter, "hover");
            if let Some(row) = c_enter.closest("[data-rs-table-row]").ok().flatten() {
                state::add_state(&row, "hover");
            }
        });
        let c_leave = cell.clone();
        let leave = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
            state::remove_state(&c_leave, "hover");
            if let Some(row) = c_leave.closest("[data-rs-table-row]").ok().flatten() {
                state::remove_state(&row, "hover");
            }
        });
        let _ = cell.add_event_listener_with_callback("mouseenter", enter.as_ref().unchecked_ref());
        let _ = cell.add_event_listener_with_callback("mouseleave", leave.as_ref().unchecked_ref());
        enter.forget();
        leave.forget();
    }

    // hover — cell copyable
    for cell in query::all(&root, "[data-rs-copyable]") {
        let c_enter = cell.clone();
        let enter = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
            state::add_state(&c_enter, "hover");
        });
        let c_leave = cell.clone();
        let leave = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
            state::remove_state(&c_leave, "hover");
        });
        let _ = cell.add_event_listener_with_callback("mouseenter", enter.as_ref().unchecked_ref());
        let _ = cell.add_event_listener_with_callback("mouseleave", leave.as_ref().unchecked_ref());
        enter.forget();
        leave.forget();
    }

    // copy
    for cell in query::all(&root, "[data-rs-copyable]") {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            e.stop_propagation();
            let Some(t) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let text = t.text_content().unwrap_or_default().trim().to_string();
            if let Some(win) = web_sys::window() {
                let _ = win.navigator().clipboard().write_text(&text);
                state::add_state(&t, "copied");
                let _ = t.set_attribute("data-rs-copy-label", "Copied!");
                let t2 = t.clone();
                let cb2 = Closure::once(move || {
                    state::remove_state(&t2, "copied");
                    let _ = t2.remove_attribute("data-rs-copy-label");
                });
                let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(cb2.as_ref().unchecked_ref(), 1500);
                cb2.forget();
            }
        });
        let _ = cell.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // click — sort + select
    {
        let root_c = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if t.closest("[data-rs-copyable]").ok().flatten().is_some() { return; }
            let Some(rc) = t.closest("[data-rs-table]").ok().flatten() else { return };

            if let Some(th) = t.closest("[data-rs-table-head]").ok().flatten() {
                let next = match th.get_attribute("data-rs-sort").as_deref() {
                    Some("ascending") => "descending",
                    _ => "ascending",
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
                // navigate action
                if row.get_attribute("data-rs-action").as_deref() == Some("navigate") {
                    if let Some(href) = row.get_attribute("data-rs-href") {
                        if let Some(win) = web_sys::window() {
                            let _ = win.location().set_href(&href);
                        }
                    }
                    return;
                }
                if e.shift_key() {
                    let rows = get_rows(&rc);
                    let focused = focused_row_index(&rows);
                    let clicked = rows.iter().position(|r| *r == row);
                    if let (Some(from), Some(to)) = (focused, clicked) {
                        let (start, end) = if from <= to { (from, to) } else { (to, from) };
                        for r in &rows[start..=end] { select_row(r); }
                    }
                } else if e.ctrl_key() || e.meta_key() {
                    if row.get_attribute("data-rs-state").unwrap_or_default().contains("selected") {
                        deselect_row(&row);
                    } else {
                        select_row(&row);
                    }
                } else {
                    deselect_all(&rc);
                    select_row(&row);
                }
            }
        }));
        let _ = root_c.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // captura setas antes do scroll — prevent_default na fase de captura
    {
        let root_cap = root.clone();
        let cap = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if t.closest("[data-rs-table-row]").ok().flatten().is_none() { return; }
            match e.key().as_str() {
                "ArrowUp" | "ArrowDown" | "Home" | "End" | "Enter" | " " => {
                    e.prevent_default();
                }
                _ => {}
            }
        });
        let _ = root_cap.add_event_listener_with_callback_and_bool(
            "keydown", cap.as_ref().unchecked_ref(), true,
        );
        cap.forget();
    }

    // keyboard — via runtime::keyboard::init_nav
    {
        let root_kb = root.clone();
        let on_enter = {
            Box::new(move |idx: usize, items: &[Element]| {
                if let Some(row) = items.get(idx) {
                    // simula click — sheet handler cuida do select
                    if row.get_attribute("data-rs-action").as_deref() == Some("navigate") {
                        if let Some(href) = row.get_attribute("data-rs-href") {
                            if let Some(win) = web_sys::window() {
                                let _ = win.location().set_href(&href);
                            }
                        }
                    } else if row.get_attribute("data-rs-action").as_deref() == Some("open-sheet") {
                        if let Some(win) = web_sys::window() {
                            if win.document().is_some() {
                                if let Some(doc) = win.document() {
                                    if let Ok(event) = doc.create_event("MouseEvents") {
                                        let _ = event.init_event_with_bubbles_and_cancelable("click", true, true);
                                        let _ = row.dispatch_event(&event);
                                    }
                                }
                            }
                        }
                    } else {
                        select_row(row);
                    }
                }
            })
        };
        let on_escape = Box::new(move || {}) as Box<dyn Fn() + 'static>;
        let current_idx = keyboard::init_nav(
            &root_kb,
            "[data-rs-table-body] [data-rs-table-row]",
            keyboard::NavConfig {
                orientation: keyboard::Orientation::Vertical,
                element_type: keyboard::ElementType::Button,
                focus_state: "focused",
                wrap: false,
            },
            Some(on_enter),
            Some(on_escape),
        );

        // sincroniza current_idx quando foco volta para uma row (ex: após fechar sheet)
        for (i, row) in get_rows(&root_kb).iter().enumerate() {
            let idx_sync = current_idx.clone();
            let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::new(move |_: web_sys::FocusEvent| {
                idx_sync.set(Some(i));
            });
            let _ = row.add_event_listener_with_callback("focus", cb.as_ref().unchecked_ref());
            cb.forget();
        }
    }
}
