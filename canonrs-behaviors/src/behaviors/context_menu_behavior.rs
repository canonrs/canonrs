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
    register_behavior("data-context-menu", Box::new(|element_id, state| {
        let document = window().unwrap().document().unwrap();
        let context_menu = document.get_element_by_id(element_id)
            .ok_or_else(|| BehaviorError::ElementNotFound { selector: element_id.to_string() })?;

        let open_signal = state.open;
        let trigger_selector = format!("[data-context-menu-trigger=\"{}\"]", element_id);

        if let Ok(Some(trigger)) = document.query_selector(&trigger_selector) {
            let context_menu_clone = context_menu.clone();
            let content_selector = format!("#{} [data-context-menu-content]", element_id);
            let document_clone = document.clone();
            
            let cb_open = Closure::wrap(Box::new(move |e: MouseEvent| {
                e.prevent_default();
                
                if let Ok(Some(content)) = document_clone.query_selector(&content_selector) {
                    if let Some(html_content) = content.dyn_ref::<HtmlElement>() {
                        let style = html_content.style();
                        style.set_property("left", &format!("{}px", e.client_x())).ok();
                        style.set_property("top", &format!("{}px", e.client_y())).ok();
                    }
                }
                
                open_signal.set(true);
                context_menu_clone.set_attribute("data-state", "open").ok();
            }) as Box<dyn FnMut(MouseEvent)>);
            trigger.add_event_listener_with_callback("contextmenu", cb_open.as_ref().unchecked_ref()).unwrap();
            cb_open.forget();

            let context_menu_clone = context_menu.clone();
            let cb_close = Closure::wrap(Box::new(move |_: MouseEvent| {
                open_signal.set(false);
                context_menu_clone.set_attribute("data-state", "closed").ok();
            }) as Box<dyn FnMut(MouseEvent)>);
            document.add_event_listener_with_callback("click", cb_close.as_ref().unchecked_ref()).unwrap();
            cb_close.forget();
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
