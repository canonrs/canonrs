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
    register_behavior("data-dropdown-menu", Box::new(|element_id, state| {
        let document = window().unwrap().document().unwrap();
        let dropdown = document.get_element_by_id(element_id)
            .ok_or_else(|| BehaviorError::ElementNotFound { selector: element_id.to_string() })?;

        let open_signal = state.open;
        let trigger_selector = format!("[data-dropdown-menu-trigger=\"{}\"]", element_id);

        if let Ok(Some(trigger)) = document.query_selector(&trigger_selector) {
            let content_selector = format!("#{} [data-dropdown-menu-content]", element_id);
            
            let dropdown_clone = dropdown.clone();
            let document_clone = document.clone();
            let trigger_selector_clone = trigger_selector.clone();
            
            let cb_toggle = Closure::wrap(Box::new(move |_: MouseEvent| {
                let current_state = dropdown_clone.get_attribute("data-state").unwrap_or_else(|| "closed".to_string());
                let is_open = current_state == "open";
                let new_state = !is_open;
                
                open_signal.set(new_state);
                dropdown_clone.set_attribute("data-state", if new_state { "open" } else { "closed" }).ok();
                
                if new_state {
                    if let (Ok(Some(content)), Ok(Some(trigger_el))) = (
                        document_clone.query_selector(&content_selector),
                        document_clone.query_selector(&trigger_selector_clone)
                    ) {
                        let trigger_rect = trigger_el.get_bounding_client_rect();
                        
                        if let Some(html_content) = content.dyn_ref::<HtmlElement>() {
                            let style = html_content.style();
                            let x = trigger_rect.left();
                            let y = trigger_rect.bottom() + 4.0;
                            
                            style.set_property("left", &format!("{}px", x)).ok();
                            style.set_property("top", &format!("{}px", y)).ok();
                        }
                    }
                }
            }) as Box<dyn FnMut(MouseEvent)>);
            trigger.add_event_listener_with_callback("click", cb_toggle.as_ref().unchecked_ref()).unwrap();
            cb_toggle.forget();

            let dropdown_clone = dropdown.clone();
            let cb_close = Closure::wrap(Box::new(move |e: MouseEvent| {
                if let Some(target) = e.target() {
                    if let Some(element) = target.dyn_ref::<web_sys::Element>() {
                        if element.closest("[data-dropdown-menu-content]").ok().flatten().is_none() &&
                           element.closest("[data-dropdown-menu-trigger]").ok().flatten().is_none() {
                            open_signal.set(false);
                            dropdown_clone.set_attribute("data-state", "closed").ok();
                        }
                    }
                }
            }) as Box<dyn FnMut(MouseEvent)>);
            document.add_event_listener_with_callback("click", cb_close.as_ref().unchecked_ref()).unwrap();
            cb_close.forget();
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
