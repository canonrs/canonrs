//! Chart Behavior - Enterprise Canvas Engine
//! DPI scaling, ResizeObserver, token-based tooltip/legend, Chart↔DataTable sync

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
    register_behavior("data-chart", Box::new(|element_id, _state| {
        let Some(root) = document().get_element_by_id(element_id) else {
            return Ok(());
        };

        let chart_type = root.get_attribute("data-chart-type").unwrap_or_else(|| "line".to_string());
        let height: f64 = root.get_attribute("data-chart-height")
            .and_then(|h| h.parse().ok()).unwrap_or(320.0);
        let show_grid  = root.get_attribute("data-chart-grid").as_deref()    != Some("false");
        let show_legend= root.get_attribute("data-chart-legend").as_deref()  != Some("false");
        let show_tooltip=root.get_attribute("data-chart-tooltip").as_deref() != Some("false");
        let sync_table = root.get_attribute("data-chart-sync-table");

        // Read data from <script type="application/json" data-chart-data>
        let data_json = read_chart_data(&root);

        let canvas_id = format!("{}-canvas", element_id);
        let Some(canvas_el) = document().get_element_by_id(&canvas_id) else {
            return Ok(());
        };
        let canvas: web_sys::HtmlCanvasElement = canvas_el.dyn_into()
            .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "canvas cast".into() })?;

        // DPI scaling
        set_canvas_dpi(&canvas, &root, height);

        let parsed = parse_chart_data(&data_json);
        let (labels, series) = match parsed {
            Some(d) => d,
            None => return Ok(()),
        };

        draw_chart(&canvas, &chart_type, &labels, &series, show_grid, height);

        if show_legend {
            if let Some(legend_el) = root.query_selector("[data-chart-legend-el]").ok().flatten() {
                draw_legend(&legend_el, &series, &root, &canvas, &chart_type, &labels, show_grid, height);
            }
        }

        if show_tooltip {
            setup_tooltip(&canvas, &root, &labels, &series, height, sync_table.clone());
        }

        setup_resize_observer(&canvas, &root, &labels, &series, &chart_type, show_grid, height);

        if let Some(table_id) = sync_table {
            setup_datatable_to_chart_sync(&root, &canvas, &table_id, &labels, &series, &chart_type, show_grid, height);
        }

        Ok(())
    }));
}

// ─── DATA READ ───────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn read_chart_data(root: &web_sys::Element) -> String {
    // Prefer <script type="application/json" data-chart-data>
    if let Ok(Some(script)) = root.query_selector("script[data-chart-data]") {
        if let Some(text) = script.text_content() {
            return text;
        }
    }
    // Fallback: data attribute
    root.get_attribute("data-chart-data").unwrap_or_default()
}

// ─── DPI SCALING ─────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn set_canvas_dpi(canvas: &web_sys::HtmlCanvasElement, root: &web_sys::Element, height: f64) {
    let window = web_sys::window().unwrap();
    let dpr = window.device_pixel_ratio();
    let parent_width = root.client_width() as f64;
    let w = if parent_width > 0.0 { parent_width } else { 600.0 };

    canvas.set_width((w * dpr) as u32);
    canvas.set_height((height * dpr) as u32);

    let style = canvas.unchecked_ref::<web_sys::HtmlElement>().style();
    let _ = style.set_property("width", &format!("{}px", w));
    let _ = style.set_property("height", &format!("{}px", height));

    if let Some(ctx) = get_context(canvas) {
        ctx.scale(dpr, dpr).ok();
    }
}

// ─── DRAW ROUTER ─────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn draw_chart(
    canvas: &web_sys::HtmlCanvasElement,
    chart_type: &str,
    labels: &[String],
    series: &[(String, Vec<f64>, String, bool)],
    show_grid: bool,
    height: f64,
) {
    let active: Vec<(String, Vec<f64>, String, bool)> = series.iter()
        .filter(|(_, _, _, active)| *active)
        .cloned()
        .collect();
    match chart_type {
        "bar"   => draw_bar(canvas, labels, &active, show_grid, height),
        "area"  => draw_area(canvas, labels, &active, show_grid, height),
        "donut" => draw_donut(canvas, labels, &active, height),
        _       => draw_line(canvas, labels, &active, show_grid, height),
    }
}

// ─── PARSE ───────────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
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

// ─── LEGEND (token-based, interactive) ───────────────────────────────────────

#[cfg(feature = "hydrate")]
fn draw_legend(
    legend_el: &web_sys::Element,
    series: &[(String, Vec<f64>, String, bool)],
    root: &web_sys::Element,
    canvas: &web_sys::HtmlCanvasElement,
    chart_type: &str,
    labels: &[String],
    show_grid: bool,
    height: f64,
) {
    legend_el.set_inner_html("");
    for (i, (name, _, color, active)) in series.iter().enumerate() {
        let item = document().create_element("span").unwrap();
        item.set_attribute("data-chart-legend-item", "").ok();
        item.set_attribute("data-series-index", &i.to_string()).ok();
        item.set_attribute("data-state", if *active { "active" } else { "inactive" }).ok();

        let dot = document().create_element("span").unwrap();
        dot.set_attribute("data-chart-legend-dot", "").ok();
        let _ = dot.unchecked_ref::<web_sys::HtmlElement>().style()
            .set_property("background", color);

        let label = document().create_element("span").unwrap();
        label.set_text_content(Some(name));

        item.append_child(&dot).ok();
        item.append_child(&label).ok();
        legend_el.append_child(&item).ok();

        // Click → toggle série
        let canvas_c    = canvas.clone();
        let root_c      = root.clone();
        let chart_type_c= chart_type.to_string();
        let labels_c    = labels.to_vec();
        let item_c      = item.clone();
        let legend_c    = legend_el.clone();

        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let idx = item_c.get_attribute("data-series-index")
                .and_then(|v| v.parse::<usize>().ok()).unwrap_or(0);
            let current = item_c.get_attribute("data-state").unwrap_or_default();
            let new_state = if current == "active" { "inactive" } else { "active" };
            item_c.set_attribute("data-state", new_state).ok();

            // Re-parse e redraw com estados atualizados
            let data_json = read_chart_data(&root_c);
            if let Some((lbl, mut ser)) = parse_chart_data(&data_json) {
                // Sync active state from legend DOM
                let items = legend_c.query_selector_all("[data-chart-legend-item]").unwrap();
                for j in 0..items.length() {
                    if let Some(el) = items.item(j).and_then(|e| e.dyn_into::<web_sys::Element>().ok()) {
                        let si = el.get_attribute("data-series-index")
                            .and_then(|v| v.parse::<usize>().ok()).unwrap_or(0);
                        let active = el.get_attribute("data-state").as_deref() == Some("active");
                        if let Some(s) = ser.get_mut(si) { s.3 = active; }
                    }
                }
                draw_chart(&canvas_c, &chart_type_c, &lbl, &ser, true, 320.0);
            }
        }) as Box<dyn FnMut(_)>);

        item.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
        closure.forget();
    }
}

// ─── TOOLTIP (data-state, sem inline styles de posição exceto left/top) ──────

#[cfg(feature = "hydrate")]
fn setup_tooltip(
    canvas: &web_sys::HtmlCanvasElement,
    root: &web_sys::Element,
    labels: &[String],
    series: &[(String, Vec<f64>, String, bool)],
    height: f64,
    sync_table: Option<String>,
) {
    let pad_l  = 50.0;
    let chart_w = canvas.unchecked_ref::<web_sys::HtmlElement>()
        .offset_width() as f64 - pad_l - 20.0;
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

        let Some(tooltip) = root_c.query_selector("[data-chart-tooltip-el]").ok().flatten() else { return };
        let tooltip_el: &web_sys::HtmlElement = tooltip.unchecked_ref();

        // Build HTML usando apenas tokens via CSS classes
        let mut html = format!(
            r#"<div style="font-weight:600;margin-bottom:4px;">{}</div>"#,
            labels_d.get(idx).map(|s| s.as_str()).unwrap_or("")
        );
        for (name, data, color, active) in &series_d {
            if !active { continue; }
            if let Some(&val) = data.get(idx) {
                html.push_str(&format!(
                    r#"<div style="display:flex;align-items:center;gap:6px;">
                        <span data-chart-legend-dot style="background:{};width:8px;height:8px;border-radius:50%;display:inline-block;"></span>
                        <span>{}: </span><strong>{:.1}</strong>
                    </div>"#,
                    color, name, val
                ));
            }
        }
        tooltip.set_inner_html(&html);

        let x = pad_l + idx as f64 * step_x;
        let _ = tooltip_el.style().set_property("left", &format!("{}px", x + 12.0));
        let _ = tooltip_el.style().set_property("top", "20px");
        tooltip.set_attribute("data-state", "visible").ok();

        // Dispatch canon:chart:hover
        // Dispatch via JS interop (web-sys sem CustomEventInit)
        let detail = js_sys::Object::new();
        js_sys::Reflect::set(&detail, &JsValue::from_str("index"), &JsValue::from_f64(idx as f64)).ok();
        dispatch_custom_event("canon:chart:hover", &detail);

        // Crosshair
        if let Some(crosshair) = root_c.query_selector("[data-chart-crosshair]").ok().flatten() {
            let ch: &web_sys::HtmlElement = crosshair.unchecked_ref();
            let _ = ch.style().set_property("left", &format!("{}px", x));
            crosshair.set_attribute("data-state", "visible").ok();
        }

        // Sync datatable highlight
        if let Some(ref table_id) = sync_c {
            if let Some(table) = document().get_element_by_id(table_id) {
                let rows = table.query_selector_all("[data-datatable-row]").unwrap();
                for i in 0..rows.length() {
                    if let Some(row) = rows.item(i).and_then(|r| r.dyn_into::<web_sys::Element>().ok()) {
                        let row_idx = row.get_attribute("data-row-index")
                            .and_then(|v| v.parse::<usize>().ok()).unwrap_or(usize::MAX);
                        if row_idx == idx {
                            row.set_attribute("data-chart-highlight", "").ok();
                        } else {
                            row.remove_attribute("data-chart-highlight").ok();
                        }
                    }
                }
            }
        }
    }) as Box<dyn FnMut(_)>);

    canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref()).ok();
    closure.forget();

    // Hide on leave
    let root_c2 = root.clone();
    let sync_c2 = sync_table;
    let hide = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
        if let Some(t) = root_c2.query_selector("[data-chart-tooltip-el]").ok().flatten() {
            t.set_attribute("data-state", "hidden").ok();
            // Dispatch canon:chart:leave
            if let Ok(ev) = web_sys::CustomEvent::new("canon:chart:leave") {
                document().dispatch_event(&ev).ok();
            }
        }
        if let Some(c) = root_c2.query_selector("[data-chart-crosshair]").ok().flatten() {
            c.set_attribute("data-state", "hidden").ok();
        }
        if let Some(ref table_id) = sync_c2 {
            if let Some(table) = document().get_element_by_id(table_id) {
                let rows = table.query_selector_all("[data-datatable-row][data-chart-highlight]").unwrap();
                for i in 0..rows.length() {
                    if let Some(r) = rows.item(i).and_then(|r| r.dyn_into::<web_sys::Element>().ok()) {
                        r.remove_attribute("data-chart-highlight").ok();
                    }
                }
            }
        }
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mouseleave", hide.as_ref().unchecked_ref()).ok();
    hide.forget();
}

// ─── DATATABLE → CHART SYNC ──────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn setup_datatable_to_chart_sync(
    root: &web_sys::Element,
    canvas: &web_sys::HtmlCanvasElement,
    _table_id: &str,
    labels: &[String],
    series: &[(String, Vec<f64>, String, bool)],
    chart_type: &str,
    show_grid: bool,
    height: f64,
) {
    // Escuta canon:datatable:hover — disparado pelo chart_sync_behavior
    let canvas_c     = canvas.clone();
    let root_c       = root.clone();
    let labels_c     = labels.to_vec();
    let series_c     = series.to_vec();
    let chart_type_c = chart_type.to_string();

    let on_hover = Closure::wrap(Box::new(move |e: web_sys::CustomEvent| {
        let detail = e.detail();
        let idx = js_sys::Reflect::get(&detail, &JsValue::from_str("index"))
            .ok().and_then(|v| v.as_f64()).map(|f| f as usize)
            .unwrap_or(usize::MAX);
        if idx == usize::MAX { return; }

        draw_chart(&canvas_c, &chart_type_c, &labels_c, &series_c, show_grid, height);
        draw_crosshair_on_canvas(&canvas_c, idx, &labels_c, height);

        if let Some(t) = root_c.query_selector("[data-chart-tooltip-el]").ok().flatten() {
            let pad_l = 50.0;
            let w = canvas_c.unchecked_ref::<web_sys::HtmlElement>().offset_width() as f64;
            let step_x = if labels_c.len() > 1 { (w - pad_l - 20.0) / (labels_c.len() - 1) as f64 } else { w };
            let x = pad_l + idx as f64 * step_x;
            let te: &web_sys::HtmlElement = t.unchecked_ref();
            let _ = te.style().set_property("left", &format!("{}px", x + 12.0));
            let _ = te.style().set_property("top", "20px");
            t.set_attribute("data-state", "visible").ok();
        }
        if let Some(c) = root_c.query_selector("[data-chart-crosshair]").ok().flatten() {
            let pad_l = 50.0;
            let w = canvas_c.unchecked_ref::<web_sys::HtmlElement>().offset_width() as f64;
            let step_x = if labels_c.len() > 1 { (w - pad_l - 20.0) / (labels_c.len() - 1) as f64 } else { w };
            let x = pad_l + idx as f64 * step_x;
            let ce: &web_sys::HtmlElement = c.unchecked_ref();
            let _ = ce.style().set_property("left", &format!("{}px", x));
            c.set_attribute("data-state", "visible").ok();
        }
    }) as Box<dyn FnMut(_)>);

    let on_leave = Closure::wrap(Box::new({
        let canvas_c2     = canvas.clone();
        let root_c2       = root.clone();
        let labels_c2     = labels.to_vec();
        let series_c2     = series.to_vec();
        let chart_type_c2 = chart_type.to_string();
        move |_: web_sys::Event| {
            draw_chart(&canvas_c2, &chart_type_c2, &labels_c2, &series_c2, show_grid, height);
            if let Some(t) = root_c2.query_selector("[data-chart-tooltip-el]").ok().flatten() {
                t.set_attribute("data-state", "hidden").ok();
            }
            if let Some(c) = root_c2.query_selector("[data-chart-crosshair]").ok().flatten() {
                c.set_attribute("data-state", "hidden").ok();
            }
        }
    }) as Box<dyn FnMut(_)>);

    document().add_event_listener_with_callback("canon:datatable:hover", on_hover.as_ref().unchecked_ref()).ok();
    document().add_event_listener_with_callback("canon:datatable:leave", on_leave.as_ref().unchecked_ref()).ok();
    on_hover.forget();
    on_leave.forget();
}


#[cfg(feature = "hydrate")]
fn draw_crosshair_on_canvas(
    canvas: &web_sys::HtmlCanvasElement,
    idx: usize,
    labels: &[String],
    height: f64,
) {
    let Some(ctx) = get_context(canvas) else { return };
    let pad_l = 50.0;
    let pad_b = 40.0;
    let w = canvas.unchecked_ref::<web_sys::HtmlElement>().offset_width() as f64;
    let chart_w = w - pad_l - 20.0;
    let step_x = if labels.len() > 1 { chart_w / (labels.len() - 1) as f64 } else { chart_w };
    let x = pad_l + idx as f64 * step_x;

    ctx.set_stroke_style_str("#9ca3af");
    ctx.set_line_width(1.0);
    ctx.set_line_dash(&js_sys::Array::of2(&JsValue::from(4.0), &JsValue::from(4.0))).ok();
    ctx.begin_path();
    ctx.move_to(x, 20.0);
    ctx.line_to(x, height - pad_b);
    ctx.stroke();
    ctx.set_line_dash(&js_sys::Array::new()).ok();
}

// ─── RESIZE OBSERVER ─────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn setup_resize_observer(
    canvas: &web_sys::HtmlCanvasElement,
    root: &web_sys::Element,
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

    let closure = Closure::wrap(Box::new(move || {
        set_canvas_dpi(&canvas_c, &root_c, height);
        draw_chart(&canvas_c, &chart_type_c, &labels_c, &series_c, show_grid, height);
    }) as Box<dyn Fn()>);

    // ResizeObserver via JS interop
    let observer_cb = Closure::wrap(Box::new(move |_entries: js_sys::Array| {
        closure.as_ref().unchecked_ref::<js_sys::Function>().call0(&JsValue::NULL).ok();
    }) as Box<dyn FnMut(js_sys::Array)>);

    if let Ok(observer_ctor) = js_sys::Reflect::get(&web_sys::window().unwrap(), &JsValue::from_str("ResizeObserver")) {
        if let Ok(observer) = js_sys::Reflect::construct(
            &observer_ctor.unchecked_into::<js_sys::Function>(),
            &js_sys::Array::of1(observer_cb.as_ref())
        ) {
            let observe_fn = js_sys::Reflect::get(&observer, &JsValue::from_str("observe")).unwrap();
            js_sys::Reflect::apply(
                &observe_fn.unchecked_into::<js_sys::Function>(),
                &observer,
                &js_sys::Array::of1(root)
            ).ok();
        }
    }
    observer_cb.forget();
}

// ─── CONTEXT ─────────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn get_context(canvas: &web_sys::HtmlCanvasElement) -> Option<web_sys::CanvasRenderingContext2d> {
    canvas.get_context("2d").ok()?.and_then(|c| c.dyn_into().ok())
}

// ─── LINE ────────────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn draw_line(canvas: &web_sys::HtmlCanvasElement, labels: &[String], series: &[(String, Vec<f64>, String, bool)], show_grid: bool, height: f64) {
    let Some(ctx) = get_context(canvas) else { return };
    let w = canvas.unchecked_ref::<web_sys::HtmlElement>().offset_width() as f64;
    let h = height;
    let (pl, pr, pt, pb) = (50.0, 20.0, 20.0, 40.0);
    ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    let all: Vec<f64> = series.iter().flat_map(|(_, d, _, _)| d.iter().copied()).collect();
    let max_v = all.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_v = all.iter().cloned().fold(f64::INFINITY, f64::min).min(0.0);
    let range = (max_v - min_v).max(1.0);
    let cw = w - pl - pr;
    let ch = h - pt - pb;
    let sx = if labels.len() > 1 { cw / (labels.len() - 1) as f64 } else { cw };
    if show_grid { draw_grid(&ctx, &labels, max_v, min_v, range, w, h, pl, pr, pt, pb, ch, sx); }
    for (_, data, color, _) in series {
        ctx.set_stroke_style_str(color);
        ctx.set_line_width(2.5);
        ctx.set_line_join("round");
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

// ─── BAR ─────────────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn draw_bar(canvas: &web_sys::HtmlCanvasElement, labels: &[String], series: &[(String, Vec<f64>, String, bool)], show_grid: bool, height: f64) {
    let Some(ctx) = get_context(canvas) else { return };
    let w = canvas.unchecked_ref::<web_sys::HtmlElement>().offset_width() as f64;
    let h = height;
    let (pl, pr, pt, pb) = (50.0, 20.0, 20.0, 40.0);
    ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    let all: Vec<f64> = series.iter().flat_map(|(_, d, _, _)| d.iter().copied()).collect();
    let max_v = all.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = max_v.max(1.0);
    let cw = w - pl - pr;
    let ch = h - pt - pb;
    let nl = labels.len();
    let ns = series.len();
    let gw = cw / nl as f64;
    let bw = (gw * 0.7) / ns as f64;
    let bg = gw * 0.15;
    if show_grid { draw_grid(&ctx, &labels, max_v, 0.0, range, w, h, pl, pr, pt, pb, ch, gw); }
    for (si, (_, data, color, _)) in series.iter().enumerate() {
        ctx.set_fill_style_str(color);
        for (i, &v) in data.iter().enumerate() {
            let bh = v / range * ch;
            let x = pl + i as f64 * gw + bg + si as f64 * bw;
            let y = pt + ch - bh;
            ctx.fill_rect(x, y, bw - 2.0, bh);
        }
    }
}

// ─── AREA ────────────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn draw_area(canvas: &web_sys::HtmlCanvasElement, labels: &[String], series: &[(String, Vec<f64>, String, bool)], show_grid: bool, height: f64) {
    let Some(ctx) = get_context(canvas) else { return };
    let w = canvas.unchecked_ref::<web_sys::HtmlElement>().offset_width() as f64;
    let h = height;
    let (pl, pr, pt, pb) = (50.0, 20.0, 20.0, 40.0);
    ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    let all: Vec<f64> = series.iter().flat_map(|(_, d, _, _)| d.iter().copied()).collect();
    let max_v = all.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_v = all.iter().cloned().fold(f64::INFINITY, f64::min).min(0.0);
    let range = (max_v - min_v).max(1.0);
    let cw = w - pl - pr;
    let ch = h - pt - pb;
    let sx = if labels.len() > 1 { cw / (labels.len() - 1) as f64 } else { cw };
    let by = pt + ch;
    if show_grid { draw_grid(&ctx, &labels, max_v, min_v, range, w, h, pl, pr, pt, pb, ch, sx); }
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
        ctx.set_fill_style_str(&format!("{}33", color));
        ctx.fill();
        ctx.set_stroke_style_str(color);
        ctx.set_line_width(2.5);
        ctx.begin_path();
        for (i, &v) in data.iter().enumerate() {
            let x = pl + i as f64 * sx;
            let y = pt + ch - ((v - min_v) / range * ch);
            if i == 0 { ctx.move_to(x, y); } else { ctx.line_to(x, y); }
        }
        ctx.stroke();
    }
}

// ─── DONUT ───────────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn draw_donut(canvas: &web_sys::HtmlCanvasElement, labels: &[String], series: &[(String, Vec<f64>, String, bool)], height: f64) {
    let Some(ctx) = get_context(canvas) else { return };
    let w = canvas.unchecked_ref::<web_sys::HtmlElement>().offset_width() as f64;
    let h = height;
    ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    let cx = w / 2.0; let cy = h / 2.0;
    let radius = (w.min(h) / 2.0 - 20.0).max(10.0);
    let inner  = radius * 0.6;
    let data = if let Some((_, d, _, _)) = series.first() { d } else { return };
    let colors = ["#6366f1","#f59e0b","#10b981","#ef4444","#8b5cf6","#06b6d4"];
    let total: f64 = data.iter().sum();
    if total == 0.0 { return; }
    let mut angle = -std::f64::consts::PI / 2.0;
    for (i, &v) in data.iter().enumerate() {
        let sweep = (v / total) * std::f64::consts::PI * 2.0;
        let color = series.get(i).map(|(_, _, c, _)| c.as_str()).unwrap_or(colors[i % colors.len()]);
        ctx.begin_path();
        ctx.move_to(cx, cy);
        ctx.arc(cx, cy, radius, angle, angle + sweep).ok();
        ctx.arc_with_anticlockwise(cx, cy, inner, angle + sweep, angle, true).ok();
        ctx.close_path();
        ctx.set_fill_style_str(color);
        ctx.fill();
        if sweep > 0.3 {
            let mid = angle + sweep / 2.0;
            let lr = (radius + inner) / 2.0;
            ctx.set_fill_style_str("#ffffff");
            ctx.set_font("bold 12px system-ui");
            ctx.set_text_align("center");
            let _ = ctx.fill_text(&format!("{:.0}%", (v / total * 100.0).round()), cx + mid.cos() * lr, cy + mid.sin() * lr + 4.0);
        }
        angle += sweep;
    }
    ctx.set_fill_style_str("#6b7280"); ctx.set_font("14px system-ui"); ctx.set_text_align("center");
    let _ = ctx.fill_text("Total", cx, cy - 5.0);
    ctx.set_font("bold 20px system-ui"); ctx.set_fill_style_str("#111827");
    let _ = ctx.fill_text(&format!("{:.0}", total), cx, cy + 18.0);
}

// ─── GRID HELPER ─────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn draw_grid(ctx: &web_sys::CanvasRenderingContext2d, labels: &[String], max_v: f64, min_v: f64, range: f64, w: f64, h: f64, pl: f64, pr: f64, pt: f64, pb: f64, ch: f64, sx: f64) {
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
fn format_axis_val(v: f64) -> String {
    if v.abs() >= 1_000_000.0 { format!("{:.1}M", v / 1_000_000.0) }
    else if v.abs() >= 1_000.0 { format!("{:.1}k", v / 1_000.0) }
    else { format!("{:.0}", v) }
}

// ─── JSON HELPERS ─────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn extract_json_string(json: &str, key: &str) -> Option<String> {
    let p = format!("\"{}\":", key);
    let s = json.find(&p)? + p.len();
    let r = json[s..].trim_start();
    if r.starts_with('"') { let e = r[1..].find('"')? + 1; Some(r[1..e].to_string()) } else { None }
}
#[cfg(feature = "hydrate")]
fn extract_json_key(json: &str, key: &str) -> Option<String> {
    let p = format!("\"{}\":", key);
    let s = json.find(&p)? + p.len();
    let r = json[s..].trim_start();
    let (open, close) = if r.starts_with('[') { ('[', ']') } else { ('{', '}') };
    let mut depth = 0; let mut end = 0;
    for (i, c) in r.chars().enumerate() {
        if c == open { depth += 1; } if c == close { depth -= 1; if depth == 0 { end = i + 1; break; } }
    }
    if end > 0 { Some(r[..end].to_string()) } else { None }
}
#[cfg(feature = "hydrate")]
fn extract_json_array(json: &str, key: &str) -> Option<Vec<String>> {
    let arr = extract_json_key(json, key)?;
    let inner = arr.trim_start_matches('[').trim_end_matches(']');
    Some(inner.split(',').map(|s| s.trim().trim_matches('"').to_string()).filter(|s| !s.is_empty()).collect())
}
#[cfg(feature = "hydrate")]
fn extract_json_number_array(json: &str, key: &str) -> Option<Vec<f64>> {
    let arr = extract_json_key(json, key)?;
    let inner = arr.trim_start_matches('[').trim_end_matches(']');
    Some(inner.split(',').filter_map(|s| s.trim().parse::<f64>().ok()).collect())
}
#[cfg(feature = "hydrate")]
fn split_json_objects(json: &str) -> Vec<String> {
    let inner = json.trim_start_matches('[').trim_end_matches(']');
    let mut out = Vec::new(); let mut depth = 0; let mut start = 0;
    for (i, c) in inner.chars().enumerate() {
        match c { '{' => { if depth == 0 { start = i; } depth += 1; } '}' => { depth -= 1; if depth == 0 { out.push(inner[start..=i].to_string()); } } _ => {} }
    }
    out
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
