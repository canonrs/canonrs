//! Chart Resize Observer

#[cfg(feature = "hydrate")]
use super::draw::{set_canvas_dpi, draw_chart};
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;

#[cfg(feature = "hydrate")]
pub(crate) fn setup_resize_observer(
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

    let observer_cb = Closure::wrap(Box::new(move |_entries: js_sys::Array| {
        closure.as_ref().unchecked_ref::<js_sys::Function>().call0(&wasm_bindgen::JsValue::NULL).ok();
    }) as Box<dyn FnMut(js_sys::Array)>);

    if let Ok(observer_ctor) = js_sys::Reflect::get(&web_sys::window().unwrap(), &wasm_bindgen::JsValue::from_str("ResizeObserver")) {
        if let Ok(observer) = js_sys::Reflect::construct(
            &observer_ctor.unchecked_into::<js_sys::Function>(),
            &js_sys::Array::of1(observer_cb.as_ref())
        ) {
            let observe_fn = js_sys::Reflect::get(&observer, &wasm_bindgen::JsValue::from_str("observe")).unwrap();
            js_sys::Reflect::apply(
                &observe_fn.unchecked_into::<js_sys::Function>(),
                &observer,
                &js_sys::Array::of1(root)
            ).ok();
        }
    }
    observer_cb.forget();
}
