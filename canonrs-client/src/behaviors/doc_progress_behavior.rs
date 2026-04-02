//! DocProgress behavior
//! Scroll target resolution (priority order):
//!   1. data-rs-scroll-target="id" explícito
//!   2. [data-rs-scroll-viewport] ancestral mais próximo
//!   3. Fallback: window/document

#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;
#[cfg(feature = "hydrate")]
use leptos::leptos_dom::helpers::document;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use web_sys::Element;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-doc-progress", Box::new(|root: &web_sys::Element, _state: &ComponentState| {
        setup_scroll_tracking(root)?;
        Ok(())
    }));
    register_behavior("data-rs-doc-progress-portal", Box::new(|root: &web_sys::Element, _state: &ComponentState| {
        setup_scroll_tracking(root)?;
        Ok(())
    }));
}

#[cfg(feature = "hydrate")]
fn find_scroll_container(el: &Element) -> Option<web_sys::HtmlElement> {
    use wasm_bindgen::JsCast;

    // 1. data-rs-scroll-target explícito
    if let Some(target_id) = el.get_attribute("data-rs-scroll-target") {
        if !target_id.is_empty() {
            if let Some(target) = document().get_element_by_id(&target_id) {
                if let Ok(html_el) = target.dyn_into::<web_sys::HtmlElement>() {
                    return Some(html_el);
                }
            }
        }
    }

    // 2. [data-rs-scroll-viewport] ancestral
    let mut current = el.parent_element();
    while let Some(parent) = current {
        if parent.has_attribute("data-rs-scroll-viewport") {
            if let Ok(html_el) = parent.clone().dyn_into::<web_sys::HtmlElement>() {
                return Some(html_el);
            }
        }
        current = parent.parent_element();
    }

    // 3. Fallback: None → window
    None
}

#[cfg(feature = "hydrate")]
fn update_progress(progress_el: &Element, container: &Option<web_sys::HtmlElement>) {
    use wasm_bindgen::JsCast;

    let (scroll_top, scroll_height, client_height) = if let Some(c) = container {
        (c.scroll_top() as f64, c.scroll_height() as f64, c.client_height() as f64)
    } else {
        let window = match web_sys::window() { Some(w) => w, None => return };
        let doc_el = match document().document_element() { Some(d) => d, None => return };
        (
            window.scroll_y().unwrap_or(0.0),
            doc_el.scroll_height() as f64,
            window.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(0.0),
        )
    };

    let max_scroll = scroll_height - client_height;
    if max_scroll <= 0.0 { return; }

    let pct = ((scroll_top / max_scroll) * 100.0).clamp(0.0, 100.0);
    let _ = progress_el.set_attribute("data-progress", &format!("{:.0}", pct));
    if let Ok(Some(bar)) = progress_el.query_selector("[data-rs-doc-progress-bar]") {
        if let Ok(bar_el) = bar.dyn_into::<web_sys::HtmlElement>() {
            let _ = bar_el.style().set_property("width", &format!("{}%", pct));
        }
    }
}

#[cfg(feature = "hydrate")]
fn setup_scroll_tracking(progress_el: &Element) -> BehaviorResult<()> {
    use wasm_bindgen::JsCast;

    if progress_el.get_attribute("data-tracking-attached").as_deref() == Some("1") {
        return Ok(());
    }
    let _ = progress_el.set_attribute("data-tracking-attached", "1");

    let container       = find_scroll_container(progress_el);
    let progress_scroll = progress_el.clone();
    let progress_init   = progress_el.clone();
    let container_s     = container.clone();
    let container_i     = container.clone();

    let handler = Closure::wrap(Box::new(move || {
        update_progress(&progress_scroll, &container_s);
    }) as Box<dyn FnMut()>);

    if let Some(ref c) = container {
        c.add_event_listener_with_callback("scroll", handler.as_ref().unchecked_ref())
            .map_err(|_| crate::BehaviorError::JsError { message: "scroll listener failed".into() })?;
    } else if let Some(window) = web_sys::window() {
        window.add_event_listener_with_callback("scroll", handler.as_ref().unchecked_ref())
            .map_err(|_| crate::BehaviorError::JsError { message: "scroll listener failed".into() })?;
    }

    handler.forget();

    if let Some(window) = web_sys::window() {
        let closure = Closure::once_into_js(move || {
            update_progress(&progress_init, &container_i);
        });
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.unchecked_ref(), 100,
        );
    }

    Ok(())
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
