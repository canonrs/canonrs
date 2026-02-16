//! DocProgress behavior
//! Calculates and updates reading progress based on scroll position

#[cfg(feature = "hydrate")]
use super::register_behavior;
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;
#[cfg(feature = "hydrate")]
use leptos::leptos_dom::helpers::document;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use web_sys::Element;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-doc-progress", Box::new(|element_id, _state| {
        let Some(progress_el) = document().get_element_by_id(element_id) else {
            return Ok(());
        };

        setup_scroll_tracking(&progress_el)?;
        Ok(())
    }));
}

#[cfg(feature = "hydrate")]
fn setup_scroll_tracking(progress_el: &Element) -> BehaviorResult<()> {
    if progress_el.get_attribute("data-tracking-attached").as_deref() == Some("1") {
        return Ok(());
    }
    let _ = progress_el.set_attribute("data-tracking-attached", "1");

    let progress_clone = progress_el.clone();
    
    let scroll_handler = Closure::wrap(Box::new(move || {
        if let Some(window) = web_sys::window() {
            if let Some(doc_el) = document().document_element() {
                let scroll_top = window.scroll_y().unwrap_or(0.0);
                let scroll_height = doc_el.scroll_height() as f64;
                let client_height = window.inner_height()
                    .ok()
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);
                
                let max_scroll = (scroll_height - client_height).max(1.0);
                let progress_percent = ((scroll_top / max_scroll) * 100.0).clamp(0.0, 100.0);
                
                // Update data-progress attribute
                let _ = progress_clone.set_attribute(
                    "data-progress",
                    &format!("{:.0}", progress_percent)
                );
                
                // Update progress bar width via CSS variable or direct style
                if let Ok(Some(bar)) = progress_clone.query_selector("[data-doc-progress-bar]") {
                    if let Ok(html_el) = bar.dyn_into::<web_sys::HtmlElement>() {
                        let _ = html_el.style().set_property("width", &format!("{}%", progress_percent));
                    }
                }
            }
        }
    }) as Box<dyn FnMut()>);

    if let Some(window) = web_sys::window() {
        window.add_event_listener_with_callback(
            "scroll",
            scroll_handler.as_ref().unchecked_ref()
        ).map_err(|_| canonrs_shared::BehaviorError::JsError { 
            message: "scroll listener failed".into() 
        })?;
        
        // Initial calculation
        scroll_handler.as_ref().unchecked_ref::<js_sys::Function>().call0(&JsValue::NULL).ok();
    }

    scroll_handler.forget();
    Ok(())
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
