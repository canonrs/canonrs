#[cfg(feature = "hydrate")]
use super::*;
#[cfg(feature = "hydrate")]
use canonrs_shared::{BehaviorResult, BehaviorError};
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use leptos::web_sys::{window, MouseEvent, KeyboardEvent, HtmlElement, FocusEvent};
#[cfg(feature = "hydrate")]
use leptos::prelude::Set;

#[cfg(feature = "hydrate")]
fn get_focusable_elements(container: &web_sys::Element) -> Vec<web_sys::Element> {
    let selector = "a[href],button:not([disabled]),input:not([disabled]),select:not([disabled]),textarea:not([disabled]),[tabindex]:not([tabindex=\"-1\"])";
    let mut elements = vec![];
    if let Ok(list) = container.query_selector_all(selector) {
        for i in 0..list.length() {
            if let Some(el) = list.item(i) {
                if let Ok(element) = el.dyn_into::<web_sys::Element>() { elements.push(element); }
            }
        }
    }
    elements
}

#[cfg(feature = "hydrate")]
fn open_dialog(dialog: &web_sys::Element, open_signal: leptos::prelude::RwSignal<bool>) {
    dialog.set_attribute("data-state", "open").ok();
    open_signal.set(true);

    // Scroll lock
    if let Some(body) = window().unwrap().document().unwrap().body() {
        body.style().set_property("overflow", "hidden").ok();
    }

    // Focus first focusable element
    let focusable = get_focusable_elements(dialog);
    if let Some(first) = focusable.first() {
        if let Ok(el) = first.clone().dyn_into::<HtmlElement>() {
            el.focus().ok();
        }
    }
}

#[cfg(feature = "hydrate")]
fn close_dialog(dialog: &web_sys::Element, open_signal: leptos::prelude::RwSignal<bool>) {
    dialog.set_attribute("data-state", "closed").ok();
    open_signal.set(false);

    // Restore scroll
    if let Some(body) = window().unwrap().document().unwrap().body() {
        body.style().remove_property("overflow").ok();
    }
}

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-dialog", Box::new(|element_id, state| {
        let document = window().unwrap().document().unwrap();
        let dialog = document.get_element_by_id(element_id)
            .ok_or_else(|| BehaviorError::ElementNotFound { selector: element_id.to_string() })?;

        let open_signal = state.open;
        let trigger_selector = format!("[data-dialog-trigger=\"{}\"]", element_id);

        // Trigger click → open
        if let Ok(Some(trigger)) = document.query_selector(&trigger_selector) {
            let dialog_clone = dialog.clone();
            let cb_open = Closure::wrap(Box::new(move |_: MouseEvent| {
                open_dialog(&dialog_clone, open_signal);
            }) as Box<dyn FnMut(MouseEvent)>);
            trigger.add_event_listener_with_callback("click", cb_open.as_ref().unchecked_ref()).unwrap();
            cb_open.forget();
        }

        // Overlay click → close
        let overlay_selector = format!("#{} [data-dialog-overlay]", element_id);
        if let Ok(Some(overlay)) = document.query_selector(&overlay_selector) {
            let dialog_clone = dialog.clone();
            let cb_close = Closure::wrap(Box::new(move |_: MouseEvent| {
                close_dialog(&dialog_clone, open_signal);
            }) as Box<dyn FnMut(MouseEvent)>);
            overlay.add_event_listener_with_callback("click", cb_close.as_ref().unchecked_ref()).unwrap();
            cb_close.forget();
        }


        // Close button → close
        let close_selector = format!("[data-dialog-close=\"{0}\"]", element_id);
        if let Ok(Some(close_btn)) = document.query_selector(&close_selector) {
            let dialog_clone = dialog.clone();
            let cb_close_btn = Closure::wrap(Box::new(move |_: MouseEvent| {
                close_dialog(&dialog_clone, open_signal);
            }) as Box<dyn FnMut(MouseEvent)>);
            close_btn.add_event_listener_with_callback("click", cb_close_btn.as_ref().unchecked_ref()).unwrap();
            cb_close_btn.forget();
        }

        // Escape key → close
        {
            let dialog_clone = dialog.clone();
            let cb_key = Closure::wrap(Box::new(move |e: KeyboardEvent| {
                if e.key() == "Escape" {
                    let state = dialog_clone.get_attribute("data-state").unwrap_or_default();
                    if state == "open" {
                        close_dialog(&dialog_clone, open_signal);
                    }
                }
            }) as Box<dyn FnMut(KeyboardEvent)>);
            document.add_event_listener_with_callback("keydown", cb_key.as_ref().unchecked_ref()).unwrap();
            cb_key.forget();
        }

        // Focus trap
        {
            let dialog_clone = dialog.clone();
            let cb_focus = Closure::wrap(Box::new(move |e: KeyboardEvent| {
                let state = dialog_clone.get_attribute("data-state").unwrap_or_default();
                if state != "open" || e.key() != "Tab" { return; }

                let focusable = get_focusable_elements(&dialog_clone);
                if focusable.is_empty() { return; }

                let first = focusable.first().unwrap().clone();
                let last = focusable.last().unwrap().clone();

                let doc = window().unwrap().document().unwrap();
                let active = doc.active_element();

                if e.shift_key() {
                    if active.as_ref() == Some(&first) {
                        e.prevent_default();
                        if let Ok(el) = last.dyn_into::<HtmlElement>() { el.focus().ok(); }
                    }
                } else {
                    if active.as_ref() == Some(&last) {
                        e.prevent_default();
                        if let Ok(el) = first.dyn_into::<HtmlElement>() { el.focus().ok(); }
                    }
                }
            }) as Box<dyn FnMut(KeyboardEvent)>);
            document.add_event_listener_with_callback("keydown", cb_focus.as_ref().unchecked_ref()).unwrap();
            cb_focus.forget();
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
