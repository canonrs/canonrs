//! Slider Interaction Engine

use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, PointerEvent};
use canonrs_interactions_core::dom::{state, attrs};
use canonrs_interactions_core::runtime::listeners;
use crate::runtime::drag;

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
    let init = web_sys::CustomEventInit::new();
    init.set_bubbles(true);
    if let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict("rs-change", &init) {
        let _ = el.dispatch_event(&event);
    }
}

pub fn init(root: Element) {
    if root.has_attribute("data-rs-disabled") && root.get_attribute("aria-disabled").as_deref() == Some("true") { return; }

    let initial = attrs::get_f64(&root, "data-rs-value", attrs::get_f64(&root, "data-rs-min", 0.0));
    set_value(&root, initial);

    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

    // pointerdown — inicia drag + captura pointer no root
    listeners::listen(&uid, &root, "pointerdown", move |e: web_sys::Event| {
        let e = e.dyn_into::<PointerEvent>().unwrap();
        let Some(cur) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        e.prevent_default(); e.stop_propagation();
        drag::set_drag(&cur, e.pointer_id(), 0.0, 0.0);
        if let Ok(h) = cur.clone().dyn_into::<HtmlElement>() { h.set_pointer_capture(e.pointer_id()).ok(); }
        if let Some(pct) = drag::calc_pct_horizontal(&cur, e.client_x() as f64) {
            set_value(&cur, drag::calc_value_from_pct(pct, attrs::get_f64(&cur, "data-rs-min", 0.0), attrs::get_f64(&cur, "data-rs-max", 100.0)));
        }
        state::add(&cur, canonrs_interactions_core::dom::state::State::Active.as_str());
    });

    // pointermove — pointer capture garante que chega aqui sem query global
    listeners::listen(&uid, &root, "pointermove", move |e: web_sys::Event| {
        let e = e.dyn_into::<PointerEvent>().unwrap();
        let Some(cur) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        if !drag::drag_active(&cur, e.pointer_id()) { return; }
        if let Some(pct) = drag::calc_pct_horizontal(&cur, e.client_x() as f64) {
            set_value(&cur, drag::calc_value_from_pct(pct, attrs::get_f64(&cur, "data-rs-min", 0.0), attrs::get_f64(&cur, "data-rs-max", 100.0)));
        }
    });

    // pointerup — encerra drag
    listeners::listen(&uid, &root, "pointerup", move |e: web_sys::Event| {
        let e = e.dyn_into::<PointerEvent>().unwrap();
        let Some(cur) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        if !drag::drag_active(&cur, e.pointer_id()) { return; }
        drag::clear_drag(&cur);
        state::remove(&cur, canonrs_interactions_core::dom::state::State::Active.as_str());
        if let Ok(h) = cur.clone().dyn_into::<HtmlElement>() { let _ = h.release_pointer_capture(e.pointer_id()); }
    });

    // focus / blur
    listeners::listen(&uid, &root, "focus", move |e: web_sys::Event| {
        let Some(cur) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        state::add(&cur, "focus");
    });

    listeners::listen(&uid, &root, "blur", move |e: web_sys::Event| {
        let Some(cur) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        state::remove(&cur, "focus");
    });

    // keydown
    listeners::listen(&uid, &root, "keydown", move |e: web_sys::Event| {
        let e = e.dyn_into::<web_sys::KeyboardEvent>().unwrap();
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
}
