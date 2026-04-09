//! Slider Interaction Engine — pointer drag + keyboard

use wasm_bindgen::prelude::*;
use crate::shared::{add_state, remove_state, is_initialized, mark_initialized};
use wasm_bindgen::JsCast;
use web_sys::Element;
use std::cell::Cell;
use std::rc::Rc;

fn is_element_alive(el: &web_sys::Element) -> bool {
    use wasm_bindgen::JsValue;
    let val: &JsValue = el.as_ref();
    !val.is_undefined() && !val.is_null()
}

fn get_min(el: &Element) -> f64 { el.get_attribute("data-rs-min").and_then(|s| s.parse().ok()).unwrap_or(0.0) }
fn get_max(el: &Element) -> f64 { el.get_attribute("data-rs-max").and_then(|s| s.parse().ok()).unwrap_or(100.0) }
fn get_step(el: &Element) -> f64 { el.get_attribute("data-rs-step").and_then(|s| s.parse().ok()).unwrap_or(1.0) }

fn set_value(el: &Element, value: f64) {
    let min = get_min(el); let max = get_max(el); let step = get_step(el);
    let snapped = if step > 0.0 { ((value / step).round() * step).clamp(min, max) } else { value.clamp(min, max) };
    let pct = (snapped - min) / (max - min) * 100.0;
    let _ = el.set_attribute("data-rs-value", &format!("{:.2}", snapped));
    let _ = el.set_attribute("data-rs-percent", &format!("{:.4}", pct));
    let _ = el.set_attribute("aria-valuenow", &format!("{:.2}", snapped));
    if let Ok(el_html) = el.clone().dyn_into::<web_sys::HtmlElement>() {
        let _ = el_html.style().set_property("--slider-fill", &format!("{:.4}%", pct));
    }
}

pub fn init(root: Element) {
    if root.has_attribute("data-rs-disabled") && root.get_attribute("aria-disabled").as_deref() == Some("true") { return; }

    let dragging:   Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let active_ptr: Rc<Cell<i32>>  = Rc::new(Cell::new(-1));

    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };

    // pointerdown no root
    {
        let root_cb = root.clone();
        let drag_d = dragging.clone(); let ptr_d = active_ptr.clone();
        let cb = Closure::<dyn Fn(web_sys::PointerEvent)>::new(move |e: web_sys::PointerEvent| {
            if !is_element_alive(&root_cb) { return; }
            e.prevent_default();
            e.stop_propagation();
            web_sys::console::log_1(&format!("[slider] pointerdown ptr={}", e.pointer_id()).into());
            drag_d.set(true);
            ptr_d.set(e.pointer_id());
            if let Ok(el) = root_cb.clone().dyn_into::<web_sys::HtmlElement>() {
                el.set_pointer_capture(e.pointer_id()).ok();
                let rect = el.get_bounding_client_rect();
                let x = (e.client_x() as f64 - rect.left()).max(0.0).min(rect.width());
                set_value(&root_cb, get_min(&root_cb) + (x / rect.width()) * (get_max(&root_cb) - get_min(&root_cb)));
            }
            add_state(&root_cb, "active");
        });
        let _ = root.add_event_listener_with_callback("pointerdown", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // pointermove no document
    {
        let root_cb = root.clone();
        let drag_m = dragging.clone(); let ptr_m = active_ptr.clone();
        let cb = Closure::<dyn Fn(web_sys::PointerEvent)>::new(move |e: web_sys::PointerEvent| {
            web_sys::console::log_1(&format!("[slider] pointermove drag={} ptr={} evt={}", drag_m.get(), ptr_m.get(), e.pointer_id()).into());
            if !drag_m.get() || ptr_m.get() != e.pointer_id() { return; }
            if !is_element_alive(&root_cb) { return; }
            if let Ok(el) = root_cb.clone().dyn_into::<web_sys::HtmlElement>() {
                let rect = el.get_bounding_client_rect();
                let x = (e.client_x() as f64 - rect.left()).max(0.0).min(rect.width());
                set_value(&root_cb, get_min(&root_cb) + (x / rect.width()) * (get_max(&root_cb) - get_min(&root_cb)));
            }
        });
        let doc_target: &web_sys::EventTarget = doc.as_ref();
        doc_target.add_event_listener_with_callback("pointermove", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }

    // pointerup no document
    {
        let root_cb = root.clone();
        let drag_u = dragging.clone(); let ptr_u = active_ptr.clone();
        let cb = Closure::<dyn Fn(web_sys::PointerEvent)>::new(move |e: web_sys::PointerEvent| {
            if ptr_u.get() != e.pointer_id() { return; }
            if !is_element_alive(&root_cb) { return; }
            web_sys::console::log_1(&"[slider] pointerup".into());
            drag_u.set(false);
            ptr_u.set(-1);
            remove_state(&root_cb, "active");
        });
        let doc_target: &web_sys::EventTarget = doc.as_ref();
        doc_target.add_event_listener_with_callback("pointerup", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }

    // focus/blur
    { let rc = root.clone(); let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::new(move |_| { if !is_element_alive(&rc) { return; } add_state(&rc, "focus"); }); let _ = root.add_event_listener_with_callback("focus", cb.as_ref().unchecked_ref()); cb.forget(); }
    { let rc = root.clone(); let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::new(move |_| { if !is_element_alive(&rc) { return; } remove_state(&rc, "focus"); }); let _ = root.add_event_listener_with_callback("blur", cb.as_ref().unchecked_ref()); cb.forget(); }

    // keydown
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
            if !is_element_alive(&root_cb) { return; }
            let min = get_min(&root_cb); let max = get_max(&root_cb); let step = get_step(&root_cb);
            let cur = root_cb.get_attribute("data-rs-value").and_then(|s| s.parse::<f64>().ok()).unwrap_or(min);
            let inc = if step > 0.0 { step } else { 1.0 };
            let new_val = match e.key().as_str() {
                "ArrowRight" | "ArrowUp"   => { e.prevent_default(); cur + inc }
                "ArrowLeft"  | "ArrowDown" => { e.prevent_default(); cur - inc }
                "Home" => { e.prevent_default(); min }
                "End"  => { e.prevent_default(); max }
                _ => return,
            };
            set_value(&root_cb, new_val);
        });
        let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        cb.forget();
    }
    web_sys::console::log_1(&"[slider] init complete".into());
}
