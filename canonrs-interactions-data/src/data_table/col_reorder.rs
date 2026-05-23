//! ColReorder — drag-to-reorder column behavior
//! Core: dom/{state, attrs}
use canonrs_interactions_core::dom::lifecycle;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use crate::runtime::listeners;
use crate::runtime::drag;

pub fn init(table: &HtmlElement) {
    if !lifecycle::init_guard(&table.clone().into()) { return; }
    let uid = table.get_attribute("data-rs-uid").unwrap_or_default();
    let Ok(head) = table.query_selector("[data-rs-datatable-head-row]") else { return };
    let Some(head_row) = head else { return };
    let Ok(head_el) = head_row.clone().dyn_into::<HtmlElement>() else { return };

    let drag_from: std::rc::Rc<std::cell::Cell<Option<usize>>> = std::rc::Rc::new(std::cell::Cell::new(None));
    let drag_over: std::rc::Rc<std::cell::Cell<Option<usize>>> = std::rc::Rc::new(std::cell::Cell::new(None));
    let dragging   = std::rc::Rc::new(std::cell::Cell::new(false));

    {
        let drag_from_md = drag_from.clone();
        let drag_over_md = drag_over.clone();
        let dragging_md  = dragging.clone();
        let head_md      = head_el.clone();
        let table_md     = table.clone();
        let uid_md       = uid.clone();

        listeners::listen(&uid, &head_el.clone().into(), "mousedown", move |e: web_sys::Event| {
            use wasm_bindgen::JsCast;
            let e = match e.dyn_into::<web_sys::MouseEvent>() { Ok(e) => e, Err(_) => return };
            let Some(t) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
            let Some(th) = t.closest("[data-rs-datatable-head-cell]").ok().flatten() else { return };
            if t.has_attribute("data-rs-datatable-resize-handle") { return; }
            let col_index = th.get_attribute("data-rs-col-index").and_then(|s| s.parse::<usize>().ok());
            let Some(idx) = col_index else { return };
            drag_from_md.set(Some(idx));
            dragging_md.set(true);
            let _ = th.set_attribute("data-rs-dragging", "true");

            let drag_over_move = drag_over_md.clone();
            let dragging_move  = dragging_md.clone();
            let head_move      = head_md.clone();
            let drag_from_up   = drag_from_md.clone();
            let drag_over_up   = drag_over_md.clone();
            let dragging_up    = dragging_md.clone();
            let head_up        = head_md.clone();
            let table_up       = table_md.clone();

            drag::start(
                &uid_md,
                move |_dx, _dy, client_x, client_y| {
                    if !dragging_move.get() { return; }
                    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
                    let Some(el_at) = doc.element_from_point(client_x as f32, client_y as f32) else { return };
                    let Some(th) = el_at.closest("[data-rs-datatable-head-cell]").ok().flatten() else { return };
                    let col_index = th.get_attribute("data-rs-col-index").and_then(|s| s.parse::<usize>().ok());
                    if let Ok(cells) = head_move.query_selector_all("[data-rs-drag-over]") {
                        for i in 0..cells.length() {
                            if let Some(el) = cells.item(i).and_then(|n| n.dyn_into::<web_sys::Element>().ok()) {
                                let _ = el.remove_attribute("data-rs-drag-over");
                            }
                        }
                    }
                    if let Some(i) = col_index {
                        drag_over_move.set(Some(i));
                        let _ = th.set_attribute("data-rs-drag-over", "true");
                    }
                },
                move || {
                    dragging_up.set(false);
                    if let Ok(cells) = head_up.query_selector_all("[data-rs-dragging],[data-rs-drag-over]") {
                        for i in 0..cells.length() {
                            if let Some(el) = cells.item(i).and_then(|n| n.dyn_into::<web_sys::Element>().ok()) {
                                let _ = el.remove_attribute("data-rs-dragging");
                                let _ = el.remove_attribute("data-rs-drag-over");
                            }
                        }
                    }
                    let from = drag_from_up.get();
                    let to   = drag_over_up.get();
                    drag_from_up.set(None);
                    drag_over_up.set(None);
                    let (Some(from_idx), Some(to_idx)) = (from, to) else { return };
                    if from_idx == to_idx { return; }
                    reorder_columns(&table_up, from_idx, to_idx);
                    let detail = js_sys::Object::new();
                    let _ = js_sys::Reflect::set(&detail, &wasm_bindgen::JsValue::from_str("from"), &wasm_bindgen::JsValue::from_f64(from_idx as f64));
                    let _ = js_sys::Reflect::set(&detail, &wasm_bindgen::JsValue::from_str("to"), &wasm_bindgen::JsValue::from_f64(to_idx as f64));
                    let init = web_sys::CustomEventInit::new();
                    init.set_bubbles(true);
                    init.set_detail(&wasm_bindgen::JsValue::from(detail));
                    if let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict("rs-datatable-col-reorder", &init) {
                        let root: web_sys::Element = table_up.clone().into();
                        let _ = root.dispatch_event(&event);
                    }
                },
            );
        });
    }
}

fn reorder_columns(table: &HtmlElement, from: usize, to: usize) {
    if let Ok(Some(head_row)) = table.query_selector("[data-rs-datatable-head-row]") {
        let ths: Vec<web_sys::Element> = canonrs_interactions_core::dom::query::all(&head_row, "[data-rs-datatable-head-cell]");
        if from < ths.len() && to < ths.len() {
            let moving = ths[from].clone();
            let target = ths[to].clone();
            if from < to {
                if let Some(next) = target.next_element_sibling() { let _ = head_row.insert_before(&moving, Some(&next)); }
                else { let _ = head_row.append_child(&moving); }
            } else {
                let _ = head_row.insert_before(&moving, Some(&target));
            }
            let updated: Vec<web_sys::Element> = canonrs_interactions_core::dom::query::all(&head_row, "[data-rs-datatable-head-cell]");
            for (i, th) in updated.iter().enumerate() {
                if th.get_attribute("data-rs-col-expand").is_some() || th.get_attribute("data-rs-col-select").is_some() { continue; }
                let _ = th.set_attribute("data-rs-col-index", &i.to_string());
            }
        }
    }
    if let Ok(rows) = table.query_selector_all("[data-rs-datatable-row]") {
        for i in 0..rows.length() {
            let Some(row) = rows.item(i).and_then(|n| n.dyn_into::<web_sys::Element>().ok()) else { continue };
            let tds: Vec<web_sys::Element> = canonrs_interactions_core::dom::query::all(&row, "[data-rs-col-index]");
            if from < tds.len() && to < tds.len() {
                let moving = tds[from].clone();
                let target = tds[to].clone();
                if from < to {
                    if let Some(next) = target.next_element_sibling() { let _ = row.insert_before(&moving, Some(&next)); }
                    else { let _ = row.append_child(&moving); }
                } else {
                    let _ = row.insert_before(&moving, Some(&target));
                }
                let updated: Vec<web_sys::Element> = canonrs_interactions_core::dom::query::all(&row, "[data-rs-col-index]");
                for (j, td) in updated.iter().enumerate() {
                    let _ = td.set_attribute("data-rs-col-index", &j.to_string());
                }
            }
        }
    }
}
