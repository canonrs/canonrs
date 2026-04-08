//! Chart Interaction Engine
//! Canvas rendering, tooltip, legend toggle, resize observer, datatable sync

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlCanvasElement, HtmlElement};

fn read_chart_data(root: &Element) -> String {
    if let Ok(Some(script)) = root.query_selector("script[data-rs-chart-data]") {
        if let Some(text) = script.text_content() { return text; }
    }
    root.get_attribute("data-rs-chart-data").unwrap_or_default()
}

fn parse_chart_data(json: &str) -> Option<(Vec<String>, Vec<(String, Vec<f64>, String, bool)>)> {
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

fn get_context(canvas: &HtmlCanvasElement) -> Option<web_sys::CanvasRenderingContext2d> {
    canvas.get_context("2d").ok()?.and_then(|c| c.dyn_into().ok())
}

fn format_axis_val(v: f64) -> String {
    if v.abs() >= 1_000_000.0 { format!("{:.1}M", v / 1_000_000.0) }
    else if v.abs() >= 1_000.0 { format!("{:.1}k", v / 1_000.0) }
    else { format!("{:.0}", v) }
}

fn dispatch_custom_event(target: &Element, name: &str, detail: &js_sys::Object) {
    let init = web_sys::CustomEventInit::new();
    init.set_bubbles(true);
    init.set_detail(&JsValue::from(detail));
    if let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict(name, &init) {
        target.dispatch_event(&event).ok();
    }
}

fn set_canvas_dpi(canvas: &HtmlCanvasElement, root: &Element, height: f64) {
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

fn draw_chart(canvas: &HtmlCanvasElement, chart_type: &str, labels: &[String], series: &[(String, Vec<f64>, String, bool)], show_grid: bool, height: f64) {
    let active: Vec<(String, Vec<f64>, String, bool)> = series.iter().filter(|(_, _, _, a)| *a).cloned().collect();
    match chart_type {
        "bar"   => draw_bar(canvas, labels, &active, show_grid, height),
        "area"  => draw_area(canvas, labels, &active, show_grid, height),
        "donut" => draw_donut(canvas, labels, &active, height),
        _       => draw_line(canvas, labels, &active, show_grid, height),
    }
}

fn get_css_var(name: &str, fallback: &str) -> String {
    web_sys::window()
        .and_then(|w| w.get_computed_style(&w.document().unwrap().body().unwrap()).ok().flatten())
        .and_then(|s| s.get_property_value(name).ok())
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| fallback.to_string())
}

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
        ctx.begin_path();
        ctx.move_to(pad_l, y);
        ctx.line_to(w - 10.0, y);
        ctx.stroke();
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

fn draw_line(canvas: &HtmlCanvasElement, labels: &[String], series: &[(String, Vec<f64>, String, bool)], show_grid: bool, height: f64) {
    let Some(ctx) = get_context(canvas) else { return };
    let w = canvas.width() as f64 / web_sys::window().unwrap().device_pixel_ratio();
    let h = height;
    let pad_l = 50.0; let pad_b = 40.0;
    ctx.clear_rect(0.0, 0.0, w * web_sys::window().unwrap().device_pixel_ratio(), h * web_sys::window().unwrap().device_pixel_ratio());
    let all_vals: Vec<f64> = series.iter().flat_map(|(_, d, _, _)| d.iter().copied()).collect();
    let min = all_vals.iter().cloned().fold(f64::INFINITY, f64::min).min(0.0);
    let max = all_vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = (max - min).max(1.0);
    if show_grid { draw_grid(&ctx, labels, min, max, pad_l, pad_b, w, h); }
    let n = labels.len();
    let step_x = if n > 1 { (w - pad_l - 20.0) / (n - 1) as f64 } else { w - pad_l - 20.0 };
    for (_, data, color, _) in series {
        ctx.set_stroke_style_str(color);
        ctx.set_line_width(2.0);
        ctx.begin_path();
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
            ctx.begin_path();
            ctx.arc(x, y, 3.5, 0.0, std::f64::consts::TAU).ok();
            ctx.fill();
        }
    }
}

fn draw_area(canvas: &HtmlCanvasElement, labels: &[String], series: &[(String, Vec<f64>, String, bool)], show_grid: bool, height: f64) {
    let Some(ctx) = get_context(canvas) else { return };
    let w = canvas.width() as f64 / web_sys::window().unwrap().device_pixel_ratio();
    let h = height;
    let pad_l = 50.0; let pad_b = 40.0;
    ctx.clear_rect(0.0, 0.0, w * web_sys::window().unwrap().device_pixel_ratio(), h * web_sys::window().unwrap().device_pixel_ratio());
    let all_vals: Vec<f64> = series.iter().flat_map(|(_, d, _, _)| d.iter().copied()).collect();
    let min = all_vals.iter().cloned().fold(f64::INFINITY, f64::min).min(0.0);
    let max = all_vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = (max - min).max(1.0);
    if show_grid { draw_grid(&ctx, labels, min, max, pad_l, pad_b, w, h); }
    let n = labels.len();
    let step_x = if n > 1 { (w - pad_l - 20.0) / (n - 1) as f64 } else { w - pad_l - 20.0 };
    for (_, data, color, _) in series {
        let baseline = h - pad_b;
        ctx.begin_path();
        ctx.move_to(pad_l, baseline);
        for (i, &val) in data.iter().enumerate() {
            let x = pad_l + i as f64 * step_x;
            let y = h - pad_b - ((val - min) / range) * (h - pad_b - 10.0);
            ctx.line_to(x, y);
        }
        ctx.line_to(pad_l + (data.len() - 1) as f64 * step_x, baseline);
        ctx.close_path();
        ctx.set_fill_style_str(&format!("{}33", color));
        ctx.fill();
        ctx.set_stroke_style_str(color);
        ctx.set_line_width(2.0);
        ctx.begin_path();
        for (i, &val) in data.iter().enumerate() {
            let x = pad_l + i as f64 * step_x;
            let y = h - pad_b - ((val - min) / range) * (h - pad_b - 10.0);
            if i == 0 { ctx.move_to(x, y); } else { ctx.line_to(x, y); }
        }
        ctx.stroke();
    }
}

fn draw_bar(canvas: &HtmlCanvasElement, labels: &[String], series: &[(String, Vec<f64>, String, bool)], show_grid: bool, height: f64) {
    let Some(ctx) = get_context(canvas) else { return };
    let w = canvas.width() as f64 / web_sys::window().unwrap().device_pixel_ratio();
    let h = height;
    let pad_l = 50.0; let pad_b = 40.0;
    ctx.clear_rect(0.0, 0.0, w * web_sys::window().unwrap().device_pixel_ratio(), h * web_sys::window().unwrap().device_pixel_ratio());
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
}

fn draw_donut(canvas: &HtmlCanvasElement, labels: &[String], series: &[(String, Vec<f64>, String, bool)], height: f64) {
    let Some(ctx) = get_context(canvas) else { return };
    let w = canvas.width() as f64 / web_sys::window().unwrap().device_pixel_ratio();
    let h = height;
    ctx.clear_rect(0.0, 0.0, w * web_sys::window().unwrap().device_pixel_ratio(), h * web_sys::window().unwrap().device_pixel_ratio());
    let cx = w / 2.0; let cy = h / 2.0;
    let r = (h / 2.0 - 20.0).min(w / 2.0 - 20.0);
    let ir = r * 0.55;
    let first = series.first();
    let (data, colors) = match first {
        Some((_, d, _, _)) => {
            let cs: Vec<String> = series.iter().map(|(_, _, c, _)| c.clone()).collect();
            (d.clone(), cs)
        }
        None => return,
    };
    let total: f64 = data.iter().sum();
    if total == 0.0 { return; }
    let mut start = -std::f64::consts::FRAC_PI_2;
    for (i, &val) in data.iter().enumerate() {
        let sweep = (val / total) * std::f64::consts::TAU;
        ctx.begin_path();
        ctx.move_to(cx, cy);
        ctx.arc(cx, cy, r, start, start + sweep).ok();
        ctx.close_path();
        ctx.set_fill_style_str(colors.get(i).map(|s| s.as_str()).unwrap_or("#6366f1"));
        ctx.fill();
        start += sweep;
    }
    ctx.begin_path();
    ctx.arc(cx, cy, ir, 0.0, std::f64::consts::TAU).ok();
    let bg = get_css_var("--theme-surface-bg", "#0a0a0a");
    ctx.set_fill_style_str(&bg);
    ctx.fill();
    let _ = labels;
}

pub fn init(root: Element) {
    let Ok(Some(canvas_node)) = root.query_selector("[data-rs-chart-canvas]") else { return };
    let Ok(canvas) = canvas_node.dyn_into::<HtmlCanvasElement>() else { return };

    let chart_type = root.get_attribute("data-rs-chart-type").unwrap_or_else(|| "line".to_string());
    let height = root.get_attribute("data-rs-chart-height")
        .and_then(|v| v.parse::<f64>().ok()).unwrap_or(320.0);
    let show_grid = root.get_attribute("data-rs-chart-grid").as_deref() != Some("hidden");
    let show_legend = root.get_attribute("data-rs-chart-legend").as_deref() != Some("hidden");
    let sync_table = root.get_attribute("data-rs-chart-sync-table").filter(|s| !s.is_empty());

    let json = read_chart_data(&root);
    let Some((labels, series)) = parse_chart_data(&json) else { return };

    set_canvas_dpi(&canvas, &root, height);
    draw_chart(&canvas, &chart_type, &labels, &series, show_grid, height);

    // legend
    if show_legend {
        if let Ok(Some(legend_el)) = root.query_selector("[data-rs-chart-legend]") {
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
                let ct_c = chart_type.clone(); let item_c = item.clone();
                let legend_c = legend_el.clone(); let labels_c = labels.clone();
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
    }

    // tooltip + crosshair
    {
        let pad_l = 50.0;
        let chart_w = canvas.unchecked_ref::<HtmlElement>().offset_width() as f64 - pad_l - 20.0;
        let n = labels.len();
        let step_x = if n > 1 { chart_w / (n - 1) as f64 } else { chart_w };
        let canvas_c = canvas.clone(); let root_c = root.clone();
        let labels_c = labels.clone(); let series_c = series.clone();
        let ct_c = chart_type.clone(); let sync_c = sync_table.clone();

        let on_move = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let rect = canvas_c.get_bounding_client_rect();
            let mx = e.client_x() as f64 - rect.left() - pad_l;
            if mx < 0.0 { return; }
            let idx = ((mx / step_x).round() as usize).min(labels_c.len().saturating_sub(1));
            if let Ok(Some(tooltip)) = root_c.query_selector("[data-rs-chart-tooltip]") {
                let mut html = format!("<div style=\"font-weight:600;margin-bottom:4px;\">{}</div>", labels_c.get(idx).map(|s| s.as_str()).unwrap_or(""));
                for (name, data, color, active) in &series_c {
                    if !active { continue; }
                    if let Some(&val) = data.get(idx) {
                        html.push_str(&format!("<div style=\"display:flex;align-items:center;gap:6px;\"><span style=\"background:{};width:8px;height:8px;border-radius:50%;display:inline-block;\"></span><span>{}: </span><strong>{:.1}</strong></div>", color, name, val));
                    }
                }
                tooltip.set_inner_html(&html);
                let x = pad_l + idx as f64 * step_x;
                tooltip.unchecked_ref::<HtmlElement>().style().set_property("left", &format!("{}px", x + 12.0)).ok();
                tooltip.unchecked_ref::<HtmlElement>().style().set_property("top", "20px").ok();
                tooltip.set_attribute("data-rs-state", "open").ok();
            }
            if let Ok(Some(ch)) = root_c.query_selector("[data-rs-chart-crosshair]") {
                let x = pad_l + idx as f64 * step_x;
                ch.unchecked_ref::<HtmlElement>().style().set_property("left", &format!("{}px", x)).ok();
                ch.set_attribute("data-rs-state", "open").ok();
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
            let detail = js_sys::Object::new();
            dispatch_custom_event(&root_c2, "canon:chart:leave", &detail);
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback("mousemove", on_move.as_ref().unchecked_ref()).ok();
        canvas.add_event_listener_with_callback("mouseleave", on_leave.as_ref().unchecked_ref()).ok();
        on_move.forget(); on_leave.forget();
    }

    // resize observer
    {
        let canvas_c = canvas.clone(); let root_c = root.clone();
        let labels_c = labels.clone(); let series_c = series.clone();
        let ct_c = chart_type.clone();
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
                js_sys::Reflect::apply(&observe_fn.unchecked_into::<js_sys::Function>(), &observer, &js_sys::Array::of1(&root)).ok();
            }
        }
        obs_cb.forget();
    }

    // datatable sync
    if let Some(ref _table_id) = sync_table {
        let canvas_c = canvas.clone(); let root_c = root.clone();
        let labels_c = labels.clone(); let series_c = series.clone();
        let ct_c = chart_type.clone();
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
                t.unchecked_ref::<HtmlElement>().style().set_property("left", &format!("{}px", x + 12.0)).ok();
                t.unchecked_ref::<HtmlElement>().style().set_property("top", "20px").ok();
                t.set_attribute("data-rs-state", "visible").ok();
            }
            if let Ok(Some(c)) = root_c.query_selector("[data-rs-chart-crosshair]") {
                c.unchecked_ref::<HtmlElement>().style().set_property("left", &format!("{}px", x)).ok();
                c.set_attribute("data-rs-state", "visible").ok();
            }
        }) as Box<dyn FnMut(_)>);
        let root_c2 = root.clone(); let canvas_c2 = canvas.clone();
        let labels_c2 = labels.clone(); let series_c2 = series.clone(); let ct_c2 = chart_type.clone();
        let on_leave = Closure::wrap(Box::new(move |_: web_sys::Event| {
            draw_chart(&canvas_c2, &ct_c2, &labels_c2, &series_c2, show_grid, height);
            if let Ok(Some(t)) = root_c2.query_selector("[data-rs-chart-tooltip]") { t.set_attribute("data-rs-state", "hidden").ok(); }
            if let Ok(Some(c)) = root_c2.query_selector("[data-rs-chart-crosshair]") { c.set_attribute("data-rs-state", "hidden").ok(); }
        }) as Box<dyn FnMut(_)>);
        root.add_event_listener_with_callback("canon:datatable:hover", on_hover.as_ref().unchecked_ref()).ok();
        root.add_event_listener_with_callback("canon:datatable:leave", on_leave.as_ref().unchecked_ref()).ok();
        on_hover.forget(); on_leave.forget();
    }
}

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-chart-legend-item]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init(el); }
        }
    }
}
