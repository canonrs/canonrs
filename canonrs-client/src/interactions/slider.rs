//! Slider Interaction Engine — pointer drag + keyboard

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use std::cell::RefCell;
use std::rc::Rc;

fn add_state(el: &Element, state: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    if !current.split_whitespace().any(|s| s == state) {
        let next = if current.is_empty() { state.to_string() } else { format!("{} {}", current, state) };
        el.set_attribute("data-rs-state", &next).ok();
    }
}

fn remove_state(el: &Element, state: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    let next: Vec<&str> = current.split_whitespace().filter(|s| *s != state).collect();
    el.set_attribute("data-rs-state", &next.join(" ")).ok();
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
    let is_disabled = root.get_attribute("data-rs-state").map(|s| s.contains("disabled")).unwrap_or(false);
    if is_disabled { return; }

    let active_pointer: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(None));
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };

    // pointerdown on root
    {
        let root_cb = root.clone();
        let ap = active_pointer.clone();
        let cb = Closure::<dyn Fn(web_sys::PointerEvent)>::wrap(Box::new(move |e: web_sys::PointerEvent| {
            e.prevent_default();
            *ap.borrow_mut() = Some(e.pointer_id());
            add_state(&root_cb, "active");
            if let Ok(el) = root_cb.clone().dyn_into::<web_sys::HtmlElement>() {
                el.set_pointer_capture(e.pointer_id()).ok();
                let rect = el.get_bounding_client_rect();
                let x = (e.client_x() as f64 - rect.left()).max(0.0).min(rect.width());
                let pct = x / rect.width();
                let min = get_min(&root_cb); let max = get_max(&root_cb);
                set_value(&root_cb, min + pct * (max - min));
            }
        }));
        let _ = root.add_event_listener_with_callback("pointerdown", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // pointermove on document
    {
        let root_cb = root.clone();
        let ap = active_pointer.clone();
        let cb = Closure::<dyn Fn(web_sys::PointerEvent)>::wrap(Box::new(move |e: web_sys::PointerEvent| {
            if *ap.borrow() != Some(e.pointer_id()) { return; }
            if let Ok(el) = root_cb.clone().dyn_into::<web_sys::HtmlElement>() {
                let rect = el.get_bounding_client_rect();
                let x = (e.client_x() as f64 - rect.left()).max(0.0).min(rect.width());
                let pct = x / rect.width();
                let min = get_min(&root_cb); let max = get_max(&root_cb);
                set_value(&root_cb, min + pct * (max - min));
            }
        }));
        let doc_el: &web_sys::EventTarget = doc.as_ref();
        let _ = doc_el.add_event_listener_with_callback("pointermove", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // pointerup on document
    {
        let root_cb = root.clone();
        let ap = active_pointer.clone();
        let cb = Closure::<dyn Fn(web_sys::PointerEvent)>::wrap(Box::new(move |e: web_sys::PointerEvent| {
            if *ap.borrow() == Some(e.pointer_id()) {
                *ap.borrow_mut() = None;
                remove_state(&root_cb, "active");
            }
        }));
        let doc_el: &web_sys::EventTarget = doc.as_ref();
        let _ = doc_el.add_event_listener_with_callback("pointerup", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // focus/blur
    { let rc = root.clone(); let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::wrap(Box::new(move |_| { add_state(&rc, "focus"); })); let _ = root.add_event_listener_with_callback("focus", cb.as_ref().unchecked_ref()); cb.forget(); }
    { let rc = root.clone(); let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::wrap(Box::new(move |_| { remove_state(&rc, "focus"); })); let _ = root.add_event_listener_with_callback("blur", cb.as_ref().unchecked_ref()); cb.forget(); }

    // keydown
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            let min = get_min(&root_cb); let max = get_max(&root_cb); let step = get_step(&root_cb);
            let cur = root_cb.get_attribute("data-rs-value").and_then(|s| s.parse::<f64>().ok()).unwrap_or(min);
            let inc = if step > 0.0 { step } else { 1.0 };
            let new_val = match e.key().as_str() {
                "ArrowRight" | "ArrowUp" => { e.prevent_default(); cur + inc }
                "ArrowLeft" | "ArrowDown" => { e.prevent_default(); cur - inc }
                "Home" => { e.prevent_default(); min }
                "End" => { e.prevent_default(); max }
                _ => return,
            };
            set_value(&root_cb, new_val);
        }));
        let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-slider]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<web_sys::Element>() { init(el); }
        }
    }
}