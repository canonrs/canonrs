#[cfg(feature = "hydrate")]
use super::*;
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use leptos::web_sys::{window, MouseEvent};

#[cfg(feature = "hydrate")]
pub fn register() {
    web_sys::console::log_1(&"🎯 Registering theme-toggle".into());
    
    register_behavior("data-theme-toggle", Box::new(|_, _state| {
        web_sys::console::log_1(&"🔥 theme-toggle behavior init".into());
        
        let document = window().unwrap().document().unwrap();
        let root = document.document_element().unwrap();

        // Aplica estado inicial do localStorage
        if let Ok(Some(storage)) = window().unwrap().local_storage() {
            if let Ok(Some(theme)) = storage.get_item("canonrs-theme") {
                web_sys::console::log_1(&format!("📦 Initial theme: {}", theme).into());
                if theme == "Dark" {
                    root.class_list().add_1("dark").ok();
                }
            }
        }

        let el = document
            .query_selector("[data-theme-toggle]")
            .unwrap()
            .expect("data-theme-toggle element not found");

        web_sys::console::log_1(&"✅ Element found, attaching listener".into());

        let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
            web_sys::console::log_1(&"🖱️ Click detected!".into());
            
            if root.class_list().contains("dark") {
                web_sys::console::log_1(&"☀️ Switching to Light".into());
                root.class_list().remove_1("dark").ok();
                if let Ok(Some(storage)) = window().unwrap().local_storage() {
                    storage.set_item("canonrs-theme", "Light").ok();
                }
            } else {
                web_sys::console::log_1(&"🌙 Switching to Dark".into());
                root.class_list().add_1("dark").ok();
                if let Ok(Some(storage)) = window().unwrap().local_storage() {
                    storage.set_item("canonrs-theme", "Dark").ok();
                }
            }
        }) as Box<dyn FnMut(MouseEvent)>);

        el.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())
            .unwrap();

        cb.forget();
        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
