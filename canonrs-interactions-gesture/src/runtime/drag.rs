//! Drag — state management via DOM attributes (GC-safe, no Rc/Cell)

use wasm_bindgen::JsCast;
use web_sys::Element;

// --- DOM-based drag state ---

pub fn set_drag(root: &Element, ptr_id: i32, size: f64, offset: f64) {
    let _ = root.set_attribute("data-rs-drag-ptr",    &ptr_id.to_string());
    let _ = root.set_attribute("data-rs-drag-size",   &size.to_string());
    let _ = root.set_attribute("data-rs-drag-offset", &offset.to_string());
}

pub fn clear_drag(root: &Element) {
    let _ = root.remove_attribute("data-rs-drag-ptr");
    let _ = root.remove_attribute("data-rs-drag-size");
    let _ = root.remove_attribute("data-rs-drag-offset");
}

pub fn drag_active(root: &Element, ptr_id: i32) -> bool {
    root.get_attribute("data-rs-drag-ptr")
        .and_then(|s| s.parse::<i32>().ok())
        .map(|p| p == ptr_id)
        .unwrap_or(false)
}

pub fn drag_size(root: &Element) -> f64 {
    root.get_attribute("data-rs-drag-size").and_then(|s| s.parse().ok()).unwrap_or(0.0)
}

pub fn drag_offset(root: &Element) -> f64 {
    root.get_attribute("data-rs-drag-offset").and_then(|s| s.parse().ok()).unwrap_or(0.0)
}

// --- Position helpers ---

pub fn calc_pct_horizontal(el: &web_sys::Element, client_x: f64) -> Option<f64> {
    let h = el.clone().dyn_into::<web_sys::HtmlElement>().ok()?;
    let rect = h.get_bounding_client_rect();
    if rect.width() == 0.0 { return None; }
    let x = (client_x - rect.left()).max(0.0).min(rect.width());
    Some(x / rect.width() * 100.0)
}

pub fn calc_pct_vertical(el: &web_sys::Element, client_y: f64) -> Option<f64> {
    let h = el.clone().dyn_into::<web_sys::HtmlElement>().ok()?;
    let rect = h.get_bounding_client_rect();
    if rect.height() == 0.0 { return None; }
    let y = (client_y - rect.top()).max(0.0).min(rect.height());
    Some(y / rect.height() * 100.0)
}

pub fn calc_value_from_pct(pct: f64, min: f64, max: f64) -> f64 {
    min + (pct / 100.0) * (max - min)
}
