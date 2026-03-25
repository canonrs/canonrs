//! Chart Utils - helpers compartilhados

#[cfg(feature = "hydrate")]
// use wasm_bindgen::prelude::*; // removed: unused
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;

#[cfg(feature = "hydrate")]
pub fn get_context(canvas: &web_sys::HtmlCanvasElement) -> Option<web_sys::CanvasRenderingContext2d> {
    canvas.get_context("2d").ok()?.and_then(|c| c.dyn_into().ok())
}

#[cfg(feature = "hydrate")]
pub fn format_axis_val(v: f64) -> String {
    if v.abs() >= 1_000_000.0 { format!("{:.1}M", v / 1_000_000.0) }
    else if v.abs() >= 1_000.0 { format!("{:.1}k", v / 1_000.0) }
    else { format!("{:.0}", v) }
}

#[cfg(feature = "hydrate")]
pub fn dispatch_custom_event(target: &web_sys::Element, name: &str, detail: &js_sys::Object) {
    use wasm_bindgen::JsValue;
    let init = web_sys::CustomEventInit::new();
    init.set_bubbles(true);
    init.set_detail(&JsValue::from(detail));
    if let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict(name, &init) {
        target.dispatch_event(&event).ok();
    }
}
