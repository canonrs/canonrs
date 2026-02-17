#[cfg(feature = "hydrate")]
use super::register_behavior;
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;
#[cfg(feature = "hydrate")]
use leptos::leptos_dom::helpers::document;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::Element;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-resizable", Box::new(|element_id, _state| {
        let Some(resizable) = document().get_element_by_id(element_id) else {
            return Ok(());
        };
        
        setup_resizable(&resizable)?;
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

    handle.set_attribute("aria-orientation", &direction).ok();
    handle.set_attribute("aria-valuemin", &min_size.to_string()).ok();
    handle.set_attribute("aria-valuemax", &max_size.to_string()).ok();

    let resizable_clone = resizable.clone();
    let is_horizontal = direction == "horizontal";
    
    let on_pointerdown = Closure::wrap(Box::new(move |e: web_sys::PointerEvent| {
        e.prevent_default();
        
        let container: web_sys::HtmlElement = resizable_clone.clone().dyn_into().unwrap();
        let rect = container.get_bounding_client_rect();
        
        let container_size = if is_horizontal { rect.width() } else { rect.height() };
        let offset = if is_horizontal { rect.left() } else { rect.top() };

        let resizable_for_move = resizable_clone.clone();
        
        let on_move = Closure::wrap(Box::new(move |e: web_sys::PointerEvent| {
            let current_pos = if is_horizontal {
                e.client_x() as f64
            } else {
                e.client_y() as f64
            };

            let relative_pos = current_pos - offset;
            let percentage = (relative_pos / container_size * 100.0).max(min_size).min(max_size);
            
            // Update panels
            if let Ok(panels) = resizable_for_move.query_selector_all("[data-resizable-panel-content]") {
                if let Some(node) = panels.item(0) {
                    if let Ok(panel) = node.dyn_into::<web_sys::HtmlElement>() {
                        let _ = panel.style().set_property("flex-basis", &format!("{}%", percentage));
                    }
                }
                if let Some(node) = panels.item(1) {
                    if let Ok(panel) = node.dyn_into::<web_sys::HtmlElement>() {
                        let _ = panel.style().set_property("flex-basis", &format!("{}%", 100.0 - percentage));
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        let on_up = Closure::wrap(Box::new(move |_: web_sys::PointerEvent| {
            // Cleanup
        }) as Box<dyn FnMut(_)>);

        let _ = document().add_event_listener_with_callback("pointermove", on_move.as_ref().unchecked_ref());
        let _ = document().add_event_listener_with_callback("pointerup", on_up.as_ref().unchecked_ref());
        
        on_move.forget();
        on_up.forget();
        
    }) as Box<dyn FnMut(_)>);

    handle.add_event_listener_with_callback("pointerdown", on_pointerdown.as_ref().unchecked_ref())
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "pointerdown".into() })?;
    
    on_pointerdown.forget();

    Ok(())
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
