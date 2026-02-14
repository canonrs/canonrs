#[cfg(feature = "hydrate")]
use leptos::leptos_dom::helpers::{document, window};
#[cfg(feature = "hydrate")]
use wasm_bindgen::{JsCast, closure::Closure, JsValue};
#[cfg(feature = "hydrate")]
use super::register_behavior;
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;
#[cfg(feature = "hydrate")]
use web_sys::{Element, MouseEvent};
#[cfg(feature = "hydrate")]
use std::rc::Rc;
#[cfg(feature = "hydrate")]
use std::cell::RefCell;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-resize-container", Box::new(|element_id, _state| -> BehaviorResult<()> {
        let doc = document();
        if let Some(container_el) = doc.get_element_by_id(element_id) {
            setup_resize(&container_el)?;
        }
        Ok(())
    }));
}

#[cfg(feature = "hydrate")]
fn setup_resize(container: &Element) -> BehaviorResult<()> {
    if container.get_attribute("data-resize-behavior-attached").as_deref() == Some("1") {
        return Ok(());
    }
    let _ = container.set_attribute("data-resize-behavior-attached", "1");

    let is_resizing = Rc::new(RefCell::new(false));
    let start_x = Rc::new(RefCell::new(0i32));
    let start_width = Rc::new(RefCell::new(0u32));
    let current_col = Rc::new(RefCell::new(String::new()));

    // Mousedown on WINDOW (capture phase)
    {
        let container_clone = container.clone();
        let is_resizing = is_resizing.clone();
        let start_x = start_x.clone();
        let start_width = start_width.clone();
        let current_col = current_col.clone();

        let mousedown = Closure::wrap(Box::new(move |e: MouseEvent| {
            let Some(target) = e.target() else { return; };
            let Ok(el) = target.dyn_into::<Element>() else { return; };
            
            if !el.has_attribute("data-resize-handle") {
                return;
            }

            e.prevent_default();
            e.stop_propagation();
            
            let Some(col_id) = el.get_attribute("data-resize-col-id") else { return; };
            
            // Pegar largura inicial do th parent
            let mut parent = el.parent_element();
            while let Some(p) = parent {
                if p.tag_name() == "TH" {
                    let width = p.client_width() as u32;
                    *start_width.borrow_mut() = width;
                    break;
                }
                parent = p.parent_element();
            }
            
            *is_resizing.borrow_mut() = true;
            *start_x.borrow_mut() = e.client_x();
            *current_col.borrow_mut() = col_id.clone();

            if let Ok(ev) = web_sys::Event::new("canon:resize:start") {
                let _ = js_sys::Reflect::set(&ev, &JsValue::from_str("colId"), &JsValue::from_str(&col_id));
                let _ = container_clone.dispatch_event(&ev);
            }
        }) as Box<dyn FnMut(_)>);

        let _ = window().add_event_listener_with_callback_and_bool("mousedown", mousedown.as_ref().unchecked_ref(), true);
        mousedown.forget();
    }

    // Mousemove on window
    {
        let container_clone = container.clone();
        let is_resizing = is_resizing.clone();
        let start_x = start_x.clone();
        let start_width = start_width.clone();
        let current_col = current_col.clone();

        let mousemove = Closure::wrap(Box::new(move |e: MouseEvent| {
            if !*is_resizing.borrow() {
                return;
            }

            let delta = e.client_x() - *start_x.borrow();
            let new_width = (*start_width.borrow() as i32 + delta).max(50) as u32;
            let col = current_col.borrow().clone();

            if let Ok(ev) = web_sys::Event::new("canon:resize:move") {
                let _ = js_sys::Reflect::set(&ev, &JsValue::from_str("colId"), &JsValue::from_str(&col));
                let _ = js_sys::Reflect::set(&ev, &JsValue::from_str("width"), &JsValue::from(new_width));
                let _ = container_clone.dispatch_event(&ev);
            }
        }) as Box<dyn FnMut(_)>);

        let _ = window().add_event_listener_with_callback("mousemove", mousemove.as_ref().unchecked_ref());
        mousemove.forget();
    }

    // Mouseup on window
    {
        let container_clone = container.clone();
        let is_resizing = is_resizing.clone();
        let current_col = current_col.clone();

        let mouseup = Closure::wrap(Box::new(move |_: MouseEvent| {
            if !*is_resizing.borrow() {
                return;
            }

            *is_resizing.borrow_mut() = false;
            let col = current_col.borrow().clone();

            if let Ok(ev) = web_sys::Event::new("canon:resize:end") {
                let _ = js_sys::Reflect::set(&ev, &JsValue::from_str("colId"), &JsValue::from_str(&col));
                let _ = container_clone.dispatch_event(&ev);
            }
        }) as Box<dyn FnMut(_)>);

        let _ = window().add_event_listener_with_callback("mouseup", mouseup.as_ref().unchecked_ref());
        mouseup.forget();
    }

    Ok(())
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
