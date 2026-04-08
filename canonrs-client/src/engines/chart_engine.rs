//! Chart Engine — rendering, parsing, layout
//! Chamado pelo interaction/chart.rs — nunca diretamente pelo island

use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlCanvasElement, HtmlElement};

// ─── Data types ───────────────────────────────────────────────────────────────

pub type Series = Vec<(String, Vec<f64>, String, bool)>;

// ─── Parse ────────────────────────────────────────────────────────────────────

pub fn read_chart_data(root: &Element) -> String {
    if let Ok(Some(script)) = root.query_selector("script[data-rs-chart-data]") {
        if let Some(text) = script.text_content() { return text; }
    }
    root.get_attribute("data-rs-chart-data").unwrap_or_default()
}

pub fn parse_chart_data(json: &str) -> Option<(Vec<String>, Series)> {
    let labels = extract_json_array(json, "labels")?;
    let series_json = extract_json_key(json, "series")?;
    let colors = ["#6366f1","#f59e0b","#10b981","#ef4444","#8b5cf6","#06b6d4"];
    let mut series = Vec::new();
    for (i, item) in split_json_objects(&series_json).iter().enumerate() {
        let name  = extract_json_string(item, "name").unwrap_or_else(|| format!("Series {}", i+1));
        let data  = extract_json_number_array(item, "data").unwrap_or_default();
        let color = extract_json_string(item, "color")
            .filter(|c| !c.is_empty())
            .unwrap_or_else(|| colors[i % colors.len()].to_string());
        series.push((name, data, color, true));
    }
    Some((labels, series))
}

fn extract_json_string(json: &str, key: &str) -> Option<String> {
    let p = format!("\"{}\":", key);
    let s = json.find(&p)? + p.len();
    let r = json[s..].trim_start();
    if r.starts_with('"') { let e = r[1..].find('"')? + 1; Some(r[1..e].to_string()) } else { None }
}

fn extract_json_key(json: &str, key: &str) -> Option<String> {
    let p = format!("\"{}\":", key);
    let s = json.find(&p)? + p.len();
    let r = json[s..].trim_start();
    let (open, close) = if r.starts_with('[') { ('[', ']') } else { ('{', '}') };
    let mut depth = 0; let mut end = 0;
    for (i, c) in r.chars().enumerate() {
        if c == open { depth += 1; }
        if c == close { depth -= 1; if depth == 0 { end = i + 1; break; } }
    }
    if end > 0 { Some(r[..end].to_string()) } else { None }
}

fn extract_json_array(json: &str, key: &str) -> Option<Vec<String>> {
    let arr = extract_json_key(json, key)?;
    let inner = arr.trim_start_matches('[').trim_end_matches(']');
    Some(inner.split(',').map(|s| s.trim().trim_matches('"').to_string()).filter(|s| !s.is_empty()).collect())
}

fn extract_json_number_array(json: &str, key: &str) -> Option<Vec<f64>> {
    let arr = extract_json_key(json, key)?;
    let inner = arr.trim_start_matches('[').trim_end_matches(']');
    Some(inner.split(',').filter_map(|s| s.trim().parse::<f64>().ok()).collect())
}

fn split_json_objects(json: &str) -> Vec<String> {
    let inner = json.trim_start_matches('[').trim_end_matches(']');
    let mut out = Vec::new(); let mut depth = 0; let mut start = 0;
    for (i, c) in inner.chars().enumerate() {
        match c {
            '{' => { if depth == 0 { start = i; } depth += 1; }
            '}' => { depth -= 1; if depth == 0 { out.push(inner[start..=i].to_string()); } }
            _ => {}
        }
    }
    out
}

// ─── Canvas setup ─────────────────────────────────────────────────────────────

pub fn get_context(canvas: &HtmlCanvasElement) -> Option<web_sys::CanvasRenderingContext2d> {
    canvas.get_context("2d").ok()?.and_then(|c| c.dyn_into().ok())
}

pub fn set_canvas_dpi(canvas: &HtmlCanvasElement, root: &Element, height: f64) {
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
    let style = canvas.unchecked_ref::<HtmlElement>().style();
    let _ = style.set_property("width", &format!("{}px", w));
    let _ = style.set_property("height", &format!("{}px", height));
    if let Some(ctx) = get_context(canvas) { ctx.scale(dpr, dpr).ok(); }
}

// ─── Draw dispatch ────────────────────────────────────────────────────────────

pub fn draw_chart(canvas: &HtmlCanvasElement, chart_type: &str, labels: &[String], series: &Series, show_grid: bool, height: f64) {
    let active: Series = series.iter().filter(|(_, _, _, a)| *a).cloned().collect();
    match chart_type {
        "bar"   => draw_bar(canvas, labels, &active, show_grid, height),
        "area"  => draw_area(canvas, labels, &active, show_grid, height),
        "donut" => draw_donut(canvas, labels, &active, height),
        _       => draw_line(canvas, labels, &active, show_grid, height),
    }
}

// ─── CSS vars ─────────────────────────────────────────────────────────────────

pub fn get_css_var(name: &str, fallback: &str) -> String {
    web_sys::window()
        .and_then(|w| w.get_computed_style(&w.document().unwrap().body().unwrap()).ok().flatten())
        .and_then(|s| s.get_property_value(name).ok())
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| fallback.to_string())
}

fn format_axis_val(v: f64) -> String {
    if v.abs() >= 1_000_000.0 { format!("{:.1}M", v / 1_000_000.0) }
    else if v.abs() >= 1_000.0 { format!("{:.1}k", v / 1_000.0) }
    else { format!("{:.0}", v) }
}

// ─── Grid ─────────────────────────────────────────────────────────────────────

fn draw_grid(ctx: &web_sys::CanvasRenderingContext2d, labels: &[String], min: f64, max: f64, pad_l: f64, pad_b: f64, w: f64, h: f64) {
    let steps = 4;
    let grid_color = get_css_var("--theme-surface-border", "rgba(255,255,255,0.08)");
    let text_color = get_css_var("--theme-surface-fg-muted", "rgba(255,255,255,0.4)");
    ctx.set_stroke_style_str(&grid_color);
    ctx.set_fill_style_str(&text_color);
    ctx.set_font("11px var(--font-sans, system-ui)");
    ctx.set_line_width(0.5);
    for i in 0..=steps {
        let y = h - pad_b - (i as f64 / steps as f64) * (h - pad_b - 10.0);
        ctx.begin_path(); ctx.move_to(pad_l, y); ctx.line_to(w - 10.0, y); ctx.stroke();
        let val = min + (i as f64 / steps as f64) * (max - min);
        ctx.set_text_align("right");
        ctx.fill_text(&format_axis_val(val), pad_l - 6.0, y + 4.0).ok();
    }
    let n = labels.len();
    let step_x = if n > 1 { (w - pad_l - 20.0) / (n - 1) as f64 } else { w - pad_l - 20.0 };
    for (i, label) in labels.iter().enumerate() {
        let x = pad_l + i as f64 * step_x;
        ctx.set_text_align("center");
        ctx.fill_text(label, x, h - pad_b + 16.0).ok();
    }
}

// ─── Line ─────────────────────────────────────────────────────────────────────

fn draw_line(canvas: &HtmlCanvasElement, labels: &[String], series: &Series, show_grid: bool, height: f64) {
    let Some(ctx) = get_context(canvas) else { return };
    let dpr = web_sys::window().unwrap().device_pixel_ratio();
    let w = canvas.width() as f64 / dpr; let h = height;
    let pad_l = 50.0; let pad_b = 40.0;
    ctx.clear_rect(0.0, 0.0, w * dpr, h * dpr);
    let all_vals: Vec<f64> = series.iter().flat_map(|(_, d, _, _)| d.iter().copied()).collect();
    let min = all_vals.iter().cloned().fold(f64::INFINITY, f64::min).min(0.0);
    let max = all_vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = (max - min).max(1.0);
    if show_grid { draw_grid(&ctx, labels, min, max, pad_l, pad_b, w, h); }
    let n = labels.len();
    let step_x = if n > 1 { (w - pad_l - 20.0) / (n - 1) as f64 } else { w - pad_l - 20.0 };
    for (_, data, color, _) in series {
        ctx.set_stroke_style_str(color); ctx.set_line_width(2.0); ctx.begin_path();
        for (i, &val) in data.iter().enumerate() {
            let x = pad_l + i as f64 * step_x;
            let y = h - pad_b - ((val - min) / range) * (h - pad_b - 10.0);
            if i == 0 { ctx.move_to(x, y); } else { ctx.line_to(x, y); }
        }
        ctx.stroke();
        ctx.set_fill_style_str(color);
        for (i, &val) in data.iter().enumerate() {
            let x = pad_l + i as f64 * step_x;
            let y = h - pad_b - ((val - min) / range) * (h - pad_b - 10.0);
            ctx.begin_path(); ctx.arc(x, y, 3.5, 0.0, std::f64::consts::TAU).ok(); ctx.fill();
        }
    }
}

// ─── Area ─────────────────────────────────────────────────────────────────────

fn draw_area(canvas: &HtmlCanvasElement, labels: &[String], series: &Series, show_grid: bool, height: f64) {
    let Some(ctx) = get_context(canvas) else { return };
    let dpr = web_sys::window().unwrap().device_pixel_ratio();
    let w = canvas.width() as f64 / dpr; let h = height;
    let pad_l = 50.0; let pad_b = 40.0;
    ctx.clear_rect(0.0, 0.0, w * dpr, h * dpr);
    let all_vals: Vec<f64> = series.iter().flat_map(|(_, d, _, _)| d.iter().copied()).collect();
    let min = all_vals.iter().cloned().fold(f64::INFINITY, f64::min).min(0.0);
    let max = all_vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = (max - min).max(1.0);
    if show_grid { draw_grid(&ctx, labels, min, max, pad_l, pad_b, w, h); }
    let n = labels.len();
    let step_x = if n > 1 { (w - pad_l - 20.0) / (n - 1) as f64 } else { w - pad_l - 20.0 };
    for (_, data, color, _) in series {
        let baseline = h - pad_b;
        ctx.begin_path(); ctx.move_to(pad_l, baseline);
        for (i, &val) in data.iter().enumerate() {
            let x = pad_l + i as f64 * step_x;
            let y = h - pad_b - ((val - min) / range) * (h - pad_b - 10.0);
            ctx.line_to(x, y);
        }
        ctx.line_to(pad_l + (data.len() - 1) as f64 * step_x, baseline);
        ctx.close_path();
        ctx.set_fill_style_str(&format!("{}33", color)); ctx.fill();
        ctx.set_stroke_style_str(color); ctx.set_line_width(2.0); ctx.begin_path();
        for (i, &val) in data.iter().enumerate() {
            let x = pad_l + i as f64 * step_x;
            let y = h - pad_b - ((val - min) / range) * (h - pad_b - 10.0);
            if i == 0 { ctx.move_to(x, y); } else { ctx.line_to(x, y); }
        }
        ctx.stroke();
    }
}

// ─── Bar ──────────────────────────────────────────────────────────────────────

fn draw_bar(canvas: &HtmlCanvasElement, labels: &[String], series: &Series, show_grid: bool, height: f64) {
    let Some(ctx) = get_context(canvas) else { return };
    let dpr = web_sys::window().unwrap().device_pixel_ratio();
    let w = canvas.width() as f64 / dpr; let h = height;
    let pad_l = 50.0; let pad_b = 40.0;
    ctx.clear_rect(0.0, 0.0, w * dpr, h * dpr);
    let all_vals: Vec<f64> = series.iter().flat_map(|(_, d, _, _)| d.iter().copied()).collect();
    let max = all_vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = max.max(1.0);
    if show_grid { draw_grid(&ctx, labels, 0.0, max, pad_l, pad_b, w, h); }
    let n = labels.len();
    let total_w = w - pad_l - 20.0;
    let group_w = total_w / n as f64;
    let bar_w = (group_w / series.len() as f64 * 0.7).max(4.0);
    for (si, (_, data, color, _)) in series.iter().enumerate() {
        ctx.set_fill_style_str(color);
        for (i, &val) in data.iter().enumerate() {
            let bh = (val / range) * (h - pad_b - 10.0);
            let x = pad_l + i as f64 * group_w + si as f64 * bar_w + group_w * 0.15;
            let y = h - pad_b - bh;
            ctx.fill_rect(x, y, bar_w, bh);
        }
    }
    let _ = labels;
}

// ─── Donut ────────────────────────────────────────────────────────────────────

fn draw_donut(canvas: &HtmlCanvasElement, labels: &[String], series: &Series, height: f64) {
    let Some(ctx) = get_context(canvas) else { return };
    let dpr = web_sys::window().unwrap().device_pixel_ratio();
    let w = canvas.width() as f64 / dpr; let h = height;
    ctx.clear_rect(0.0, 0.0, w * dpr, h * dpr);
    let cx = w / 2.0; let cy = h / 2.0;
    let r = (h / 2.0 - 20.0).min(w / 2.0 - 20.0);
    let ir = r * 0.55;
    let first = series.first();
    let (data, colors) = match first {
        Some((_, d, _, _)) => (d.clone(), series.iter().map(|(_, _, c, _)| c.clone()).collect::<Vec<_>>()),
        None => return,
    };
    let total: f64 = data.iter().sum();
    if total == 0.0 { return; }
    let mut start = -std::f64::consts::FRAC_PI_2;
    for (i, &val) in data.iter().enumerate() {
        let sweep = (val / total) * std::f64::consts::TAU;
        ctx.begin_path(); ctx.move_to(cx, cy);
        ctx.arc(cx, cy, r, start, start + sweep).ok();
        ctx.close_path();
        ctx.set_fill_style_str(colors.get(i).map(|s| s.as_str()).unwrap_or("#6366f1"));
        ctx.fill();
        start += sweep;
    }
    ctx.begin_path(); ctx.arc(cx, cy, ir, 0.0, std::f64::consts::TAU).ok();
    ctx.set_fill_style_str(&get_css_var("--theme-surface-bg", "#0a0a0a"));
    ctx.fill();
    let _ = labels;
}
