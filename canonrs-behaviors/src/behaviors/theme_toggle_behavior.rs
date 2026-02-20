#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::MouseEvent;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-theme-toggle", Box::new(|id: &str, _state: &ComponentState| -> BehaviorResult<()> {
        use leptos::leptos_dom::helpers::{document, window};

        let Some(el) = document().get_element_by_id(id) else { return Ok(()); };
        if el.get_attribute("data-theme-toggle-attached").as_deref() == Some("1") { return Ok(()); }
        el.set_attribute("data-theme-toggle-attached", "1").ok();

        // Aplicar tema inicial do localStorage
        if let Some(storage) = window().local_storage().ok().flatten() {
            if let Ok(Some(theme)) = storage.get_item("canonrs-theme") {
                if let Some(root) = document().document_element() {
                    if theme == "dark" {
                        root.class_list().add_1("dark").ok();
                    } else {
                        root.class_list().remove_1("dark").ok();
                    }
                }
            }
        }

        let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
            let Some(root) = document().document_element() else { return; };
            let is_dark = root.class_list().contains("dark");
            if is_dark {
                root.class_list().remove_1("dark").ok();
                if let Some(storage) = window().local_storage().ok().flatten() {
                    storage.set_item("canonrs-theme", "light").ok();
                }
            } else {
                root.class_list().add_1("dark").ok();
                if let Some(storage) = window().local_storage().ok().flatten() {
                    storage.set_item("canonrs-theme", "dark").ok();
                }
            }
        }) as Box<dyn FnMut(_)>);

        el.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
