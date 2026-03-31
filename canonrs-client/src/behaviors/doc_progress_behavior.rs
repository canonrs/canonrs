//! DocProgress behavior
//! Suporta dois modos:
//!   1. [data-rs-doc-progress]       — standalone, posicionamento próprio
//!   2. [data-rs-doc-progress-portal] — portal, injetado pelo site no layout

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
    // Modo standalone
    register_behavior("data-rs-doc-progress", Box::new(|root: &web_sys::Element, _state: &ComponentState| {
        setup_scroll_tracking(root)?;
        Ok(())
    }));
    // Modo portal
    register_behavior("data-rs-doc-progress-portal", Box::new(|root: &web_sys::Element, _state: &ComponentState| {
        setup_scroll_tracking(root)?;
        Ok(())
    }));
}

#[cfg(feature = "hydrate")]
fn update_progress(progress_el: &Element) {
    let window = match web_sys::window() { Some(w) => w, None => return };
    let doc_el = match document().document_element() { Some(d) => d, None => return };

    let scroll_top    = window.scroll_y().unwrap_or(0.0);
    let scroll_height = doc_el.scroll_height() as f64;
    let client_height = window.inner_height().ok()
        .and_then(|v| v.as_f64()).unwrap_or(0.0);
    let max_scroll = scroll_height - client_height;

    if max_scroll <= 0.0 { return; }

    let progress_percent = ((scroll_top / max_scroll) * 100.0).clamp(0.0, 100.0);

    let _ = progress_el.set_attribute("data-progress", &format!("{:.0}", progress_percent));
    if let Ok(Some(bar)) = progress_el.query_selector("[data-rs-doc-progress-bar]") {
        if let Ok(bar_el) = bar.dyn_into::<web_sys::HtmlElement>() {
            let _ = bar_el.style().set_property("width", &format!("{}%", progress_percent));
        }
    }
}

#[cfg(feature = "hydrate")]
fn setup_scroll_tracking(progress_el: &Element) -> BehaviorResult<()> {
    if progress_el.get_attribute("data-tracking-attached").as_deref() == Some("1") {
        return Ok(());
    }
    let _ = progress_el.set_attribute("data-tracking-attached", "1");

    let progress_scroll = progress_el.clone();
    let progress_init   = progress_el.clone();

    let scroll_handler = Closure::wrap(Box::new(move || {
        update_progress(&progress_scroll);
    }) as Box<dyn FnMut()>);

    if let Some(window) = web_sys::window() {
        window.add_event_listener_with_callback(
            "scroll",
            scroll_handler.as_ref().unchecked_ref()
        ).map_err(|_| crate::BehaviorError::JsError {
            message: "scroll listener failed".into()
        })?;
    }

    scroll_handler.forget();

    // Initial calculation após DOM pronto
    if let Some(window) = web_sys::window() {
        let closure = Closure::once_into_js(move || {
            update_progress(&progress_init);
        });
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.unchecked_ref(),
            100,
        );
    }

    Ok(())
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
