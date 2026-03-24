#[cfg(feature = "hydrate")]
use super::*;
#[cfg(feature = "hydrate")]
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use leptos::web_sys::{window, MouseEvent, Element, HtmlElement, Event};
#[cfg(feature = "hydrate")]

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-menubar", Box::new(|root: &web_sys::Element, _state: &ComponentState| {
        let _menubar = root;
        let document = web_sys::window().unwrap().document().unwrap();
        let trigger_selector = "[data-rs-menubar-trigger]";

        if let Ok(triggers) = root.query_selector_all(trigger_selector) {
            for i in 0..triggers.length() {
                if let Some(node) = triggers.get(i) {
                    if let Some(trigger) = node.dyn_ref::<Element>() {
                        let document_clone = document.clone();
                        let element_id_clone = "menubar".to_string();
                        let trigger_clone = trigger.clone();

                        let cb_toggle = Closure::wrap(Box::new(move |_: MouseEvent| {
                            let target_menu = trigger_clone.get_attribute("data-target").unwrap_or_default();

                            let open_menu_selector = format!("#{} [data-menubar-content][data-state='open']", element_id_clone);
                            let should_open = document_clone
                                .query_selector(&open_menu_selector)
                                .ok()
                                .flatten()
                                .and_then(|open_menu| open_menu.get_attribute("data-menu"))
                                .map(|open_menu_id| open_menu_id != target_menu)
                                .unwrap_or(true);

                            // Fechar tudo
                            if let Ok(all_triggers) = document_clone.query_selector_all(&format!("#{} [data-menubar-trigger]", element_id_clone)) {
                                for j in 0..all_triggers.length() {
                                    if let Some(n) = all_triggers.get(j) {
                                        if let Some(t) = n.dyn_ref::<Element>() {
                                            t.set_attribute("data-rs-state", "closed").ok();
                                            t.set_attribute("aria-expanded", "false").ok();
                                        }
                                    }
                                }
                            }

                            if let Ok(all_menus) = document_clone.query_selector_all(&format!("#{} [data-menubar-content]", element_id_clone)) {
                                for j in 0..all_menus.length() {
                                    if let Some(n) = all_menus.get(j) {
                                        if let Some(m) = n.dyn_ref::<Element>() {
                                            m.set_attribute("data-rs-state", "closed").ok();
                                        }
                                    }
                                }
                            }

                            // Abrir se necessário
                            if should_open {
                                trigger_clone.set_attribute("data-rs-state", "open").ok();
                                trigger_clone.set_attribute("aria-expanded", "true").ok();

                                let menu_selector = format!("#{} [data-menubar-content][data-menu='{}']", element_id_clone, target_menu);
                                if let Ok(Some(menu)) = document_clone.query_selector(&menu_selector) {
                                    menu.set_attribute("data-rs-state", "open").ok();

                                    if let Some(html_menu) = menu.dyn_ref::<HtmlElement>() {
                                        let trigger_rect = trigger_clone.get_bounding_client_rect();
                                        let style = html_menu.style();
                                        style.set_property("left", &format!("{}px", trigger_rect.left())).ok();
                                        style.set_property("top", &format!("{}px", trigger_rect.bottom() + 4.0)).ok();
                                    }
                                }
                            }
                        }) as Box<dyn FnMut(MouseEvent)>);
                        
                        trigger.add_event_listener_with_callback("click", cb_toggle.as_ref().unchecked_ref()).unwrap();
                        cb_toggle.forget();
                    }
                }
            }
        }

        // Click outside listener
        let document_clone = document.clone();
        let element_id_clone = "menubar".to_string();
        let cb_outside = Closure::wrap(Box::new(move |e: MouseEvent| {
            if let Some(target) = e.target() {
                if let Some(element) = target.dyn_ref::<Element>() {
                    let inside_menubar = element.closest(&format!("#{}", element_id_clone)).ok().flatten().is_some();
                    let inside_content = element.closest("[data-menubar-content]").ok().flatten().is_some();
                    
                    if !inside_menubar && !inside_content {
                        if let Ok(all_triggers) = document_clone.query_selector_all(&format!("#{} [data-menubar-trigger]", element_id_clone)) {
                            for j in 0..all_triggers.length() {
                                if let Some(n) = all_triggers.get(j) {
                                    if let Some(t) = n.dyn_ref::<Element>() {
                                        t.set_attribute("data-rs-state", "closed").ok();
                                        t.set_attribute("aria-expanded", "false").ok();
                                    }
                                }
                            }
                        }
                        if let Ok(all_menus) = document_clone.query_selector_all(&format!("#{} [data-menubar-content]", element_id_clone)) {
                            for j in 0..all_menus.length() {
                                if let Some(n) = all_menus.get(j) {
                                    if let Some(m) = n.dyn_ref::<Element>() {
                                        m.set_attribute("data-rs-state", "closed").ok();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(MouseEvent)>);
        document.add_event_listener_with_callback("click", cb_outside.as_ref().unchecked_ref()).unwrap();
        cb_outside.forget();

        // Scroll listener - fechar menu ao rolar
        let document_clone = document.clone();
        let element_id_clone = "menubar".to_string();
        let cb_scroll = Closure::wrap(Box::new(move |_: Event| {
            if let Ok(all_triggers) = document_clone.query_selector_all(&format!("#{} [data-menubar-trigger]", element_id_clone)) {
                for j in 0..all_triggers.length() {
                    if let Some(n) = all_triggers.get(j) {
                        if let Some(t) = n.dyn_ref::<Element>() {
                            t.set_attribute("data-rs-state", "closed").ok();
                            t.set_attribute("aria-expanded", "false").ok();
                        }
                    }
                }
            }
            if let Ok(all_menus) = document_clone.query_selector_all(&format!("#{} [data-menubar-content]", element_id_clone)) {
                for j in 0..all_menus.length() {
                    if let Some(n) = all_menus.get(j) {
                        if let Some(m) = n.dyn_ref::<Element>() {
                            m.set_attribute("data-rs-state", "closed").ok();
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(Event)>);
        window().unwrap().add_event_listener_with_callback("scroll", cb_scroll.as_ref().unchecked_ref()).unwrap();
        cb_scroll.forget();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
