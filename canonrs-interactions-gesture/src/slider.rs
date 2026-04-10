//! Slider Interaction Engine

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state, drag};

fn get_min(el: &Element) -> f64 { el.get_attribute("data-rs-min").and_then(|s| s.parse().ok()).unwrap_or(0.0) }
fn get_max(el: &Element) -> f64 { el.get_attribute("data-rs-max").and_then(|s| s.parse().ok()).unwrap_or(100.0) }
fn get_step(el: &Element) -> f64 { el.get_attribute("data-rs-step").and_then(|s| s.parse().ok()).unwrap_or(1.0) }

fn set_value(el: &Element, value: f64) {
    let min = get_min(el); let max = get_max(el); let step = get_step(el);
    let snapped = if step > 0.0 { ((value/step).round()*step).clamp(min,max) } else { value.clamp(min,max) };
    let pct = (snapped-min)/(max-min)*100.0;
    let _ = el.set_attribute("data-rs-value", &format!("{:.2}", snapped));
    let _ = el.set_attribute("data-rs-percent", &format!("{:.4}", pct));
    let _ = el.set_attribute("aria-valuenow", &format!("{:.2}", snapped));
    if let Ok(h) = el.clone().dyn_into::<web_sys::HtmlElement>() {
        let _ = h.style().set_property("--slider-fill", &format!("{:.4}%", pct));
    }
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    if root.has_attribute("data-rs-disabled") && root.get_attribute("aria-disabled").as_deref() == Some("true") { return; }

    let ptr = drag::new();
    let doc = web_sys::window().and_then(|w| w.document()).unwrap();

    {
        let r = root.clone(); let p = ptr.clone();
        let cb = Closure::<dyn Fn(web_sys::PointerEvent)>::new(move |e: web_sys::PointerEvent| {
            if !state::is_valid(&r) { return; }
            e.prevent_default(); e.stop_propagation();
            drag::start(&p, e.pointer_id());
            if let Ok(h) = r.clone().dyn_into::<web_sys::HtmlElement>() {
                h.set_pointer_capture(e.pointer_id()).ok();
                let rect = h.get_bounding_client_rect();
                let x = (e.client_x() as f64 - rect.left()).max(0.0).min(rect.width());
                set_value(&r, get_min(&r) + (x/rect.width())*(get_max(&r)-get_min(&r)));
            }
            state::add(&r, "active");
        });
        let _ = root.add_event_listener_with_callback("pointerdown", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    {
        let r = root.clone(); let p = ptr.clone();
        let cb = Closure::<dyn Fn(web_sys::PointerEvent)>::new(move |e: web_sys::PointerEvent| {
            if !drag::is_active(&p, e.pointer_id()) { return; }
            if !state::is_valid(&r) { return; }
            if let Ok(h) = r.clone().dyn_into::<web_sys::HtmlElement>() {
                let rect = h.get_bounding_client_rect();
                let x = (e.client_x() as f64 - rect.left()).max(0.0).min(rect.width());
                set_value(&r, get_min(&r) + (x/rect.width())*(get_max(&r)-get_min(&r)));
            }
        });
        let _ = doc.add_event_listener_with_callback("pointermove", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }

    {
        let r = root.clone(); let p = ptr.clone();
        let cb = Closure::<dyn Fn(web_sys::PointerEvent)>::new(move |e: web_sys::PointerEvent| {
            if !drag::is_active(&p, e.pointer_id()) { return; }
            drag::stop(&p);
            state::remove(&r, "active");
        });
        let _ = doc.add_event_listener_with_callback("pointerup", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }

    { let r = root.clone(); let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::new(move |_| { state::add(&r, "focus"); }); let _ = root.add_event_listener_with_callback("focus", cb.as_ref().unchecked_ref()); cb.forget(); }
    { let r = root.clone(); let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::new(move |_| { state::remove(&r, "focus"); }); let _ = root.add_event_listener_with_callback("blur", cb.as_ref().unchecked_ref()); cb.forget(); }

    {
        let r = root.clone();
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
            if !state::is_valid(&r) { return; }
            let min = get_min(&r); let max = get_max(&r); let step = get_step(&r);
            let cur = r.get_attribute("data-rs-value").and_then(|s| s.parse::<f64>().ok()).unwrap_or(min);
            let inc = if step > 0.0 { step } else { 1.0 };
            let new_val = match e.key().as_str() {
                "ArrowRight"|"ArrowUp"   => { e.prevent_default(); cur+inc }
                "ArrowLeft" |"ArrowDown" => { e.prevent_default(); cur-inc }
                "Home" => { e.prevent_default(); min }
                "End"  => { e.prevent_default(); max }
                _ => return,
            };
            set_value(&r, new_val);
        });
        let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}
