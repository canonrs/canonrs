//! Chart Interaction — bind only, delegates to chart_engine

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlCanvasElement, HtmlElement};
use crate::shared::{is_initialized, mark_initialized};
use crate::engines::chart_engine::{
    read_chart_data, parse_chart_data, set_canvas_dpi, draw_chart, Series,
};

fn dispatch_custom_event(target: &Element, name: &str, detail: &js_sys::Object) {
    let init = web_sys::CustomEventInit::new();
    init.set_bubbles(true);
    init.set_detail(&JsValue::from(detail));
    if let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict(name, &init) {
        target.dispatch_event(&event).ok();
    }
}

pub fn init(root: Element) {
    if is_initialized(&root) { return; }
    mark_initialized(&root);
    let Ok(Some(canvas_node)) = root.query_selector("[data-rs-chart-canvas]") else { return };
    let Ok(canvas) = canvas_node.dyn_into::<HtmlCanvasElement>() else { return };

    let chart_type = root.get_attribute("data-rs-chart-type").unwrap_or_else(|| "line".to_string());
    let height     = root.get_attribute("data-rs-chart-height").and_then(|v| v.parse::<f64>().ok()).unwrap_or(320.0);
    let show_grid  = root.get_attribute("data-rs-chart-grid").as_deref() != Some("hidden");
    let show_legend = root.get_attribute("data-rs-chart-legend").as_deref() != Some("hidden");
    let sync_table = root.get_attribute("data-rs-chart-sync-table").filter(|s| !s.is_empty());

    let json = read_chart_data(&root);
    let Some((labels, series)) = parse_chart_data(&json) else { return };

    set_canvas_dpi(&canvas, &root, height);
    draw_chart(&canvas, &chart_type, &labels, &series, show_grid, height);

    bind_legend(&root, &canvas, &chart_type, &labels, &series, show_grid, show_legend, height);
    bind_tooltip(&root, &canvas, &chart_type, &labels, &series, show_grid, height, &sync_table);
    bind_resize(&root, &canvas, &chart_type, &labels, &series, show_grid, height);
    bind_datatable_sync(&root, &canvas, &chart_type, &labels, &series, show_grid, height, &sync_table);
}

fn bind_legend(root: &Element, canvas: &HtmlCanvasElement, chart_type: &str, labels: &[String], series: &Series, show_grid: bool, show_legend: bool, height: f64) {
    if !show_legend { return; }
    let Ok(Some(legend_el)) = root.query_selector("[data-rs-chart-legend]") else { return };
    legend_el.set_inner_html("");
    let doc = web_sys::window().unwrap().document().unwrap();
    for (i, (name, _, color, active)) in series.iter().enumerate() {
        let item = doc.create_element("span").unwrap();
        item.set_attribute("data-rs-chart-legend-item", "").ok();
        item.set_attribute("data-rs-series-index", &i.to_string()).ok();
        item.set_attribute("data-rs-state", if *active { "active" } else { "inactive" }).ok();
        let dot = doc.create_element("span").unwrap();
        dot.set_attribute("data-rs-chart-legend-dot", "").ok();
        dot.unchecked_ref::<HtmlElement>().style().set_property("background", color).ok();
        let lbl = doc.create_element("span").unwrap();
        lbl.set_text_content(Some(name));
        item.append_child(&dot).ok();
        item.append_child(&lbl).ok();
        legend_el.append_child(&item).ok();

        let canvas_c = canvas.clone(); let root_c = root.clone();
        let ct_c = chart_type.to_string(); let item_c = item.clone();
        let legend_c = legend_el.clone(); let labels_c = labels.to_vec();
        let series_c = series.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let cur = item_c.get_attribute("data-rs-state").unwrap_or_default();
            item_c.set_attribute("data-rs-state", if cur == "active" { "inactive" } else { "active" }).ok();
            let data_json = read_chart_data(&root_c);
            if let Some((lbl2, mut ser2)) = parse_chart_data(&data_json) {
                let items = legend_c.query_selector_all("[data-rs-chart-legend-item]").unwrap();
                for j in 0..items.length() {
                    if let Some(el) = items.item(j).and_then(|e| e.dyn_into::<Element>().ok()) {
                        let si = el.get_attribute("data-rs-series-index").and_then(|v| v.parse::<usize>().ok()).unwrap_or(0);
                        let a = el.get_attribute("data-rs-state").as_deref() == Some("active");
                        if let Some(s) = ser2.get_mut(si) { s.3 = a; }
                    }
                }
                draw_chart(&canvas_c, &ct_c, &lbl2, &ser2, show_grid, height);
            } else {
                let mut ser2 = series_c.clone();
                let cur2 = item_c.get_attribute("data-rs-state").unwrap_or_default();
                if let Some(s) = ser2.get_mut(i) { s.3 = cur2 == "active"; }
                draw_chart(&canvas_c, &ct_c, &labels_c, &ser2, show_grid, height);
            }
        }) as Box<dyn FnMut(_)>);
        item.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }
}

#[allow(unused_variables)]
fn bind_tooltip(root: &Element, canvas: &HtmlCanvasElement, chart_type: &str, labels: &[String], series: &Series, show_grid: bool, height: f64, sync_table: &Option<String>) {
    let pad_l   = 50.0;
    let chart_w = canvas.unchecked_ref::<HtmlElement>().offset_width() as f64 - pad_l - 20.0;
    let n       = labels.len();
    let step_x  = if n > 1 { chart_w / (n - 1) as f64 } else { chart_w };

    let canvas_c = canvas.clone(); let root_c = root.clone();
    let labels_c = labels.to_vec(); let series_c = series.clone();
    let ct_c = chart_type.to_string(); let sync_c = sync_table.clone();

    let on_move = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
        let rect = canvas_c.get_bounding_client_rect();
        let mx   = e.client_x() as f64 - rect.left() - pad_l;
        if mx < 0.0 { return; }
        let idx = ((mx / step_x).round() as usize).min(labels_c.len().saturating_sub(1));
        if let Ok(Some(tooltip)) = root_c.query_selector("[data-rs-chart-tooltip]") {
            let mut html = format!("<div style=\"font-weight:600;margin-bottom:4px;\">{}</div>", labels_c.get(idx).map(|s| s.as_str()).unwrap_or(""));
            for (name, data, color, active) in &series_c {
                if !active { continue; }
                if let Some(&val) = data.get(idx) {
                    html.push_str(&format!("<div><span style=\"background:{};width:8px;height:8px;border-radius:50%;display:inline-block;margin-right:6px;\"></span>{}: <strong>{:.1}</strong></div>", color, name, val));
                }
            }
            tooltip.set_inner_html(&html);
            let _ = tooltip.set_attribute("data-rs-tooltip-x", &format!("{}px", pad_l + idx as f64 * step_x + 12.0));
            let _ = tooltip.set_attribute("data-rs-tooltip-y", "20px");
            let _ = tooltip.set_attribute("data-rs-state", "open");
        }
        if let Ok(Some(ch)) = root_c.query_selector("[data-rs-chart-crosshair]") {
            let _ = ch.set_attribute("data-rs-crosshair-x", &format!("{}px", pad_l + idx as f64 * step_x));
            let _ = ch.set_attribute("data-rs-state", "open");
        }
        let detail = js_sys::Object::new();
        js_sys::Reflect::set(&detail, &JsValue::from_str("index"), &JsValue::from_f64(idx as f64)).ok();
        dispatch_custom_event(&root_c, "canon:chart:hover", &detail);
        if let Some(ref table_id) = sync_c {
            if let Some(table) = web_sys::window().unwrap().document().unwrap()
                .query_selector(&format!("[data-rs-datatable='{}']", table_id)).ok().flatten()
            {
                if let Ok(rows) = table.query_selector_all("[data-rs-datatable-row]") {
                    for i in 0..rows.length() {
                        if let Some(row) = rows.item(i).and_then(|r| r.dyn_into::<Element>().ok()) {
                            let ri = row.get_attribute("data-rs-row-index").and_then(|v| v.parse::<usize>().ok()).unwrap_or(usize::MAX);
                            if ri == idx { row.set_attribute("data-rs-chart-highlight", "").ok(); }
                            else { row.remove_attribute("data-rs-chart-highlight").ok(); }
                        }
                    }
                }
            }
        }
        let _ = ct_c.as_str();
    }) as Box<dyn FnMut(_)>);

    let root_c2 = root.clone();
    let on_leave = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
        if let Ok(Some(t)) = root_c2.query_selector("[data-rs-chart-tooltip]") { t.set_attribute("data-rs-state", "closed").ok(); }
        if let Ok(Some(c)) = root_c2.query_selector("[data-rs-chart-crosshair]") { c.set_attribute("data-rs-state", "closed").ok(); }
        dispatch_custom_event(&root_c2, "canon:chart:leave", &js_sys::Object::new());
    }) as Box<dyn FnMut(_)>);

    canvas.add_event_listener_with_callback("mousemove", on_move.as_ref().unchecked_ref()).ok();
    canvas.add_event_listener_with_callback("mouseleave", on_leave.as_ref().unchecked_ref()).ok();
    on_move.forget(); on_leave.forget();
}

fn bind_resize(root: &Element, canvas: &HtmlCanvasElement, chart_type: &str, labels: &[String], series: &Series, show_grid: bool, height: f64) {
    let canvas_c = canvas.clone(); let root_c = root.clone();
    let labels_c = labels.to_vec(); let series_c = series.clone(); let ct_c = chart_type.to_string();
    let cb = Closure::wrap(Box::new(move || {
        set_canvas_dpi(&canvas_c, &root_c, height);
        draw_chart(&canvas_c, &ct_c, &labels_c, &series_c, show_grid, height);
    }) as Box<dyn Fn()>);
    let obs_cb = Closure::wrap(Box::new(move |_: js_sys::Array| {
        cb.as_ref().unchecked_ref::<js_sys::Function>().call0(&JsValue::NULL).ok();
    }) as Box<dyn FnMut(js_sys::Array)>);
    let win = web_sys::window().unwrap();
    if let Ok(ctor) = js_sys::Reflect::get(&win, &JsValue::from_str("ResizeObserver")) {
        if let Ok(observer) = js_sys::Reflect::construct(&ctor.unchecked_into::<js_sys::Function>(), &js_sys::Array::of1(obs_cb.as_ref())) {
            let observe_fn = js_sys::Reflect::get(&observer, &JsValue::from_str("observe")).unwrap();
            js_sys::Reflect::apply(&observe_fn.unchecked_into::<js_sys::Function>(), &observer, &js_sys::Array::of1(root)).ok();
        }
    }
    obs_cb.forget();
}

#[allow(unused_variables)]
#[allow(unused_variables)]
fn bind_datatable_sync(root: &Element, canvas: &HtmlCanvasElement, chart_type: &str, labels: &[String], series: &Series, show_grid: bool, height: f64, sync_table: &Option<String>) {
    if sync_table.is_none() { return; }
    let canvas_c = canvas.clone(); let root_c = root.clone();
    let labels_c = labels.to_vec(); let series_c = series.clone(); let ct_c = chart_type.to_string();
    let on_hover = Closure::wrap(Box::new(move |e: web_sys::CustomEvent| {
        let detail = e.detail();
        let idx = js_sys::Reflect::get(&detail, &JsValue::from_str("index"))
            .ok().and_then(|v| v.as_f64()).map(|f| f as usize).unwrap_or(usize::MAX);
        if idx == usize::MAX { return; }
        draw_chart(&canvas_c, &ct_c, &labels_c, &series_c, show_grid, height);
        let pad_l = 50.0;
        let w = canvas_c.unchecked_ref::<HtmlElement>().offset_width() as f64;
        let step_x = if labels_c.len() > 1 { (w - pad_l - 20.0) / (labels_c.len() - 1) as f64 } else { w };
        let x = pad_l + idx as f64 * step_x;
        if let Ok(Some(t)) = root_c.query_selector("[data-rs-chart-tooltip]") {
            let _ = t.set_attribute("data-rs-tooltip-x", &format!("{}px", x + 12.0));
            let _ = t.set_attribute("data-rs-tooltip-y", "20px");
            let _ = t.set_attribute("data-rs-state", "open");
        }
        if let Ok(Some(c)) = root_c.query_selector("[data-rs-chart-crosshair]") {
            let _ = c.set_attribute("data-rs-crosshair-x", &format!("{}px", x));
            let _ = c.set_attribute("data-rs-state", "open");
        }
    }) as Box<dyn FnMut(_)>);
    let root_c2 = root.clone(); let canvas_c2 = canvas.clone();
    let labels_c2 = labels.to_vec(); let series_c2 = series.clone(); let ct_c2 = chart_type.to_string();
    let on_leave = Closure::wrap(Box::new(move |_: web_sys::Event| {
        draw_chart(&canvas_c2, &ct_c2, &labels_c2, &series_c2, show_grid, height);
        if let Ok(Some(t)) = root_c2.query_selector("[data-rs-chart-tooltip]") { t.set_attribute("data-rs-state", "closed").ok(); }
        if let Ok(Some(c)) = root_c2.query_selector("[data-rs-chart-crosshair]") { c.set_attribute("data-rs-state", "closed").ok(); }
    }) as Box<dyn FnMut(_)>);
    root.add_event_listener_with_callback("canon:datatable:hover", on_hover.as_ref().unchecked_ref()).ok();
    root.add_event_listener_with_callback("canon:datatable:leave", on_leave.as_ref().unchecked_ref()).ok();
    on_hover.forget(); on_leave.forget();
}

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-chart]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init(el); }
        }
    }
}
