#[cfg(feature = "hydrate")]
use super::register_behavior;
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;
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
pub fn register() {
    register_behavior("data-slider", Box::new(|id: &str, _state| -> BehaviorResult<()> {
        use leptos::leptos_dom::helpers::document;

        let Some(slider_el) = document().get_element_by_id(id) else { return Ok(()); };
        if slider_el.get_attribute("data-slider-attached").as_deref() == Some("1") { return Ok(()); }
        slider_el.set_attribute("data-slider-attached", "1").ok();

        let min: f64 = slider_el.get_attribute("aria-valuemin").and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let max: f64 = slider_el.get_attribute("aria-valuemax").and_then(|s| s.parse().ok()).unwrap_or(100.0);
        let val: f64 = slider_el.get_attribute("aria-valuenow").and_then(|s| s.parse().ok()).unwrap_or(0.0);

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
        }) as Box<dyn FnMut(_)>);

        let slider_move = slider_html.clone();
        let slider_el_move = slider_el.clone();
        let on_move = Closure::wrap(Box::new(move |e: web_sys::PointerEvent| {
            if !is_dragging_move.get() { return; }
            if *active_move.borrow() != Some(e.pointer_id()) { return; }
            let rect = slider_move.get_bounding_client_rect();
            let x = (e.client_x() as f64 - rect.left()).max(0.0).min(rect.width());
            let pct = (x / rect.width()) * 100.0;
            let value = min + (pct / 100.0) * (max - min);
            slider_el_move.set_attribute("aria-valuenow", &format!("{:.0}", value)).ok();
            update_slider_ui(&slider_el_move, pct);
        }) as Box<dyn FnMut(_)>);

        let on_up = Closure::wrap(Box::new(move |e: web_sys::PointerEvent| {
            if *active_up.borrow() == Some(e.pointer_id()) {
                is_dragging_up.set(false);
                *active_up.borrow_mut() = None;
            }
        }) as Box<dyn FnMut(_)>);

        slider_html.add_event_listener_with_callback("pointerdown", on_down.as_ref().unchecked_ref()).ok();
        slider_html.add_event_listener_with_callback("pointermove", on_move.as_ref().unchecked_ref()).ok();
        slider_html.add_event_listener_with_callback("pointerup", on_up.as_ref().unchecked_ref()).ok();
        on_down.forget();
        on_move.forget();
        on_up.forget();

        Ok(())
    }));
}

#[cfg(feature = "hydrate")]
fn update_slider_ui(slider_el: &web_sys::Element, percent: f64) {
    if let Some(range) = slider_el.query_selector("[data-slider-range]").ok().flatten() {
        range.set_attribute("style", &format!("width: {}%", percent)).ok();
    }
    if let Some(thumb) = slider_el.query_selector("[data-slider-thumb]").ok().flatten() {
        thumb.set_attribute("style", &format!("left: {}%", percent)).ok();
    }
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
