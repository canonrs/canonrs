#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::HtmlElement;
#[cfg(feature = "hydrate")]
use std::rc::Rc;
#[cfg(feature = "hydrate")]
use std::cell::{Cell, RefCell};

#[cfg(feature = "hydrate")]
fn add_state(el: &web_sys::Element, state: &str) {
    let mut s = el.get_attribute("data-rs-state").unwrap_or_default();
    if !s.split_whitespace().any(|x| x == state) {
        s = format!("{} {}", s, state).trim().to_string();
    }
    el.set_attribute("data-rs-state", &s).ok();
}

#[cfg(feature = "hydrate")]
fn remove_state(el: &web_sys::Element, state: &str) {
    if let Some(s) = el.get_attribute("data-rs-state") {
        let f = s.split_whitespace().filter(|x| *x != state).collect::<Vec<_>>().join(" ");
        if f.is_empty() { el.remove_attribute("data-rs-state").ok(); }
        else { el.set_attribute("data-rs-state", &f).ok(); }
    }
}

#[cfg(feature = "hydrate")]
fn update_slider_ui(slider_el: &web_sys::Element, percent: f64) {
    slider_el.set_attribute("data-rs-percent", &format!("{:.4}", percent)).ok();
    if let Ok(html) = slider_el.clone().dyn_into::<web_sys::HtmlElement>() {
        html.style().set_property("--slider-fill", &format!("{:.4}%", percent)).ok();
    }
}

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-slider", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {
        
        let slider_el = root;
        if slider_el.get_attribute("data-rs-slider-attached").as_deref() == Some("1") { return Ok(()); }
        slider_el.set_attribute("data-rs-slider-attached", "1").ok();

        // disabled state
        if slider_el.has_attribute("data-rs-disabled") {
            add_state(slider_el, "disabled");
            return Ok(());
        }

        let min: f64 = slider_el.get_attribute("aria-valuemin").and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let max: f64 = slider_el.get_attribute("aria-valuemax").and_then(|s| s.parse().ok()).unwrap_or(100.0);
        let val: f64 = slider_el.get_attribute("aria-valuenow").and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let step: f64 = slider_el.get_attribute("data-rs-step").and_then(|s| s.parse().ok()).unwrap_or(0.0);

        let percent = ((val - min) / (max - min)) * 100.0;
        update_slider_ui(&slider_el, percent);

        let is_dragging = Rc::new(Cell::new(false));
        let active_pointer: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(None));

        let is_dragging_down = is_dragging.clone();
        let is_dragging_move = is_dragging.clone();
        let is_dragging_up = is_dragging.clone();
        let active_down = active_pointer.clone();
        let active_move = active_pointer.clone();
        let active_up = active_pointer.clone();

        let Ok(slider_html) = slider_el.clone().dyn_into::<HtmlElement>() else { return Ok(()); };

        let slider_down = slider_html.clone();
        let on_down = Closure::wrap(Box::new(move |e: web_sys::PointerEvent| {
            is_dragging_down.set(true);
            *active_down.borrow_mut() = Some(e.pointer_id());
            slider_down.set_pointer_capture(e.pointer_id()).ok();
            add_state(&slider_down.clone().dyn_into::<web_sys::Element>().unwrap(), "active");
        }) as Box<dyn FnMut(_)>);

        let slider_move = slider_html.clone();
        let slider_el_move = slider_el.clone();
        let on_move = Closure::wrap(Box::new(move |e: web_sys::PointerEvent| {
            if !is_dragging_move.get() { return; }
            if *active_move.borrow() != Some(e.pointer_id()) { return; }
            let rect = slider_move.get_bounding_client_rect();
            let x = (e.client_x() as f64 - rect.left()).max(0.0).min(rect.width());
            let pct = (x / rect.width()) * 100.0;
            let raw = min + (pct / 100.0) * (max - min);
            let value = if step > 0.0 {
                let snapped = (raw / step).round() * step;
                snapped.clamp(min, max)
            } else {
                raw.clamp(min, max)
            };
            let pct = ((value - min) / (max - min)) * 100.0;
            slider_el_move.set_attribute("data-rs-value", &format!("{:.2}", value)).ok();
            update_slider_ui(&slider_el_move, pct);

            let init = web_sys::CustomEventInit::new();
            init.set_detail(&wasm_bindgen::JsValue::from_f64(value));
            init.set_bubbles(true);
            init.set_cancelable(false);
            if let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict("rs-change", &init) {
                slider_el_move.dispatch_event(&event).ok();
            }
        }) as Box<dyn FnMut(_)>);

        let slider_el_up = slider_el.clone();
        let on_up = Closure::wrap(Box::new(move |e: web_sys::PointerEvent| {
            if *active_up.borrow() == Some(e.pointer_id()) {
                is_dragging_up.set(false);
                *active_up.borrow_mut() = None;
                remove_state(&slider_el_up, "active");
                let val: f64 = slider_el_up.get_attribute("data-rs-value")
                    .and_then(|s| s.parse().ok()).unwrap_or(0.0);
                let init = web_sys::CustomEventInit::new();
                init.set_detail(&wasm_bindgen::JsValue::from_f64(val));
                init.set_bubbles(true);
                if let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict("rs-change", &init) {
                    slider_el_up.dispatch_event(&event).ok();
                }
            }
        }) as Box<dyn FnMut(_)>);

        slider_html.add_event_listener_with_callback("pointerdown", on_down.as_ref().unchecked_ref()).ok();
        slider_html.add_event_listener_with_callback("pointermove", on_move.as_ref().unchecked_ref()).ok();
        slider_html.add_event_listener_with_callback("pointerup", on_up.as_ref().unchecked_ref()).ok();
        on_down.forget();
        on_move.forget();
        on_up.forget();

        // focus state on thumb
        if let Ok(Some(thumb)) = slider_el.query_selector("[data-rs-slider-thumb]") {
            let thumb_in = thumb.clone();
            let thumb_out = thumb.clone();
            let on_focus = Closure::wrap(Box::new(move |_: web_sys::FocusEvent| {
                add_state(&thumb_in, "focus");
            }) as Box<dyn FnMut(_)>);
            let on_blur = Closure::wrap(Box::new(move |_: web_sys::FocusEvent| {
                remove_state(&thumb_out, "focus");
            }) as Box<dyn FnMut(_)>);
            thumb.add_event_listener_with_callback("focusin", on_focus.as_ref().unchecked_ref()).ok();
            thumb.add_event_listener_with_callback("focusout", on_blur.as_ref().unchecked_ref()).ok();
            on_focus.forget();
            on_blur.forget();
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
