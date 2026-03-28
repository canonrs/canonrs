#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::MouseEvent;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-theme-toggle", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {
        use leptos::leptos_dom::helpers::window;

        let el = root.clone();
        if el.get_attribute("data-theme-toggle-attached").as_deref() == Some("1") { return Ok(()); }
        el.set_attribute("data-theme-toggle-attached", "1").ok();

        // Aplicar tema inicial do localStorage
        if let Some(storage) = window().local_storage().ok().flatten() {
            if let Ok(Some(theme)) = storage.get_item("canonrs-theme") {
                if let Some(root_el) = web_sys::window().unwrap().document().unwrap().document_element() {
                    if theme == "dark" {
                        root_el.class_list().add_1("dark").ok();
                        el.set_inner_html("☀️");
                    } else {
                        root_el.class_list().remove_1("dark").ok();
                        el.set_inner_html("🌙");
                    }
                }
            }
        }

        let el_clone = el.clone();
        let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
            let Some(root_el) = web_sys::window().unwrap().document().unwrap().document_element() else { return; };
            let is_dark = root_el.class_list().contains("dark");
            if is_dark {
                root_el.class_list().remove_1("dark").ok();
                el_clone.set_inner_html("🌙");
                if let Some(storage) = window().local_storage().ok().flatten() {
                    storage.set_item("canonrs-theme", "light").ok();
                }
            } else {
                root_el.class_list().add_1("dark").ok();
                el_clone.set_inner_html("☀️");
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
