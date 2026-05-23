//! ColResize — column resize handle behavior
//! Core: dom/{state, attrs}
use canonrs_interactions_core::dom::lifecycle;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use crate::runtime::listeners;
use crate::runtime::drag;

pub fn init(table: &HtmlElement) {
    if !lifecycle::init_guard(&table.clone().into()) { return; }
    let uid = table.get_attribute("data-rs-uid").unwrap_or_default();
    let handles = match table.query_selector_all("[data-rs-datatable-resize-handle]") {
        Ok(h) => h, Err(_) => return,
    };
    for i in 0..handles.length() {
        let Some(node) = handles.item(i) else { continue };
        let Ok(handle) = node.dyn_into::<web_sys::Element>() else { continue };
        let Ok(th) = handle.parent_element().ok_or(()).and_then(|el| el.dyn_into::<HtmlElement>().map_err(|_| ())) else { continue };

        let th_start = std::rc::Rc::new(std::cell::Cell::new(0i32));
        let x_start  = std::rc::Rc::new(std::cell::Cell::new(0i32));

        let th_s  = th_start.clone();
        let x_s   = x_start.clone();
        let th_md = th.clone();
        let uid_md = uid.clone();

        listeners::listen(&uid, &handle, "mousedown", move |e: web_sys::Event| {
            use wasm_bindgen::JsCast;
            let e = match e.dyn_into::<web_sys::MouseEvent>() { Ok(e) => e, Err(_) => return };
            e.prevent_default();
            x_s.set(e.client_x());
            th_s.set(th_md.offset_width());
            if let Some(win) = web_sys::window() {
                let _ = win.document().and_then(|d| {
                    d.document_element().map(|el| { let _ = el.class_list().add_1("rs-resizing"); el })
                });
            }
            let th_move = th_md.clone();
            let th_start_move = th_s.clone();
            let x_start_move  = x_s.clone();
            let uid_drag = uid_md.clone();
            drag::start(
                &uid_drag,
                move |_dx, _dy, client_x, _client_y| {
                    let delta = client_x as i32 - x_start_move.get();
                    let new_width = (th_start_move.get() + delta).max(40);
                    let col_index = th_move.get_attribute("data-rs-col-index").unwrap_or_default();
                    let _ = th_move.style().set_property("width", &format!("{}px", new_width));
                    let _ = th_move.set_attribute("data-rs-col-width", &new_width.to_string());
                    if let Some(win) = web_sys::window() {
                        if let Some(doc) = win.document() {
                            if let Ok(cells) = doc.query_selector_all(&format!("[data-rs-col-index='{}']", col_index)) {
                                for j in 0..cells.length() {
                                    if let Some(cell) = cells.item(j).and_then(|n| n.dyn_into::<HtmlElement>().ok()) {
                                        let _ = cell.style().set_property("width", &format!("{}px", new_width));
                                    }
                                }
                            }
                            let detail = js_sys::Object::new();
                            let _ = js_sys::Reflect::set(&detail, &wasm_bindgen::JsValue::from_str("colIndex"), &wasm_bindgen::JsValue::from_str(&col_index));
                            let _ = js_sys::Reflect::set(&detail, &wasm_bindgen::JsValue::from_str("width"), &wasm_bindgen::JsValue::from_f64(new_width as f64));
                            let init = web_sys::CustomEventInit::new();
                            init.set_bubbles(true);
                            init.set_detail(&wasm_bindgen::JsValue::from(detail));
                            if let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict("rs-datatable-col-resize", &init) {
                                let _ = th_move.dispatch_event(&event);
                            }
                        }
                    }
                },
                move || {
                    if let Some(win) = web_sys::window() {
                        let _ = win.document().and_then(|d| {
                            d.document_element().map(|el| { let _ = el.class_list().remove_1("rs-resizing"); el })
                        });
                    }
                },
            );
        });
    }
}
