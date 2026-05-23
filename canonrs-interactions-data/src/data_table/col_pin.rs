//! ColPin — freeze/pin column behavior
//! Core: dom/{state, attrs}
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use crate::runtime::listeners;

pub fn init(table: &HtmlElement) {
    let uid = table.get_attribute("data-rs-uid").unwrap_or_default();
    let root: web_sys::Element = table.clone().into();
    let table_c = table.clone();
    listeners::listen(&uid, &root, "click", move |e: web_sys::Event| {
        use wasm_bindgen::JsCast;
        let e = match e.dyn_into::<web_sys::MouseEvent>() { Ok(e) => e, Err(_) => return };
        let Some(t) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
        let btn = if t.has_attribute("data-rs-datatable-pin-btn") { Some(t) }
            else { t.closest("[data-rs-datatable-pin-btn]").ok().flatten() };
        let Some(btn) = btn else { return };
        e.prevent_default(); e.stop_propagation();
        let col_index = btn.get_attribute("data-rs-col-index").unwrap_or_default();
        let current = btn.get_attribute("data-rs-pin-state").unwrap_or_else(|| "unpinned".to_string());
        let (new_state, new_icon) = match current.as_str() {
            "unpinned"    => ("pinned-left",  "⬅📌"),
            "pinned-left" => ("pinned-right", "📌➡"),
            _             => ("unpinned",     "📍"),
        };
        let _ = btn.set_attribute("data-rs-pin-state", new_state);
        btn.set_text_content(Some(new_icon));
        apply_pin(&table_c, &col_index, new_state);
        let detail = js_sys::Object::new();
        let _ = js_sys::Reflect::set(&detail, &wasm_bindgen::JsValue::from_str("colIndex"), &wasm_bindgen::JsValue::from_str(&col_index));
        let _ = js_sys::Reflect::set(&detail, &wasm_bindgen::JsValue::from_str("pinState"), &wasm_bindgen::JsValue::from_str(new_state));
        let init = web_sys::CustomEventInit::new();
        init.set_bubbles(true);
        init.set_detail(&wasm_bindgen::JsValue::from(detail));
        if let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict("rs-datatable-col-pin", &init) {
            let root2: web_sys::Element = table_c.clone().into();
            let _ = root2.dispatch_event(&event);
        }
    });
}

fn apply_pin(table: &HtmlElement, col_index: &str, state: &str) {
    let selector = format!("[data-rs-col-index='{}']", col_index);
    let Ok(cells) = table.query_selector_all(&selector) else { return };
    for i in 0..cells.length() {
        let Some(el) = cells.item(i).and_then(|n| n.dyn_into::<HtmlElement>().ok()) else { continue };
        let _ = el.set_attribute("data-rs-pin-state", state);
        match state {
            "pinned-left"  => { let _ = el.style().set_property("position", "sticky"); let _ = el.style().set_property("left", "0"); let _ = el.style().set_property("z-index", "2"); }
            "pinned-right" => { let _ = el.style().set_property("position", "sticky"); let _ = el.style().set_property("right", "0"); let _ = el.style().set_property("z-index", "2"); }
            _ => { let _ = el.style().remove_property("position"); let _ = el.style().remove_property("left"); let _ = el.style().remove_property("right"); let _ = el.style().remove_property("z-index"); }
        }
    }
}
