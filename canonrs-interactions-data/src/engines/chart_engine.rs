//! Chart Engine — canvas rendering
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlCanvasElement, HtmlElement};

pub type Series = Vec<(String, Vec<f64>, String, bool)>;

pub fn read_chart_data(root: &Element) -> String {
    // data-rs-chart-data is on a child element, not the root
    if let Ok(Some(el)) = root.query_selector("[data-rs-chart-data]") {
        return el.get_attribute("data-rs-chart-data").unwrap_or_default();
    }
    // fallback: check root itself
    root.get_attribute("data-rs-chart-data").unwrap_or_default()
}

pub fn parse_chart_data(json: &str) -> Option<(Vec<String>, Series)> {
    let json = json.trim();
    if json.is_empty() { return None; }
    let val = js_sys::JSON::parse(json).ok()?;
    let obj = val.dyn_into::<js_sys::Object>().ok()?;
    let labels_arr = js_sys::Reflect::get(&obj, &wasm_bindgen::JsValue::from_str("labels"))
        .ok()?.dyn_into::<js_sys::Array>().ok()?;
    let labels: Vec<String> = (0..labels_arr.length())
        .filter_map(|i| labels_arr.get(i).as_string()).collect();
    let series_arr = js_sys::Reflect::get(&obj, &wasm_bindgen::JsValue::from_str("series"))
        .ok()?.dyn_into::<js_sys::Array>().ok()?;
    let colors = ["#6366f1","#f59e0b","#10b981","#ef4444","#8b5cf6","#06b6d4"];
    let mut series: Series = Vec::new();
    for i in 0..series_arr.length() {
        let s = series_arr.get(i).dyn_into::<js_sys::Object>().ok()?;
        let name = js_sys::Reflect::get(&s, &wasm_bindgen::JsValue::from_str("name"))
            .ok()?.as_string().unwrap_or_default();
        let color = js_sys::Reflect::get(&s, &wasm_bindgen::JsValue::from_str("color"))
            .ok().and_then(|v| v.as_string())
            .unwrap_or_else(|| colors[i as usize % colors.len()].to_string());
        let data_arr = js_sys::Reflect::get(&s, &wasm_bindgen::JsValue::from_str("data"))
            .ok()?.dyn_into::<js_sys::Array>().ok()?;
        let data: Vec<f64> = (0..data_arr.length())
            .filter_map(|j| data_arr.get(j).as_f64()).collect();
        series.push((name, data, color, true));
    }
    Some((labels, series))
}

pub fn set_canvas_dpi(canvas: &HtmlCanvasElement, root: &Element, height: f64) {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let dpr = win.device_pixel_ratio();
    let w = root.unchecked_ref::<HtmlElement>().offset_width() as f64;
    canvas.set_width((w * dpr) as u32);
    canvas.set_height((height * dpr) as u32);
    let _ = canvas.unchecked_ref::<HtmlElement>().style().set_property("width",  &format!("{}px", w));
    let _ = canvas.unchecked_ref::<HtmlElement>().style().set_property("height", &format!("{}px", height));
    if let Ok(Some(ctx)) = canvas.get_context("2d") {
        if let Ok(ctx2d) = ctx.dyn_into::<web_sys::CanvasRenderingContext2d>() {
            ctx2d.scale(dpr, dpr).ok();
        }
    }
}

pub fn draw_chart(canvas: &HtmlCanvasElement, chart_type: &str, labels: &[String], series: &Series, show_grid: bool, height: f64) {
    let Ok(Some(ctx_obj)) = canvas.get_context("2d") else { return };
    let Ok(ctx) = ctx_obj.dyn_into::<web_sys::CanvasRenderingContext2d>() else { return };
    let w = canvas.unchecked_ref::<HtmlElement>().offset_width() as f64;
    let pad_l = 50.0; let pad_b = 30.0; let pad_t = 20.0; let pad_r = 20.0;
    let chart_w = w - pad_l - pad_r;
    let chart_h = height - pad_b - pad_t;
    ctx.clear_rect(0.0, 0.0, w, height);
    let active: Vec<&(String, Vec<f64>, String, bool)> = series.iter().filter(|s| s.3).collect();
    if active.is_empty() || labels.is_empty() { return; }
    let all_vals: Vec<f64> = active.iter().flat_map(|s| s.1.iter().copied()).collect();
    let max_v = all_vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max).max(1.0);
    let min_v = if chart_type == "bar" { 0.0 } else { all_vals.iter().cloned().fold(f64::INFINITY, f64::min).min(0.0) };
    let range = (max_v - min_v).max(1.0);
    let n = labels.len();
    let step_x = if n > 1 { chart_w / (n - 1) as f64 } else { chart_w };
    let to_y = |v: f64| -> f64 { pad_t + chart_h * (1.0 - (v - min_v) / range) };
    if show_grid {
        ctx.set_stroke_style_str("rgba(128,128,128,0.15)");
        ctx.set_line_width(1.0);
        for gi in 0..=4 {
            let y = pad_t + chart_h * gi as f64 / 4.0;
            ctx.begin_path(); ctx.move_to(pad_l, y); ctx.line_to(w - pad_r, y); ctx.stroke();
        }
    }
    ctx.set_font("11px system-ui");
    ctx.set_fill_style_str("rgba(128,128,128,0.7)");
    ctx.set_text_align("center");
    for (i, lbl) in labels.iter().enumerate() {
        let x = pad_l + i as f64 * step_x;
        ctx.fill_text(lbl, x, height - 8.0).ok();
    }
    for (_, data, color, _) in &active {
        if chart_type == "bar" {
            let bw = (step_x * 0.5 / active.len() as f64).max(4.0);
            ctx.set_fill_style_str(color);
            for (i, &v) in data.iter().enumerate() {
                let x = pad_l + i as f64 * step_x - bw / 2.0;
                let y = to_y(v); let bh = chart_h - (y - pad_t);
                ctx.fill_rect(x, y, bw, bh);
            }
        } else {
            ctx.set_stroke_style_str(color);
            ctx.set_line_width(2.0);
            ctx.begin_path();
            for (i, &v) in data.iter().enumerate() {
                let x = pad_l + i as f64 * step_x;
                let y = to_y(v);
                if i == 0 { ctx.move_to(x, y); } else { ctx.line_to(x, y); }
            }
            ctx.stroke();
            ctx.set_fill_style_str(color);
            for (i, &v) in data.iter().enumerate() {
                let x = pad_l + i as f64 * step_x;
                let y = to_y(v);
                ctx.begin_path();
                ctx.arc(x, y, 3.0, 0.0, std::f64::consts::TAU).ok();
                ctx.fill();
            }
        }
    }
}
