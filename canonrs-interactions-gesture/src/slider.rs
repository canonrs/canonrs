//! Slider Interaction Engine

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, PointerEvent};
use crate::runtime::{lifecycle, state, drag, attrs};

fn set_value(el: &Element, value: f64) {
    let min  = attrs::get_f64(el, "data-rs-min",  0.0);
    let max  = attrs::get_f64(el, "data-rs-max",  100.0);
    let step = attrs::get_f64(el, "data-rs-step", 1.0);
    let snapped = if step > 0.0 { ((value/step).round()*step).clamp(min,max) } else { value.clamp(min,max) };
    let pct = (snapped-min)/(max-min)*100.0;
    let _ = el.set_attribute("data-rs-value",   &format!("{:.2}", snapped));
    let _ = el.set_attribute("data-rs-percent", &format!("{:.4}", pct));
    let _ = el.set_attribute("aria-valuenow",   &format!("{:.2}", snapped));
    if let Ok(h) = el.clone().dyn_into::<HtmlElement>() {
        let _ = h.style().set_property("--slider-fill", &format!("{:.4}%", pct));
    }
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    if root.has_attribute("data-rs-disabled") && root.get_attribute("aria-disabled").as_deref() == Some("true") { return; }

    let doc = web_sys::window().and_then(|w| w.document()).unwrap();

    // pointerdown — inicia drag no root
    {
        let cb = Closure::<dyn Fn(PointerEvent)>::new(move |e: PointerEvent| {
            let Some(cur) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            e.prevent_default(); e.stop_propagation();
            drag::set_drag(&cur, e.pointer_id(), 0.0, 0.0); // size/offset não usados no slider
            if let Ok(h) = cur.clone().dyn_into::<HtmlElement>() { h.set_pointer_capture(e.pointer_id()).ok(); }
            if let Some(pct) = drag::calc_pct_horizontal(&cur, e.client_x() as f64) {
                set_value(&cur, drag::calc_value_from_pct(pct, attrs::get_f64(&cur, "data-rs-min", 0.0), attrs::get_f64(&cur, "data-rs-max", 100.0)));
            }
            state::add(&cur, "active");
        });
        let _ = root.add_event_listener_with_callback("pointerdown", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // pointermove — atualiza valor
    {
        let cb = Closure::<dyn Fn(PointerEvent)>::new(move |e: PointerEvent| {
            let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
            let Ok(Some(root_el)) = doc.query_selector("[data-rs-slider][data-rs-state~='active']") else { return };
            if !drag::drag_active(&root_el, e.pointer_id()) { return; }
            if let Some(pct) = drag::calc_pct_horizontal(&root_el, e.client_x() as f64) {
                set_value(&root_el, drag::calc_value_from_pct(pct, attrs::get_f64(&root_el, "data-rs-min", 0.0), attrs::get_f64(&root_el, "data-rs-max", 100.0)));
            }
        });
        let _ = doc.add_event_listener_with_callback("pointermove", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }

    // pointerup — encerra drag
    {
        let cb = Closure::<dyn Fn(PointerEvent)>::new(move |e: PointerEvent| {
            let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
            let Ok(Some(root_el)) = doc.query_selector("[data-rs-slider][data-rs-state~='active']") else { return };
            if !drag::drag_active(&root_el, e.pointer_id()) { return; }
            drag::clear_drag(&root_el);
            state::remove(&root_el, "active");
            if let Ok(h) = root_el.dyn_into::<HtmlElement>() { let _ = h.release_pointer_capture(e.pointer_id()); }
        });
        let _ = doc.add_event_listener_with_callback("pointerup", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }

    // focus / blur
    { let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::new(move |e: web_sys::FocusEvent| {
        let Some(cur) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        state::add(&cur, "focus");
    }); let _ = root.add_event_listener_with_callback("focus", cb.as_ref().unchecked_ref()); cb.forget(); }

    { let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::new(move |e: web_sys::FocusEvent| {
        let Some(cur) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        state::remove(&cur, "focus");
    }); let _ = root.add_event_listener_with_callback("blur", cb.as_ref().unchecked_ref()); cb.forget(); }

    // keydown
    {
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
            let Some(cur) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let min  = attrs::get_f64(&cur, "data-rs-min",  0.0);
            let max  = attrs::get_f64(&cur, "data-rs-max",  100.0);
            let step = attrs::get_f64(&cur, "data-rs-step", 1.0);
            let cur_val = cur.get_attribute("data-rs-value").and_then(|s| s.parse::<f64>().ok()).unwrap_or(min);
            let inc = if step > 0.0 { step } else { 1.0 };
            let new_val = match e.key().as_str() {
                "ArrowRight"|"ArrowUp"   => { e.prevent_default(); cur_val+inc }
                "ArrowLeft" |"ArrowDown" => { e.prevent_default(); cur_val-inc }
                "Home" => { e.prevent_default(); min }
                "End"  => { e.prevent_default(); max }
                _ => return,
            };
            set_value(&cur, new_val);
        });
        let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}
