#[cfg(feature = "hydrate")]
use leptos::leptos_dom::helpers::document;
#[cfg(feature = "hydrate")]
use wasm_bindgen::{JsCast, closure::Closure, JsValue};
#[cfg(feature = "hydrate")]
use super::register_behavior;
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;
#[cfg(feature = "hydrate")]
use web_sys::{Element, Event, HtmlElement, MutationObserver, MutationObserverInit};
#[cfg(feature = "hydrate")]
use std::rc::Rc;
#[cfg(feature = "hydrate")]
use std::cell::RefCell;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-drag-container", Box::new(|element_id, _state| -> BehaviorResult<()> {
        let doc = document();
        if let Some(container_el) = doc.get_element_by_id(element_id) {
            setup_drag_drop(&container_el)?;
        }
        Ok(())
    }));
}

#[cfg(feature = "hydrate")]
fn setup_drag_drop(container: &Element) -> BehaviorResult<()> {
    if container.get_attribute("data-drag-behavior-attached").as_deref() == Some("1") {
        return Ok(());
    }
    let _ = container.set_attribute("data-drag-behavior-attached", "1");

    ensure_handles_draggable(container);
    install_mutation_observer(container);

    let _ = container.set_attribute("data-drag-from-id", "");
    let _ = container.set_attribute("data-drag-to-id", "");

    {
        let container_clone = container.clone();
        let dragstart = Closure::wrap(Box::new(move |e: Event| {
            let Some(target) = e.target() else { return; };
            let Ok(el) = target.dyn_into::<Element>() else { return; };

            let handle = if el.has_attribute("data-drag-handle") {
                Some(el)
            } else {
                find_parent_with_attr(&el, "data-drag-handle")
            };

            let Some(handle) = handle else {
                e.prevent_default();
                return;
            };

            let Ok(item) = find_drag_item(&handle) else { return; };
            let Some(from_id) = item.get_attribute("data-drag-id") else { return; };

            let _ = container_clone.set_attribute("data-drag-from-id", &from_id);
            let _ = container_clone.set_attribute("data-drag-to-id", &from_id);
            let _ = item.class_list().add_1("is-dragging");
        }) as Box<dyn FnMut(_)>);

        let _ = container.add_event_listener_with_callback_and_bool("dragstart", dragstart.as_ref().unchecked_ref(), true);
        dragstart.forget();
    }

    {
        let container_clone = container.clone();
        let dragover = Closure::wrap(Box::new(move |e: Event| {
            e.prevent_default();
            let Some(target) = e.target() else { return; };
            let Ok(el) = target.dyn_into::<Element>() else { return; };
            let Ok(item) = find_drag_item(&el) else { return; };
            
            if let Some(to_id) = item.get_attribute("data-drag-id") {
                let _ = container_clone.set_attribute("data-drag-to-id", &to_id);
            }
        }) as Box<dyn FnMut(_)>);

        let _ = container.add_event_listener_with_callback("dragover", dragover.as_ref().unchecked_ref());
        dragover.forget();
    }

    {
        let container_clone = container.clone();
        let drop = Closure::wrap(Box::new(move |e: Event| {
            e.prevent_default();

            let from = container_clone.get_attribute("data-drag-from-id").unwrap_or_default();
            let to = container_clone.get_attribute("data-drag-to-id").unwrap_or_default();

            cleanup_drag_state(&container_clone);

            if !from.is_empty() && !to.is_empty() && from != to {
                if let Ok(ev) = web_sys::Event::new("canon:reorder") {
                    let _ = js_sys::Reflect::set(&ev, &JsValue::from_str("dragFrom"), &JsValue::from_str(&from));
                    let _ = js_sys::Reflect::set(&ev, &JsValue::from_str("dragTo"), &JsValue::from_str(&to));
                    let _ = container_clone.dispatch_event(&ev);
                }
            }
        }) as Box<dyn FnMut(_)>);

        let _ = container.add_event_listener_with_callback("drop", drop.as_ref().unchecked_ref());
        drop.forget();
    }

    {
        let container_clone = container.clone();
        let dragend = Closure::wrap(Box::new(move |_: Event| {
            cleanup_drag_state(&container_clone);
        }) as Box<dyn FnMut(_)>);

        let _ = container.add_event_listener_with_callback_and_bool("dragend", dragend.as_ref().unchecked_ref(), true);
        dragend.forget();
    }

    Ok(())
}

#[cfg(feature = "hydrate")]
fn ensure_handles_draggable(container: &Element) {
    if let Ok(handles) = container.query_selector_all("[data-drag-handle]") {
        for i in 0..handles.length() {
            if let Some(node) = handles.item(i) {
                if let Ok(handle) = node.dyn_into::<Element>() {
                    let _ = handle.set_attribute("draggable", "true");
                }
            }
        }
    }
}

#[cfg(feature = "hydrate")]
fn install_mutation_observer(container: &Element) {
    if container.get_attribute("data-drag-observer").as_deref() == Some("1") {
        return;
    }
    let _ = container.set_attribute("data-drag-observer", "1");

    let container_clone = container.clone();
    let cb = Closure::wrap(Box::new(move |_: js_sys::Array, _: MutationObserver| {
        ensure_handles_draggable(&container_clone);
    }) as Box<dyn FnMut(_, _)>);

    if let Ok(observer) = MutationObserver::new(cb.as_ref().unchecked_ref()) {
        let mut init = MutationObserverInit::new();
        init.child_list(true);
        init.subtree(true);
        let _ = observer.observe_with_options(container, &init);
        cb.forget();
        std::mem::forget(observer);
    }
}

#[cfg(feature = "hydrate")]
fn cleanup_drag_state(container: &Element) {
    let _ = container.set_attribute("data-drag-from-id", "");
    let _ = container.set_attribute("data-drag-to-id", "");
    
    if let Ok(items) = container.query_selector_all("[data-drag-item]") {
        for i in 0..items.length() {
            if let Some(node) = items.item(i) {
                if let Ok(item) = node.dyn_into::<Element>() {
                    let _ = item.class_list().remove_1("is-dragging");
                }
            }
        }
    }
}

#[cfg(feature = "hydrate")]
fn find_drag_item(el: &Element) -> Result<Element, ()> {
    let mut current = el.clone();
    loop {
        if current.has_attribute("data-drag-item") {
            return Ok(current);
        }
        match current.parent_element() {
            Some(parent) => current = parent,
            None => break,
        }
    }
    Err(())
}

#[cfg(feature = "hydrate")]
fn find_parent_with_attr(el: &Element, attr: &str) -> Option<Element> {
    let mut current = el.clone();
    loop {
        if current.has_attribute(attr) {
            return Some(current);
        }
        match current.parent_element() {
            Some(parent) => current = parent,
            None => return None,
        }
    }
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
