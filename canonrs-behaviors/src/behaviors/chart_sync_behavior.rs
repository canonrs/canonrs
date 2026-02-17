//! Chart Sync Behavior - Bidirecional Chart ↔ DataTable via CustomEvent
//! Canal: canon:chart:hover / canon:datatable:hover
//! Zero acoplamento direto entre componentes

#[cfg(feature = "hydrate")]
use super::register_behavior;
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;
#[cfg(feature = "hydrate")]
use leptos::leptos_dom::helpers::document;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;

#[cfg(feature = "hydrate")]
pub fn register() {
    // Chart → Table: escuta canon:chart:hover e destaca row
    register_behavior("data-chart", Box::new(|element_id, _state| {
        let Some(root) = document().get_element_by_id(element_id) else { return Ok(()); };
        let Some(table_id) = root.get_attribute("data-chart-sync-table") else { return Ok(()); };
        if table_id.is_empty() { return Ok(()); }
        if root.get_attribute("data-sync-chart-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-sync-chart-attached", "1").ok();

        // Escuta canon:chart:hover → highlight row na tabela
        let table_id_c = table_id.clone();
        let on_chart_hover = Closure::wrap(Box::new(move |e: web_sys::CustomEvent| {
            let detail = e.detail();
            let idx = js_sys::Reflect::get(&detail, &JsValue::from_str("index"))
                .ok().and_then(|v| v.as_f64()).map(|f| f as usize)
                .unwrap_or(usize::MAX);

            let Some(table) = document().get_element_by_id(&table_id_c) else { return };
            let Ok(rows) = table.query_selector_all("[data-datatable-row]") else { return };
            for i in 0..rows.length() {
                let Some(row) = rows.item(i).and_then(|r| r.dyn_into::<web_sys::Element>().ok()) else { continue };
                let row_idx = row.get_attribute("data-row-index")
                    .and_then(|v| v.parse::<usize>().ok()).unwrap_or(usize::MAX);
                if row_idx == idx {
                    row.set_attribute("data-chart-highlight", "").ok();
                    // Scroll suave para a row visível
                    row.unchecked_ref::<web_sys::HtmlElement>().scroll_into_view_with_bool(false);
                } else {
                    row.remove_attribute("data-chart-highlight").ok();
                }
            }
        }) as Box<dyn FnMut(_)>);

        document().add_event_listener_with_callback(
            "canon:chart:hover",
            on_chart_hover.as_ref().unchecked_ref()
        ).ok();
        on_chart_hover.forget();

        // Escuta canon:chart:leave → remove highlights
        let table_id_c2 = table_id.clone();
        let on_chart_leave = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let Some(table) = document().get_element_by_id(&table_id_c2) else { return };
            let Ok(rows) = table.query_selector_all("[data-datatable-row][data-chart-highlight]") else { return };
            for i in 0..rows.length() {
                if let Some(r) = rows.item(i).and_then(|r| r.dyn_into::<web_sys::Element>().ok()) {
                    r.remove_attribute("data-chart-highlight").ok();
                }
            }
        }) as Box<dyn FnMut(_)>);

        document().add_event_listener_with_callback(
            "canon:chart:leave",
            on_chart_leave.as_ref().unchecked_ref()
        ).ok();
        on_chart_leave.forget();

        Ok(())
    }));

    // DataTable → Chart: escuta hover nas rows e dispara canon:datatable:hover
    register_behavior("data-datatable", Box::new(|element_id, _state| {
        let Some(table) = document().get_element_by_id(element_id) else { return Ok(()); };
        let Some(chart_id) = table.get_attribute("data-table-sync-chart") else { return Ok(()); };
        if chart_id.is_empty() { return Ok(()); }
        if table.get_attribute("data-sync-table-attached").as_deref() == Some("1") { return Ok(()); }
        table.set_attribute("data-sync-table-attached", "1").ok();

        let Ok(rows) = table.query_selector_all("[data-datatable-row]") else { return Ok(()); };

        for i in 0..rows.length() {
            let Some(row) = rows.item(i).and_then(|r| r.dyn_into::<web_sys::Element>().ok()) else { continue };
            let chart_id_c = chart_id.clone();
            let row_c = row.clone();

            // mouseenter → dispara canon:datatable:hover
            let enter = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
                let idx = row_c.get_attribute("data-row-index")
                    .and_then(|v| v.parse::<usize>().ok()).unwrap_or(usize::MAX);
                if idx == usize::MAX { return; }

                let detail = js_sys::Object::new();
                js_sys::Reflect::set(&detail, &JsValue::from_str("index"), &JsValue::from_f64(idx as f64)).ok();
                js_sys::Reflect::set(&detail, &JsValue::from_str("chartId"), &JsValue::from_str(&chart_id_c)).ok();
                dispatch_custom_event("canon:datatable:hover", &detail);
            }) as Box<dyn FnMut(_)>);

            // mouseleave → dispara canon:datatable:leave
            let chart_id_c2 = chart_id.clone();
            let leave = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
                let detail = js_sys::Object::new();
                js_sys::Reflect::set(&detail, &JsValue::from_str("chartId"), &JsValue::from_str(&chart_id_c2)).ok();
                dispatch_custom_event("canon:datatable:leave", &detail);
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
fn dispatch_custom_event(name: &str, detail: &js_sys::Object) {
    let f = js_sys::Function::new_with_args(
        "name,detail",
        "var e = new CustomEvent(name, {bubbles:true, detail:detail}); document.dispatchEvent(e);"
    );
    f.call2(&JsValue::NULL, &JsValue::from_str(name), detail).ok();
}
#[cfg(not(feature = "hydrate"))]
pub fn register() {}
