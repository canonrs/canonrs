//! Chart Draw - line, bar, area, donut, grid

#[cfg(feature = "hydrate")]
use super::utils::{get_context, format_axis_val};
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;

#[cfg(feature = "hydrate")]
pub(crate) fn draw_chart(
    canvas: &web_sys::HtmlCanvasElement,
    chart_type: &str,
    labels: &[String],
    series: &[(String, Vec<f64>, String, bool)],
    show_grid: bool,
    height: f64,
) {
    let active: Vec<(String, Vec<f64>, String, bool)> = series.iter()
        .filter(|(_, _, _, active)| *active).cloned().collect();
    match chart_type {
        "bar"   => draw_bar(canvas, labels, &active, show_grid, height),
        "area"  => draw_area(canvas, labels, &active, show_grid, height),
        "donut" => draw_donut(canvas, labels, &active, height),
        _       => draw_line(canvas, labels, &active, show_grid, height),
    }
}

#[cfg(feature = "hydrate")]
pub(crate) fn set_canvas_dpi(canvas: &web_sys::HtmlCanvasElement, root: &web_sys::Element, height: f64) {
    let window = web_sys::window().unwrap();
    let dpr = window.device_pixel_ratio();
    let max_width = root.get_attribute("data-rs-chart-max-width")
        .and_then(|v| if v.is_empty() { None } else { v.parse::<f64>().ok() });
    let parent_width = root.get_bounding_client_rect().width();
    let w = match max_width {
        Some(mw) if mw > 0.0 => parent_width.min(mw),
        _ => if parent_width > 0.0 { parent_width } else { 600.0 },
    };
    canvas.set_width((w * dpr) as u32);
    canvas.set_height((height * dpr) as u32);
    let style = canvas.unchecked_ref::<web_sys::HtmlElement>().style();
    let _ = style.set_property("width", &format!("{}px", w));
    let _ = style.set_property("height", &format!("{}px", height));
    if let Some(ctx) = get_context(canvas) { ctx.scale(dpr, dpr).ok(); }
}

#[cfg(feature = "hydrate")]
pub(crate) fn draw_crosshair_on_canvas(canvas: &web_sys::HtmlCanvasElement, idx: usize, labels: &[String], height: f64) {
    let Some(ctx) = get_context(canvas) else { return };
    let pad_l = 50.0; let pad_b = 40.0;
    let w = canvas.unchecked_ref::<web_sys::HtmlElement>().offset_width() as f64;
    let chart_w = w - pad_l - 20.0;
    let step_x = if labels.len() > 1 { chart_w / (labels.len() - 1) as f64 } else { chart_w };
    let x = pad_l + idx as f64 * step_x;
    ctx.set_stroke_style_str("#9ca3af");
    ctx.set_line_width(1.0);
    ctx.set_line_dash(&js_sys::Array::of2(&wasm_bindgen::JsValue::from(4.0), &wasm_bindgen::JsValue::from(4.0))).ok();
    ctx.begin_path();
    ctx.move_to(x, 20.0);
    ctx.line_to(x, height - pad_b);
    ctx.stroke();
    ctx.set_line_dash(&js_sys::Array::new()).ok();
}

#[cfg(feature = "hydrate")]
fn draw_grid(ctx: &web_sys::CanvasRenderingContext2d, labels: &[String], max_v: f64, _min_v: f64, range: f64, w: f64, h: f64, pl: f64, pr: f64, pt: f64, pb: f64, ch: f64, sx: f64) {
    ctx.set_stroke_style_str("#e5e7eb");
    ctx.set_line_width(1.0);
    for i in 0..=5 {
        let y = pt + ch * i as f64 / 5.0;
        ctx.begin_path(); ctx.move_to(pl, y); ctx.line_to(w - pr, y); ctx.stroke();
        let val = max_v - range * i as f64 / 5.0;
        ctx.set_fill_style_str("#9ca3af"); ctx.set_font("12px system-ui"); ctx.set_text_align("right");
        let _ = ctx.fill_text(&format_axis_val(val), pl - 6.0, y + 4.0);
    }
    ctx.set_fill_style_str("#9ca3af"); ctx.set_font("12px system-ui"); ctx.set_text_align("center");
    for (i, label) in labels.iter().enumerate() {
        let _ = ctx.fill_text(label, pl + i as f64 * sx, h - pb + 20.0);
    }
}

#[cfg(feature = "hydrate")]
fn draw_line(canvas: &web_sys::HtmlCanvasElement, labels: &[String], series: &[(String, Vec<f64>, String, bool)], show_grid: bool, height: f64) {
    let Some(ctx) = get_context(canvas) else { return };
    let w = canvas.unchecked_ref::<web_sys::HtmlElement>().offset_width() as f64;
    let (pl, pr, pt, pb) = (50.0, 20.0, 20.0, 40.0);
    ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    let all: Vec<f64> = series.iter().flat_map(|(_, d, _, _)| d.iter().copied()).collect();
    let max_v = all.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_v = all.iter().cloned().fold(f64::INFINITY, f64::min).min(0.0);
    let range = (max_v - min_v).max(1.0);
    let cw = w - pl - pr; let ch = height - pt - pb;
    let sx = if labels.len() > 1 { cw / (labels.len() - 1) as f64 } else { cw };
    if show_grid { draw_grid(&ctx, labels, max_v, min_v, range, w, height, pl, pr, pt, pb, ch, sx); }
    for (_, data, color, _) in series {
        ctx.set_stroke_style_str(color); ctx.set_line_width(2.5); ctx.set_line_join("round");
        ctx.begin_path();
        for (i, &v) in data.iter().enumerate() {
            let x = pl + i as f64 * sx;
            let y = pt + ch - ((v - min_v) / range * ch);
            if i == 0 { ctx.move_to(x, y); } else { ctx.line_to(x, y); }
        }
        ctx.stroke();
        ctx.set_fill_style_str(color);
        for (i, &v) in data.iter().enumerate() {
            let x = pl + i as f64 * sx;
            let y = pt + ch - ((v - min_v) / range * ch);
            ctx.begin_path();
            ctx.arc(x, y, 4.0, 0.0, std::f64::consts::PI * 2.0).ok();
            ctx.fill();
        }
    }
}

#[cfg(feature = "hydrate")]
fn draw_bar(canvas: &web_sys::HtmlCanvasElement, labels: &[String], series: &[(String, Vec<f64>, String, bool)], show_grid: bool, height: f64) {
    let Some(ctx) = get_context(canvas) else { return };
    let w = canvas.unchecked_ref::<web_sys::HtmlElement>().offset_width() as f64;
    let (pl, pr, pt, pb) = (50.0, 20.0, 20.0, 40.0);
    ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    let all: Vec<f64> = series.iter().flat_map(|(_, d, _, _)| d.iter().copied()).collect();
    let max_v = all.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = max_v.max(1.0);
    let cw = w - pl - pr; let ch = height - pt - pb;
    let nl = labels.len(); let ns = series.len();
    let gw = cw / nl as f64; let bw = (gw * 0.7) / ns as f64; let bg = gw * 0.15;
    if show_grid { draw_grid(&ctx, labels, max_v, 0.0, range, w, height, pl, pr, pt, pb, ch, gw); }
    for (si, (_, data, color, _)) in series.iter().enumerate() {
        ctx.set_fill_style_str(color);
        for (i, &v) in data.iter().enumerate() {
            let bh = v / range * ch;
            let x = pl + i as f64 * gw + bg + si as f64 * bw;
            ctx.fill_rect(x, pt + ch - bh, bw - 2.0, bh);
        }
    }
}

#[cfg(feature = "hydrate")]
fn draw_area(canvas: &web_sys::HtmlCanvasElement, labels: &[String], series: &[(String, Vec<f64>, String, bool)], show_grid: bool, height: f64) {
    let Some(ctx) = get_context(canvas) else { return };
    let w = canvas.unchecked_ref::<web_sys::HtmlElement>().offset_width() as f64;
    let (pl, pr, pt, pb) = (50.0, 20.0, 20.0, 40.0);
    ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    let all: Vec<f64> = series.iter().flat_map(|(_, d, _, _)| d.iter().copied()).collect();
    let max_v = all.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_v = all.iter().cloned().fold(f64::INFINITY, f64::min).min(0.0);
    let range = (max_v - min_v).max(1.0);
    let cw = w - pl - pr; let ch = height - pt - pb;
    let sx = if labels.len() > 1 { cw / (labels.len() - 1) as f64 } else { cw };
    let by = pt + ch;
    if show_grid { draw_grid(&ctx, labels, max_v, min_v, range, w, height, pl, pr, pt, pb, ch, sx); }
    for (_, data, color, _) in series {
        ctx.begin_path();
        for (i, &v) in data.iter().enumerate() {
            let x = pl + i as f64 * sx;
            let y = pt + ch - ((v - min_v) / range * ch);
            if i == 0 { ctx.move_to(x, y); } else { ctx.line_to(x, y); }
        }
        if !data.is_empty() {
            ctx.line_to(pl + (data.len() - 1) as f64 * sx, by);
            ctx.line_to(pl, by);
        }
        ctx.close_path();
        ctx.set_fill_style_str(&format!("{}33", color)); ctx.fill();
        ctx.set_stroke_style_str(color); ctx.set_line_width(2.5);
        ctx.begin_path();
        for (i, &v) in data.iter().enumerate() {
            let x = pl + i as f64 * sx;
            let y = pt + ch - ((v - min_v) / range * ch);
            if i == 0 { ctx.move_to(x, y); } else { ctx.line_to(x, y); }
        }
        ctx.stroke();
    }
}

#[cfg(feature = "hydrate")]
fn draw_donut(canvas: &web_sys::HtmlCanvasElement, _labels: &[String], series: &[(String, Vec<f64>, String, bool)], height: f64) {
    let Some(ctx) = get_context(canvas) else { return };
    let w = canvas.unchecked_ref::<web_sys::HtmlElement>().offset_width() as f64;
    ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    let cx = w / 2.0; let cy = height / 2.0;
    let radius = (w.min(height) / 2.0 - 20.0).max(10.0);
    let inner = radius * 0.6;
    let data = if let Some((_, d, _, _)) = series.first() { d } else { return };
    let colors = ["#6366f1","#f59e0b","#10b981","#ef4444","#8b5cf6","#06b6d4"];
    let total: f64 = data.iter().sum();
    if total == 0.0 { return; }
    let mut angle = -std::f64::consts::PI / 2.0;
    for (i, &v) in data.iter().enumerate() {
        let sweep = (v / total) * std::f64::consts::PI * 2.0;
        let color = series.get(i).map(|(_, _, c, _)| c.as_str()).unwrap_or(colors[i % colors.len()]);
        ctx.begin_path(); ctx.move_to(cx, cy);
        ctx.arc(cx, cy, radius, angle, angle + sweep).ok();
        ctx.arc_with_anticlockwise(cx, cy, inner, angle + sweep, angle, true).ok();
        ctx.close_path();
        ctx.set_fill_style_str(color); ctx.fill();
        if sweep > 0.3 {
            let mid = angle + sweep / 2.0;
            let lr = (radius + inner) / 2.0;
            ctx.set_fill_style_str("#ffffff"); ctx.set_font("bold 12px system-ui"); ctx.set_text_align("center");
            let _ = ctx.fill_text(&format!("{:.0}%", (v / total * 100.0).round()), cx + mid.cos() * lr, cy + mid.sin() * lr + 4.0);
        }
        angle += sweep;
    }
    ctx.set_fill_style_str("#6b7280"); ctx.set_font("14px system-ui"); ctx.set_text_align("center");
    let _ = ctx.fill_text("Total", cx, cy - 5.0);
    ctx.set_font("bold 20px system-ui"); ctx.set_fill_style_str("#111827");
    let _ = ctx.fill_text(&format!("{:.0}", total), cx, cy + 18.0);
}
