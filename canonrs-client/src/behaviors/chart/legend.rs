//! Chart Legend

#[cfg(feature = "hydrate")]
use super::data::{read_chart_data, parse_chart_data};
#[cfg(feature = "hydrate")]
use super::draw::draw_chart;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;

#[cfg(feature = "hydrate")]
pub fn draw_legend(
    legend_el: &web_sys::Element,
    series: &[(String, Vec<f64>, String, bool)],
    root: &web_sys::Element,
    canvas: &web_sys::HtmlCanvasElement,
    chart_type: &str,
    _labels: &[String],
    show_grid: bool,
    height: f64,
) {
    legend_el.set_inner_html("");
    let doc = web_sys::window().unwrap().document().unwrap();
    for (i, (name, _, color, active)) in series.iter().enumerate() {
        let item = doc.create_element("span").unwrap();
        item.set_attribute("data-rs-chart-legend-item", "").ok();
        item.set_attribute("data-series-index", &i.to_string()).ok();
        // data-rs-state em vez de data-state
        item.set_attribute("data-rs-state", if *active { "active" } else { "inactive" }).ok();
        let dot = doc.create_element("span").unwrap();
        dot.set_attribute("data-rs-chart-legend-dot", "").ok();
        let _ = dot.unchecked_ref::<web_sys::HtmlElement>().style().set_property("background", color);
        let label = doc.create_element("span").unwrap();
        label.set_text_content(Some(name));
        item.append_child(&dot).ok();
        item.append_child(&label).ok();
        legend_el.append_child(&item).ok();

        let canvas_c     = canvas.clone();
        let root_c       = root.clone();
        let chart_type_c = chart_type.to_string();
        let item_c       = item.clone();
        let legend_c     = legend_el.clone();

        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let current = item_c.get_attribute("data-rs-state").unwrap_or_default();
            let new_state = if current == "active" { "inactive" } else { "active" };
            item_c.set_attribute("data-rs-state", new_state).ok();
            let data_json = read_chart_data(&root_c);
            if let Some((lbl, mut ser)) = parse_chart_data(&data_json) {
                let items = legend_c.query_selector_all("[data-rs-chart-legend-item]").unwrap();
                for j in 0..items.length() {
                    if let Some(el) = items.item(j).and_then(|e| e.dyn_into::<web_sys::Element>().ok()) {
                        let si = el.get_attribute("data-series-index")
                            .and_then(|v| v.parse::<usize>().ok()).unwrap_or(0);
                        let active = el.get_attribute("data-rs-state").as_deref() == Some("active");
                        if let Some(s) = ser.get_mut(si) { s.3 = active; }
                    }
                }
                draw_chart(&canvas_c, &chart_type_c, &lbl, &ser, show_grid, height);
            }
        }) as Box<dyn FnMut(_)>);
        item.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
        closure.forget();
    }
}
