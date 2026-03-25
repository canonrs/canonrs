//! Chart Sync Behavior - Bidirecional Chart ↔ DataTable via CustomEvent
//! Canal: canon:chart:hover / canon:datatable:hover

#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-chart", Box::new(|root: &web_sys::Element, _state: &ComponentState| {
        let Some(table_id) = root.get_attribute("data-rs-chart-sync-table") else { return Ok(()); };
        if table_id.is_empty() { return Ok(()); }
        if root.get_attribute("data-rs-sync-chart-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-sync-chart-attached", "1").ok();

        let table_id_c = table_id.clone();
        let on_chart_hover = Closure::wrap(Box::new(move |e: web_sys::CustomEvent| {
            let detail = e.detail();
            let idx = js_sys::Reflect::get(&detail, &JsValue::from_str("index"))
                .ok().and_then(|v| v.as_f64()).map(|f| f as usize)
                .unwrap_or(usize::MAX);
            // scoped: busca tabela pelo id dentro do document (cross-component intencional)
            let doc = web_sys::window().unwrap().document().unwrap();
            let Ok(rows) = doc.query_selector_all(
                &format!("[data-rs-datatable='{}'] [data-rs-datatable-row]", table_id_c)
            ) else { return };
            for i in 0..rows.length() {
                let Some(row) = rows.item(i).and_then(|r| r.dyn_into::<web_sys::Element>().ok()) else { continue };
                let row_idx = row.get_attribute("data-row-index")
                    .and_then(|v| v.parse::<usize>().ok()).unwrap_or(usize::MAX);
                if row_idx == idx {
                    row.set_attribute("data-rs-chart-highlight", "").ok();
                    row.unchecked_ref::<web_sys::HtmlElement>().scroll_into_view_with_bool(false);
                } else {
                    row.remove_attribute("data-rs-chart-highlight").ok();
                }
            }
        }) as Box<dyn FnMut(_)>);

        root.add_event_listener_with_callback("canon:chart:hover", on_chart_hover.as_ref().unchecked_ref()).ok();
        on_chart_hover.forget();

        let table_id_c2 = table_id.clone();
        let on_chart_leave = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let doc = web_sys::window().unwrap().document().unwrap();
            let Ok(rows) = doc.query_selector_all(
                &format!("[data-rs-datatable='{}'] [data-rs-datatable-row][data-rs-chart-highlight]", table_id_c2)
            ) else { return };
            for i in 0..rows.length() {
                if let Some(r) = rows.item(i).and_then(|r| r.dyn_into::<web_sys::Element>().ok()) {
                    r.remove_attribute("data-rs-chart-highlight").ok();
                }
            }
        }) as Box<dyn FnMut(_)>);

        root.add_event_listener_with_callback("canon:chart:leave", on_chart_leave.as_ref().unchecked_ref()).ok();
        on_chart_leave.forget();
        Ok(())
    }));

    register_behavior("data-rs-datatable", Box::new(|root: &web_sys::Element, _state: &ComponentState| {
        let Some(chart_id) = root.get_attribute("data-rs-table-sync-chart") else { return Ok(()); };
        if chart_id.is_empty() { return Ok(()); }
        if root.get_attribute("data-rs-sync-table-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-sync-table-attached", "1").ok();

        let Ok(rows) = root.query_selector_all("[data-rs-datatable-row]") else { return Ok(()); };

        for i in 0..rows.length() {
            let Some(row) = rows.item(i).and_then(|r| r.dyn_into::<web_sys::Element>().ok()) else { continue };
            let chart_id_c  = chart_id.clone();
            let chart_id_c2 = chart_id.clone();
            let row_c = row.clone();

            let enter = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
                let idx = row_c.get_attribute("data-row-index")
                    .and_then(|v| v.parse::<usize>().ok()).unwrap_or(usize::MAX);
                if idx == usize::MAX { return; }
                let detail = js_sys::Object::new();
                js_sys::Reflect::set(&detail, &JsValue::from_str("index"), &JsValue::from_f64(idx as f64)).ok();
                js_sys::Reflect::set(&detail, &JsValue::from_str("chartId"), &JsValue::from_str(&chart_id_c)).ok();
                dispatch_custom_event(&row_c, "canon:datatable:hover", &detail);
            }) as Box<dyn FnMut(_)>);

            let row_c2 = row.clone();
            let leave = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
                let detail = js_sys::Object::new();
                js_sys::Reflect::set(&detail, &JsValue::from_str("chartId"), &JsValue::from_str(&chart_id_c2)).ok();
                dispatch_custom_event(&row_c2, "canon:datatable:leave", &detail);
            }) as Box<dyn FnMut(_)>);

            row.add_event_listener_with_callback("mouseenter", enter.as_ref().unchecked_ref()).ok();
            row.add_event_listener_with_callback("mouseleave", leave.as_ref().unchecked_ref()).ok();
            enter.forget();
            leave.forget();
        }
        Ok(())
    }));
}

#[cfg(feature = "hydrate")]
fn dispatch_custom_event(target: &web_sys::Element, name: &str, detail: &js_sys::Object) {
    use wasm_bindgen::JsValue;
    let init = web_sys::CustomEventInit::new();
    init.set_bubbles(true);
    init.set_detail(&JsValue::from(detail));
    if let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict(name, &init) {
        target.dispatch_event(&event).ok();
    }
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
