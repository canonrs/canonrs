#[cfg(feature = "hydrate")]
use super::*;
#[cfg(feature = "hydrate")]
use canonrs_shared::{BehaviorResult, BehaviorError};
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use leptos::web_sys::{window, MouseEvent, HtmlElement};
#[cfg(feature = "hydrate")]
use leptos::prelude::Set;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-tooltip", Box::new(|element_id, state| {
        let document = window().unwrap().document().unwrap();
        let tooltip = document.get_element_by_id(element_id)
            .ok_or_else(|| BehaviorError::ElementNotFound { selector: element_id.to_string() })?;

        let open_signal = state.open;
        let trigger_selector = format!("[data-tooltip-trigger=\"{}\"]", element_id);

        if let Ok(Some(trigger)) = document.query_selector(&trigger_selector) {
            let content_selector = format!("#{} [data-tooltip-content]", element_id);
            
            let tooltip_clone = tooltip.clone();
            let document_clone = document.clone();
            let trigger_selector_clone = trigger_selector.clone();
            
            let cb_show = Closure::wrap(Box::new(move |_: MouseEvent| {
                open_signal.set(true);
                tooltip_clone.set_attribute("data-state", "open").ok();
                
                if let (Ok(Some(content)), Ok(Some(trigger_el))) = (
                    document_clone.query_selector(&content_selector),
                    document_clone.query_selector(&trigger_selector_clone)
                ) {
                    let trigger_rect = trigger_el.get_bounding_client_rect();
                    
                    if let Some(html_content) = content.dyn_ref::<HtmlElement>() {
                        let style = html_content.style();
                        let x = trigger_rect.left() + (trigger_rect.width() / 2.0);
                        let y = trigger_rect.top() - 8.0;
                        
                        style.set_property("left", &format!("{}px", x)).ok();
                        style.set_property("top", &format!("{}px", y)).ok();
                        style.set_property("transform", "translateX(-50%) translateY(-100%)").ok();
                    }
                }
            }) as Box<dyn FnMut(MouseEvent)>);
            trigger.add_event_listener_with_callback("mouseenter", cb_show.as_ref().unchecked_ref()).unwrap();
            cb_show.forget();

            let tooltip_clone = tooltip.clone();
            let cb_hide = Closure::wrap(Box::new(move |_: MouseEvent| {
                open_signal.set(false);
                tooltip_clone.set_attribute("data-state", "closed").ok();
            }) as Box<dyn FnMut(MouseEvent)>);
            trigger.add_event_listener_with_callback("mouseleave", cb_hide.as_ref().unchecked_ref()).unwrap();
            cb_hide.forget();
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
