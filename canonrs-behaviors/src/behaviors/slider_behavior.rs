#[cfg(feature = "hydrate")]
use leptos::leptos_dom::helpers::document;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use super::register_behavior;
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-slider", Box::new(|element_id, _state| -> BehaviorResult<()> {
        let doc = document();
        if let Some(slider_el) = doc.get_element_by_id(element_id) {
            // Lê min/max/value do elemento
            let min: f64 = slider_el.get_attribute("aria-valuemin").unwrap_or("0".into()).parse().unwrap_or(0.0);
            let max: f64 = slider_el.get_attribute("aria-valuemax").unwrap_or("100".into()).parse().unwrap_or(100.0);
            let val: f64 = slider_el.get_attribute("aria-valuenow").unwrap_or("0".into()).parse().unwrap_or(0.0);

            // Atualiza posição inicial
            let percent = ((val - min) / (max - min)) * 100.0;
            update_slider_ui(&slider_el, percent);

            // Drag behavior
            let slider_clone = slider_el.clone();
            let is_dragging = std::rc::Rc::new(std::cell::Cell::new(false));
            let is_dragging_mouse = is_dragging.clone();
            let is_dragging_move = is_dragging.clone();
            let is_dragging_up = is_dragging.clone();

            let min_c = min;
            let max_c = max;

            let mousedown = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: leptos::web_sys::MouseEvent| {
                is_dragging_mouse.set(true);
            }) as Box<dyn FnMut(_)>);

            let slider_move = slider_clone.clone();
            let mousemove = wasm_bindgen::closure::Closure::wrap(Box::new(move |e: leptos::web_sys::MouseEvent| {
                if !is_dragging_move.get() { return; }
                let rect = slider_move.get_bounding_client_rect();
                let x = (e.client_x() as f64 - rect.left()).max(0.0).min(rect.width());
                let percent = (x / rect.width()) * 100.0;
                let value = min_c + (percent / 100.0) * (max_c - min_c);
                slider_move.set_attribute("aria-valuenow", &value.to_string()).ok();
                update_slider_ui(&slider_move, percent);
            }) as Box<dyn FnMut(_)>);

            let mouseup = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: leptos::web_sys::MouseEvent| {
                is_dragging_up.set(false);
            }) as Box<dyn FnMut(_)>);

            slider_el.add_event_listener_with_callback("mousedown", mousedown.as_ref().unchecked_ref()).ok();
            doc.add_event_listener_with_callback("mousemove", mousemove.as_ref().unchecked_ref()).ok();
            doc.add_event_listener_with_callback("mouseup", mouseup.as_ref().unchecked_ref()).ok();

            mousedown.forget();
            mousemove.forget();
            mouseup.forget();
        }
        Ok(())
    }));
}

#[cfg(feature = "hydrate")]
fn update_slider_ui(slider_el: &leptos::web_sys::Element, percent: f64) {
    if let Some(range) = slider_el.query_selector("[data-slider-range]").ok().flatten() {
        range.set_attribute("style", &format!("width: {}%", percent)).ok();
    }
    if let Some(thumb) = slider_el.query_selector("[data-slider-thumb]").ok().flatten() {
        thumb.set_attribute("style", &format!("left: {}%", percent)).ok();
    }
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
