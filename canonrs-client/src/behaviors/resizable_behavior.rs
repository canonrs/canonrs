#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_core::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::Element;
#[cfg(feature = "hydrate")]
use std::cell::RefCell;
#[cfg(feature = "hydrate")]
use std::rc::Rc;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-resizable", Box::new(|root: &web_sys::Element, _state: &ComponentState| {
        setup_resizable(root)?;
        Ok(())
    }));
}

#[cfg(feature = "hydrate")]
fn setup_resizable(resizable: &Element) -> BehaviorResult<()> {
    if resizable.get_attribute("data-resizable-attached").as_deref() == Some("1") {
        return Ok(());
    }
    resizable.set_attribute("data-resizable-attached", "1").ok();

    let Some(wrapper) = resizable.query_selector("[data-resizable-wrapper]").ok().flatten() else {
        return Ok(());
    };

    let direction = wrapper.get_attribute("data-direction").unwrap_or_else(|| "horizontal".to_string());
    let min_size = wrapper.get_attribute("data-min-size")
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(20.0);
    let max_size = wrapper.get_attribute("data-max-size")
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(80.0);

    let Some(handle) = resizable.query_selector("[data-resizable-handle]").ok().flatten() else {
        return Ok(());
    };

    let handle_el = match handle.dyn_into::<web_sys::HtmlElement>() {
        Ok(el) => el,
        Err(_) => return Ok(()),
    };

    handle_el.set_attribute("aria-orientation", &direction).ok();
    handle_el.set_attribute("aria-valuemin", &min_size.to_string()).ok();
    handle_el.set_attribute("aria-valuemax", &max_size.to_string()).ok();
    handle_el.set_attribute("aria-valuenow", "50").ok();

    let is_horizontal = direction == "horizontal";
    let is_dragging = Rc::new(RefCell::new(false));
    let active_pointer: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(None));

    let is_dragging_down = is_dragging.clone();
    let is_dragging_move = is_dragging.clone();
    let is_dragging_up = is_dragging.clone();
    let active_pointer_down = active_pointer.clone();
    let active_pointer_move = active_pointer.clone();
    let active_pointer_up = active_pointer.clone();

    let container_rect: Rc<RefCell<(f64, f64)>> = Rc::new(RefCell::new((0.0, 0.0)));
    let container_rect_down = container_rect.clone();
    let container_rect_move = container_rect.clone();

    let resizable_clone = resizable.clone();
    let handle_down = handle_el.clone();

    let on_down = Closure::wrap(Box::new(move |e: web_sys::PointerEvent| {
        e.prevent_default();
        let Ok(container) = resizable_clone.clone().dyn_into::<web_sys::HtmlElement>() else { return; };
        let rect = container.get_bounding_client_rect();
        let size = if is_horizontal { rect.width() } else { rect.height() };
        let offset = if is_horizontal { rect.left() } else { rect.top() };
        *container_rect_down.borrow_mut() = (size, offset);
        *is_dragging_down.borrow_mut() = true;
        *active_pointer_down.borrow_mut() = Some(e.pointer_id());
        handle_down.set_pointer_capture(e.pointer_id()).ok();
    }) as Box<dyn FnMut(_)>);

    let resizable_for_move = resizable.clone();
    let handle_move = handle_el.clone();

    let on_move = Closure::wrap(Box::new(move |e: web_sys::PointerEvent| {
        if !*is_dragging_move.borrow() { return; }
        if *active_pointer_move.borrow() != Some(e.pointer_id()) { return; }

        let (size, offset) = *container_rect_move.borrow();
        if size == 0.0 { return; }

        let current_pos = if is_horizontal { e.client_x() as f64 } else { e.client_y() as f64 };
        let percentage = ((current_pos - offset) / size * 100.0).max(min_size).min(max_size);

        handle_move.set_attribute("aria-valuenow", &(percentage as u32).to_string()).ok();

        if let Ok(panels) = resizable_for_move.query_selector_all("[data-resizable-panel]") {
            if let Some(node) = panels.item(0) {
                if let Ok(el) = node.dyn_into::<web_sys::HtmlElement>() {
                    el.style().set_property("flex-basis", &format!("{}%", percentage)).ok();
                }
            }
            if let Some(node) = panels.item(1) {
                if let Ok(el) = node.dyn_into::<web_sys::HtmlElement>() {
                    el.style().set_property("flex-basis", &format!("{}%", 100.0 - percentage)).ok();
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

    handle_el.add_event_listener_with_callback("pointerdown", on_down.as_ref().unchecked_ref())
        .map_err(|_| canonrs_core::BehaviorError::JsError { message: "pointerdown".into() })?;
    handle_el.add_event_listener_with_callback("pointermove", on_move.as_ref().unchecked_ref())
        .map_err(|_| canonrs_core::BehaviorError::JsError { message: "pointermove".into() })?;
    handle_el.add_event_listener_with_callback("pointerup", on_up.as_ref().unchecked_ref())
        .map_err(|_| canonrs_core::BehaviorError::JsError { message: "pointerup".into() })?;

    on_down.forget();
    on_move.forget();
    on_up.forget();

    Ok(())
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
