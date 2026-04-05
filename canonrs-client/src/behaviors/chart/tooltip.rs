#![cfg(feature = "hydrate")]
//! Chart Tooltip

#[cfg(feature = "hydrate")]
use super::utils::dispatch_custom_event;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;

#[cfg(feature = "hydrate")]
pub(crate) fn setup_tooltip(
    canvas: &web_sys::HtmlCanvasElement,
    root: &web_sys::Element,
    labels: &[String],
    series: &[(String, Vec<f64>, String, bool)],
    _height: f64,
    sync_table: Option<String>,
) {
    let pad_l = 50.0;
    let chart_w = canvas.unchecked_ref::<web_sys::HtmlElement>().offset_width() as f64 - pad_l - 20.0;
    let n = labels.len();
    let step_x = if n > 1 { chart_w / (n - 1) as f64 } else { chart_w };

    let canvas_c = canvas.clone();
    let root_c   = root.clone();
    let labels_d = labels.to_vec();
    let series_d = series.to_vec();
    let sync_c   = sync_table.clone();

    let closure = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
        let rect = canvas_c.get_bounding_client_rect();
        let mx = e.client_x() as f64 - rect.left() - pad_l;
        if mx < 0.0 { return; }
        let idx = ((mx / step_x).round() as usize).min(labels_d.len().saturating_sub(1));

        let Some(tooltip) = root_c.query_selector("[data-rs-chart-tooltip]").ok().flatten() else { return };
        let tooltip_el: &web_sys::HtmlElement = tooltip.unchecked_ref();
        let mut html = format!(r#"<div style="font-weight:600;margin-bottom:4px;">{}</div>"#,
            labels_d.get(idx).map(|s| s.as_str()).unwrap_or(""));
        for (name, data, color, active) in &series_d {
            if !active { continue; }
            if let Some(&val) = data.get(idx) {
                html.push_str(&format!(
                    r#"<div style="display:flex;align-items:center;gap:6px;">
                        <span style="background:{};width:8px;height:8px;border-radius:50%;display:inline-block;"></span>
                        <span>{}: </span><strong>{:.1}</strong></div>"#,
                    color, name, val
                ));
            }
        }
        tooltip.set_inner_html(&html);
        let x = pad_l + idx as f64 * step_x;
        let _ = tooltip_el.style().set_property("left", &format!("{}px", x + 12.0));
        let _ = tooltip_el.style().set_property("top", "20px");
        tooltip.set_attribute("data-rs-state", "open").ok();

        let detail = js_sys::Object::new();
        js_sys::Reflect::set(&detail, &wasm_bindgen::JsValue::from_str("index"), &wasm_bindgen::JsValue::from_f64(idx as f64)).ok();
        // scoped: dispatch no root, não no document
        dispatch_custom_event(&root_c, "canon:chart:hover", &detail);

        if let Some(crosshair) = root_c.query_selector("[data-rs-chart-crosshair]").ok().flatten() {
            let ch: &web_sys::HtmlElement = crosshair.unchecked_ref();
            let _ = ch.style().set_property("left", &format!("{}px", x));
            crosshair.set_attribute("data-rs-state", "open").ok();
        }

        // sync datatable scoped via table_id
        if let Some(ref table_id) = sync_c {
            if let Some(table) = root_c.closest(&format!("[data-rs-root]")).ok().flatten()
                .and_then(|r| r.query_selector(&format!("[data-rs-datatable='{}']", table_id)).ok().flatten())
                .or_else(|| web_sys::window().unwrap().document().unwrap()
                    .query_selector(&format!("[data-rs-datatable='{}']", table_id)).ok().flatten())
            {
                if let Ok(rows) = table.query_selector_all("[data-rs-datatable-row]") {
                    for i in 0..rows.length() {
                        if let Some(row) = rows.item(i).and_then(|r| r.dyn_into::<web_sys::Element>().ok()) {
                            let row_idx = row.get_attribute("data-rs-row-index")
                                .and_then(|v| v.parse::<usize>().ok()).unwrap_or(usize::MAX);
                            if row_idx == idx { row.set_attribute("data-rs-chart-highlight", "").ok(); }
                            else { row.remove_attribute("data-rs-chart-highlight").ok(); }
                        }
                    }
                }
            }
        }
    }) as Box<dyn FnMut(_)>);

    canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref()).ok();
    closure.forget();

    let root_c2 = root.clone();
    let hide = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
        if let Some(t) = root_c2.query_selector("[data-rs-chart-tooltip]").ok().flatten() {
            t.set_attribute("data-rs-state", "closed").ok();
        }
        if let Some(c) = root_c2.query_selector("[data-rs-chart-crosshair]").ok().flatten() {
            c.set_attribute("data-rs-state", "closed").ok();
        }
        let detail = js_sys::Object::new();
        dispatch_custom_event(&root_c2, "canon:chart:leave", &detail);
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mouseleave", hide.as_ref().unchecked_ref()).ok();
    hide.forget();
}
