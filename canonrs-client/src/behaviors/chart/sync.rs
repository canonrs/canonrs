//! Chart ↔ DataTable sync

#[cfg(feature = "hydrate")]
use super::draw::{draw_chart, draw_crosshair_on_canvas};
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;

#[cfg(feature = "hydrate")]
pub(crate) fn setup_datatable_to_chart_sync(
    root: &web_sys::Element,
    canvas: &web_sys::HtmlCanvasElement,
    _table_id: &str,
    labels: &[String],
    series: &[(String, Vec<f64>, String, bool)],
    chart_type: &str,
    show_grid: bool,
    height: f64,
) {
    let canvas_c     = canvas.clone();
    let root_c       = root.clone();
    let labels_c     = labels.to_vec();
    let series_c     = series.to_vec();
    let chart_type_c = chart_type.to_string();

    let on_hover = Closure::wrap(Box::new(move |e: web_sys::CustomEvent| {
        let detail = e.detail();
        let idx = js_sys::Reflect::get(&detail, &wasm_bindgen::JsValue::from_str("index"))
            .ok().and_then(|v| v.as_f64()).map(|f| f as usize).unwrap_or(usize::MAX);
        if idx == usize::MAX { return; }
        draw_chart(&canvas_c, &chart_type_c, &labels_c, &series_c, show_grid, height);
        draw_crosshair_on_canvas(&canvas_c, idx, &labels_c, height);
        if let Some(t) = root_c.query_selector("[data-rs-chart-tooltip]").ok().flatten() {
            let pad_l = 50.0;
            let w = canvas_c.unchecked_ref::<web_sys::HtmlElement>().offset_width() as f64;
            let step_x = if labels_c.len() > 1 { (w - pad_l - 20.0) / (labels_c.len() - 1) as f64 } else { w };
            let x = pad_l + idx as f64 * step_x;
            let te: &web_sys::HtmlElement = t.unchecked_ref();
            let _ = te.style().set_property("left", &format!("{}px", x + 12.0));
            let _ = te.style().set_property("top", "20px");
            t.set_attribute("data-rs-state", "visible").ok();
        }
        if let Some(c) = root_c.query_selector("[data-rs-chart-crosshair]").ok().flatten() {
            let pad_l = 50.0;
            let w = canvas_c.unchecked_ref::<web_sys::HtmlElement>().offset_width() as f64;
            let step_x = if labels_c.len() > 1 { (w - pad_l - 20.0) / (labels_c.len() - 1) as f64 } else { w };
            let x = pad_l + idx as f64 * step_x;
            let ce: &web_sys::HtmlElement = c.unchecked_ref();
            let _ = ce.style().set_property("left", &format!("{}px", x));
            c.set_attribute("data-rs-state", "visible").ok();
        }
    }) as Box<dyn FnMut(_)>);

    let canvas_c2     = canvas.clone();
    let root_c2       = root.clone();
    let labels_c2     = labels.to_vec();
    let series_c2     = series.to_vec();
    let chart_type_c2 = chart_type.to_string();

    let on_leave = Closure::wrap(Box::new(move |_: web_sys::Event| {
        draw_chart(&canvas_c2, &chart_type_c2, &labels_c2, &series_c2, show_grid, height);
        if let Some(t) = root_c2.query_selector("[data-rs-chart-tooltip]").ok().flatten() {
            t.set_attribute("data-rs-state", "hidden").ok();
        }
        if let Some(c) = root_c2.query_selector("[data-rs-chart-crosshair]").ok().flatten() {
            c.set_attribute("data-rs-state", "hidden").ok();
        }
    }) as Box<dyn FnMut(_)>);

    // scoped no root via bubbling
    root.add_event_listener_with_callback("canon:datatable:hover", on_hover.as_ref().unchecked_ref()).ok();
    root.add_event_listener_with_callback("canon:datatable:leave", on_leave.as_ref().unchecked_ref()).ok();
    on_hover.forget();
    on_leave.forget();
}
