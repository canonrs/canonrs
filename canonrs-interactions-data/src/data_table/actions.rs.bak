//! Row actions, bulk actions, context menu, bulk bar
//! @domain: action-dispatch + command-orchestration

use web_sys::HtmlElement;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::runtime::context;
use canonrs_interactions_core::dom::state;
use super::selection::{sel_ids, set_row_selected, sync_select_all};

pub fn init(table: &HtmlElement) {
    bind_bulk_bar(table);
    bind_context_menu(table);
    bind_row_actions(table);
    bind_bulk_actions(table);
}

fn bind_bulk_actions(table: &HtmlElement) {
    let root: web_sys::Element = table.clone().into();
    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
        let Some(t) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
        let action_el = if t.has_attribute("data-rs-datatable-bulk-action") { Some(t.clone()) }
            else { t.closest("[data-rs-datatable-bulk-action]").ok().flatten() };
        let Some(action_el) = action_el else { return };
        let Some(action) = action_el.get_attribute("data-rs-datatable-bulk-action") else { return };
        let Some(rc) = context::find_root(&action_el, "[data-rs-datatable]") else { return };
        let ids = sel_ids(&rc);
        let uid = rc.get_attribute("data-rs-uid").unwrap_or_default();
        let _ = rc.set_attribute("data-rs-current-bulk-action", &action);
        let detail = js_sys::Object::new();
        let _ = js_sys::Reflect::set(&detail, &"action".into(), &wasm_bindgen::JsValue::from_str(&action));
        let _ = js_sys::Reflect::set(&detail, &"uid".into(), &wasm_bindgen::JsValue::from_str(&uid));
        let arr = js_sys::Array::new();
        for id in &ids { arr.push(&wasm_bindgen::JsValue::from_str(id)); }
        let _ = js_sys::Reflect::set(&detail, &"ids".into(), &arr);
        let event_init = web_sys::CustomEventInit::new();
        event_init.set_detail(&detail);
        event_init.set_bubbles(true);
        if let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict("rs-datatable-bulk-action", &event_init) {
            let _ = rc.dispatch_event(&event);
        }
    }));
    let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
    cb.forget();
}

fn bind_row_actions(_table: &HtmlElement) {
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
        let Some(t) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
        let has_action = t.has_attribute("data-rs-datatable-action") ||
            t.closest("[data-rs-datatable-action]").ok().flatten().is_some();
        if !has_action { return; }
        let action_el = if t.has_attribute("data-rs-datatable-action") { Some(t.clone()) }
            else { t.closest("[data-rs-datatable-action]").ok().flatten() };
        let Some(action_el) = action_el else { return };
        let Some(action) = action_el.get_attribute("data-rs-datatable-action") else { return };
        e.stop_immediate_propagation();
        let row_id = action_el.get_attribute("data-rs-row-id").unwrap_or_default();
        let rc = context::find_root(&action_el, "[data-rs-datatable]")
            .or_else(|| web_sys::window().and_then(|w| w.document())
                .and_then(|d| d.query_selector("[data-rs-datatable]").ok().flatten()));
        let Some(rc) = rc else { return };
        let row_label = rc.query_selector(&format!("[data-rs-datatable-row][data-rs-row-id='{}']", row_id))
            .ok().flatten()
            .and_then(|row| row.get_attribute("data-rs-row-label"))
            .unwrap_or_default();
        let _ = rc.set_attribute("data-rs-current-action", &action);
        let _ = rc.set_attribute("data-rs-current-row", &row_id);
        let _ = rc.set_attribute("data-rs-current-label", &row_label);
        let row_el = rc.query_selector(&format!("[data-rs-datatable-row][data-rs-row-id='{}']", row_id))
            .ok().flatten()
            .and_then(|el| el.dyn_into::<HtmlElement>().ok());
        if let Some(row) = row_el {
            match action.as_str() {
                "edit" => {
                    if state::has(&row, "editing") {
                        state::remove(&row, "editing");
                        restore_row_cells(&row);
                        return;
                    }
                    if let Ok(rows) = rc.query_selector_all("[data-rs-datatable-row][data-rs-state~='editing']") {
                        for i in 0..rows.length() {
                            if let Some(r) = rows.item(i).and_then(|n| n.dyn_into::<web_sys::Element>().ok()) {
                                state::remove(&r, "editing");
                                restore_row_cells(&r);
                            }
                        }
                    }
                    state::add(&row, "editing");
                    let _ = rc.set_attribute("data-rs-inline-editing", "true");
                    make_row_editable(&row);
                }
                "delete" => {
                    state::add(&row, "deleting");
                    let row_c = row.clone();
                    let cb = wasm_bindgen::closure::Closure::<dyn Fn()>::wrap(Box::new(move || {
                        let _ = row_c.set_attribute("hidden", "");
                        state::remove(&row_c, "deleting");
                    }));
                    let _ = web_sys::window().and_then(|w| {
                        w.set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), 400).ok()
                    });
                    cb.forget();
                }
                "save" => {
                    state::remove(&row, "editing");
                    let _ = rc.remove_attribute("data-rs-inline-editing");
                    restore_row_cells(&row);
                    let detail = js_sys::Object::new();
                    let _ = js_sys::Reflect::set(&detail, &wasm_bindgen::JsValue::from_str("rowId"), &wasm_bindgen::JsValue::from_str(&row_id));
                    let init = web_sys::CustomEventInit::new();
                    init.set_bubbles(true);
                    init.set_detail(&wasm_bindgen::JsValue::from(detail));
                    if let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict("rs-datatable-row-saved", &init) {
                        let _ = rc.dispatch_event(&event);
                    }
                    return;
                }
                "cancel-edit" => {
                    state::remove(&row, "editing");
                    let _ = rc.remove_attribute("data-rs-inline-editing");
                    restore_row_cells(&row);
                    return;
                }
                _ => {}
            }
        }
        let uid = rc.get_attribute("data-rs-uid").unwrap_or_default();
        let detail = js_sys::Object::new();
        let _ = js_sys::Reflect::set(&detail, &"action".into(), &wasm_bindgen::JsValue::from_str(&action));
        let _ = js_sys::Reflect::set(&detail, &"rowId".into(), &wasm_bindgen::JsValue::from_str(&row_id));
        let _ = js_sys::Reflect::set(&detail, &"label".into(), &wasm_bindgen::JsValue::from_str(&row_label));
        let _ = js_sys::Reflect::set(&detail, &"uid".into(), &wasm_bindgen::JsValue::from_str(&uid));
        let event_init = web_sys::CustomEventInit::new();
        event_init.set_detail(&detail);
        event_init.set_bubbles(true);
        if let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict("rs-datatable-action", &event_init) {
            let _ = rc.dispatch_event(&event);
        }
    }));
    let _ = doc.add_event_listener_with_callback_and_bool("click", cb.as_ref().unchecked_ref(), true);
    cb.forget();
}

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
            let Some(win) = web_sys::window() else { return };
            let Some(d) = win.document() else { return };
            if let Ok(list) = d.query_selector_all("[data-rs-context-menu][data-rs-state~='open']") {
                for i in 0..list.length() {
                    if let Some(el) = list.item(i).and_then(|n| n.dyn_into::<web_sys::Element>().ok()) {
                        state::remove(&el, "open"); state::add(&el, "closed");
                    }
                }
            }
            let x = e.client_x(); let y = e.client_y();
            if let Ok(el) = ctx_root.clone().dyn_into::<web_sys::HtmlElement>() {
                let _ = el.style().set_property("--context-menu-x", &format!("{}px", x));
                let _ = el.style().set_property("--context-menu-y", &format!("{}px", y));
            }
            state::remove(&ctx_root, "closed");
            state::add(&ctx_root, "open");
            let _ = content.remove_attribute("hidden");
        }));
        let _ = doc.add_event_listener_with_callback("contextmenu", cb.as_ref().unchecked_ref());
        cb.forget();
    }
    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let Some(win) = web_sys::window() else { return };
            let Some(d) = win.document() else { return };
            let Ok(list) = d.query_selector_all("[data-rs-context-menu][data-rs-state~='open']") else { return };
            for i in 0..list.length() {
                if let Some(el) = list.item(i).and_then(|n| n.dyn_into::<web_sys::Element>().ok()) {
                    state::remove(&el, "open"); state::add(&el, "closed");
                }
            }
        }));
        let _ = doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}

pub fn update_bulk_bar(root: &web_sys::Element) {
    let count = sel_ids(root).len();
    let bar = root.query_selector("[data-rs-datatable-bulk-bar]").ok().flatten();
    if let Some(bar) = bar {
        if count > 0 { state::remove(&bar, "hidden"); let _ = bar.remove_attribute("hidden"); }
        else { state::add(&bar, "hidden"); let _ = bar.set_attribute("hidden", ""); }
        if let Some(counter) = bar.query_selector("[data-rs-datatable-bulk-count]").ok().flatten() {
            counter.set_text_content(Some(&format!("{} selected", count)));
        }
    }
}

fn bind_bulk_bar(table: &HtmlElement) {
    let root: web_sys::Element = table.clone().into();
    {
        let cb = Closure::<dyn Fn(web_sys::Event)>::wrap(Box::new(move |e: web_sys::Event| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
            update_bulk_bar(&target);
        }));
        let _ = root.add_event_listener_with_callback("rs-selection-change", cb.as_ref().unchecked_ref());
        cb.forget();
    }
    if let Some(clear_btn) = root.query_selector("[data-rs-datatable-bulk-clear]").ok().flatten() {
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

// @domain: editing — candidato a módulo separado no futuro
fn make_row_editable(row: &HtmlElement) {
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
    let Ok(cells) = row.query_selector_all("[data-rs-datatable-cell][data-rs-col-index]") else { return };
    for i in 0..cells.length() {
        let Some(cell) = cells.item(i).and_then(|n| n.dyn_into::<web_sys::Element>().ok()) else { continue };
        let text = cell.text_content().unwrap_or_default().trim().to_string();
        let _ = cell.set_attribute("data-rs-original-value", &text);
        if let Ok(input) = doc.create_element("input") {
            let _ = input.set_attribute("type", "text");
            let _ = input.set_attribute("value", &text);
            let _ = input.set_attribute("data-rs-inline-edit-input", "");
            let _ = input.set_attribute("style", "width:100%;background:transparent;border:none;outline:none;font:inherit;color:inherit;padding:0;");
            cell.set_inner_html("");
            let _ = cell.append_child(&input);
        }
    }
    if let Ok(Some(first_input)) = row.query_selector("[data-rs-inline-edit-input]") {
        if let Ok(html_input) = first_input.dyn_into::<web_sys::HtmlElement>() { let _ = html_input.focus(); }
    }
    if let Ok(toolbar) = doc.create_element("div") {
        let _ = toolbar.set_attribute("data-rs-inline-edit-toolbar", "");
        let _ = toolbar.set_attribute("style", "display:flex;gap:var(--space-xs);align-items:center;");
        if let Ok(save_btn) = doc.create_element("button") {
            let _ = save_btn.set_attribute("type", "button");
            let _ = save_btn.set_attribute("data-rs-datatable-action", "save");
            let _ = save_btn.set_attribute("data-rs-row-id", &row.get_attribute("data-rs-row-id").unwrap_or_default());
            let _ = save_btn.set_attribute("style", "padding:2px 8px;font-size:var(--font-size-xs);background:var(--theme-action-primary-bg);color:var(--theme-action-primary-fg);border:none;border-radius:var(--radius-sm);cursor:pointer;");
            save_btn.set_text_content(Some("Save"));
            let _ = toolbar.append_child(&save_btn);
        }
        if let Ok(cancel_btn) = doc.create_element("button") {
            let _ = cancel_btn.set_attribute("type", "button");
            let _ = cancel_btn.set_attribute("data-rs-datatable-action", "cancel-edit");
            let _ = cancel_btn.set_attribute("data-rs-row-id", &row.get_attribute("data-rs-row-id").unwrap_or_default());
            let _ = cancel_btn.set_attribute("style", "padding:2px 8px;font-size:var(--font-size-xs);background:transparent;border:var(--border-thin) solid var(--theme-surface-border);border-radius:var(--radius-sm);cursor:pointer;color:inherit;");
            cancel_btn.set_text_content(Some("Cancel"));
            let _ = toolbar.append_child(&cancel_btn);
        }
        if let Ok(Some(cell)) = row.query_selector("[data-rs-col-actions]") {
            let _ = cell.append_child(&toolbar);
        }
    }
}

fn restore_row_cells(row: &web_sys::Element) {
    let Ok(cells) = row.query_selector_all("[data-rs-col-index]") else { return };
    for i in 0..cells.length() {
        let Some(cell) = cells.item(i).and_then(|n| n.dyn_into::<web_sys::Element>().ok()) else { continue };
        let value = cell.query_selector("[data-rs-inline-edit-input]")
            .ok().flatten()
            .and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok())
            .map(|input| input.value())
            .or_else(|| cell.get_attribute("data-rs-original-value"))
            .unwrap_or_default();
        let _ = cell.remove_attribute("data-rs-original-value");
        cell.set_text_content(Some(&value));
    }
    if let Ok(Some(toolbar)) = row.query_selector("[data-rs-inline-edit-toolbar]") {
        let _ = toolbar.parent_element().map(|p| p.remove_child(&toolbar));
    }
}
