#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use std::cell::RefCell;
#[cfg(feature = "hydrate")]
use std::rc::Rc;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-resizable", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {

        if root.get_attribute("data-rs-resizable-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-resizable-attached", "1").ok();

        let orientation = root.get_attribute("data-rs-orientation").unwrap_or_else(|| "horizontal".to_string());
        let min_size = root.get_attribute("data-rs-min-size").and_then(|s| s.parse::<f64>().ok()).unwrap_or(20.0);
        let max_size = root.get_attribute("data-rs-max-size").and_then(|s| s.parse::<f64>().ok()).unwrap_or(80.0);
        let is_horizontal = orientation == "horizontal";

        let Ok(Some(handle_el)) = root.query_selector("[data-rs-resizable-handle]") else { return Ok(()); };
        let Ok(handle) = handle_el.dyn_into::<web_sys::HtmlElement>() else { return Ok(()); };

        // inicializar panels com default-size
        if let Ok(panels) = root.query_selector_all("[data-rs-resizable-panel]") {
            for i in 0..panels.length() {
                if let Some(node) = panels.item(i) {
                    if let Ok(el) = node.dyn_into::<web_sys::HtmlElement>() {
                        let size = el.get_attribute("data-rs-default-size")
                            .and_then(|s| s.parse::<f64>().ok())
                            .unwrap_or(50.0);
                        el.style().set_property("flex-basis", &format!("{}%", size)).ok();
                    }
                }
            }
        }

        let is_dragging = Rc::new(RefCell::new(false));
        let active_pointer: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(None));
        let container_rect: Rc<RefCell<(f64, f64)>> = Rc::new(RefCell::new((0.0, 0.0)));

        let is_dragging_down = is_dragging.clone();
        let is_dragging_move = is_dragging.clone();
        let is_dragging_up = is_dragging.clone();
        let active_pointer_down = active_pointer.clone();
        let active_pointer_move = active_pointer.clone();
        let active_pointer_up = active_pointer.clone();
        let container_rect_down = container_rect.clone();
        let container_rect_move = container_rect.clone();

        let root_down = root.clone();
        let handle_down = handle.clone();
        let on_down = Closure::wrap(Box::new(move |e: web_sys::PointerEvent| {
            e.prevent_default();
            let Ok(container) = root_down.clone().dyn_into::<web_sys::HtmlElement>() else { return; };
            let rect = container.get_bounding_client_rect();
            let size = if is_horizontal { rect.width() } else { rect.height() };
            let offset = if is_horizontal { rect.left() } else { rect.top() };
            *container_rect_down.borrow_mut() = (size, offset);
            *is_dragging_down.borrow_mut() = true;
            *active_pointer_down.borrow_mut() = Some(e.pointer_id());
            handle_down.set_pointer_capture(e.pointer_id()).ok();
        }) as Box<dyn FnMut(_)>);

        let root_move = root.clone();
        let on_move = Closure::wrap(Box::new(move |e: web_sys::PointerEvent| {
            if !*is_dragging_move.borrow() { return; }
            if *active_pointer_move.borrow() != Some(e.pointer_id()) { return; }
            let (size, offset) = *container_rect_move.borrow();
            if size == 0.0 { return; }
            let pos = if is_horizontal { e.client_x() as f64 } else { e.client_y() as f64 };
            let pct = ((pos - offset) / size * 100.0).max(min_size).min(max_size);
            if let Ok(panels) = root_move.query_selector_all("[data-rs-resizable-panel]") {
                if let Some(node) = panels.item(0) {
                    if let Ok(el) = node.dyn_into::<web_sys::HtmlElement>() {
                        el.style().set_property("flex-basis", &format!("{}%", pct)).ok();
                    }
                }
                if let Some(node) = panels.item(1) {
                    if let Ok(el) = node.dyn_into::<web_sys::HtmlElement>() {
                        el.style().set_property("flex-basis", &format!("{}%", 100.0 - pct)).ok();
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        let on_up = Closure::wrap(Box::new(move |e: web_sys::PointerEvent| {
            if *active_pointer_up.borrow() == Some(e.pointer_id()) {
                *is_dragging_up.borrow_mut() = false;
                *active_pointer_up.borrow_mut() = None;
            }
        }) as Box<dyn FnMut(_)>);

        handle.add_event_listener_with_callback("pointerdown", on_down.as_ref().unchecked_ref())
            .map_err(|_| crate::BehaviorError::JsError { message: "pointerdown".into() })?;
        handle.add_event_listener_with_callback("pointermove", on_move.as_ref().unchecked_ref())
            .map_err(|_| crate::BehaviorError::JsError { message: "pointermove".into() })?;
        handle.add_event_listener_with_callback("pointerup", on_up.as_ref().unchecked_ref())
            .map_err(|_| crate::BehaviorError::JsError { message: "pointerup".into() })?;

        on_down.forget();
        on_move.forget();
        on_up.forget();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
