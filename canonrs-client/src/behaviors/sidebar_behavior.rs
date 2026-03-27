#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-sidebar", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {
        if root.get_attribute("data-rs-sidebar-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-sidebar-attached", "1").ok();

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        // === RAIL MODE: hover expand/collapse ===
        if root.has_attribute("data-rs-rail") {
            let root_enter = root.clone();
            let root_leave = root.clone();

            let on_enter = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
                root_enter.set_attribute("data-rs-state", "expanded").ok();
            }) as Box<dyn FnMut(_)>);

            let on_leave = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
                root_leave.set_attribute("data-rs-state", "collapsed").ok();
            }) as Box<dyn FnMut(_)>);

            root.add_event_listener_with_callback("mouseenter", on_enter.as_ref().unchecked_ref()).ok();
            root.add_event_listener_with_callback("mouseleave", on_leave.as_ref().unchecked_ref()).ok();
            on_enter.forget();
            on_leave.forget();
            return Ok(());
        }

        // === PIN TOGGLE ===
        let root_pin = root.clone();
        let pin_closure = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let target = match e.target() {
                Some(t) => t,
                None => return,
            };
            let el = match target.dyn_into::<web_sys::Element>() {
                Ok(el) => el,
                Err(_) => return,
            };
            let is_pin = if el.has_attribute("data-rs-sidebar-pin-toggle") {
                true
            } else {
                el.closest("[data-rs-sidebar-pin-toggle]").ok().flatten().is_some()
            };
            if !is_pin { return; }

            let pinned = root_pin.get_attribute("data-rs-pinned").as_deref() == Some("true");
            if pinned {
                root_pin.remove_attribute("data-rs-pinned").ok();
                // update icon
                if let Some(btn) = root_pin.query_selector("[data-rs-sidebar-pin-toggle]").ok().flatten() {
                    btn.set_text_content(Some("📍"));
                }
                // persist
                if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
                    let _ = storage.set_item("sidebar-pinned", "false");
                }
            } else {
                root_pin.set_attribute("data-rs-pinned", "true").ok();
                root_pin.set_attribute("data-rs-state", "expanded").ok();
                if let Some(btn) = root_pin.query_selector("[data-rs-sidebar-pin-toggle]").ok().flatten() {
                    btn.set_text_content(Some("📌"));
                }
                if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
                    let _ = storage.set_item("sidebar-pinned", "true");
                }
            }
        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback("click", pin_closure.as_ref().unchecked_ref()).ok();
        pin_closure.forget();

        // === TOGGLE (expand/collapse) — só se não pinado ===
        let root_clone = root.clone();
        let closure = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let target = match e.target() {
                Some(t) => t,
                None => return,
            };
            let el = match target.dyn_into::<web_sys::Element>() {
                Ok(el) => el,
                Err(_) => return,
            };

            let toggle = if el.has_attribute("data-rs-sidebar-toggle") {
                Some(el)
            } else {
                el.closest("[data-rs-sidebar-toggle]").ok().flatten()
            };
            if toggle.is_none() { return; }
            if toggle.unwrap().closest("[data-rs-sidebar]").ok().flatten().as_ref() != Some(&root_clone) {
                return;
            }

            // Se pinado, toggle não faz nada
            if root_clone.get_attribute("data-rs-pinned").as_deref() == Some("true") { return; }

            let current = root_clone.get_attribute("data-rs-state").unwrap_or_default();
            if current == "collapsed" {
                root_clone.set_attribute("data-rs-state", "expanded").ok();
            } else {
                root_clone.set_attribute("data-rs-state", "collapsed").ok();
            }
        }) as Box<dyn FnMut(_)>);

        let window2 = web_sys::window().unwrap();
        let document2 = window2.document().unwrap();
        document2.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
        closure.forget();

        // === RESTORE PIN STATE from localStorage ===
        if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
            if let Ok(Some(val)) = storage.get_item("sidebar-pinned") {
                if val == "true" {
                    root.set_attribute("data-rs-pinned", "true").ok();
                    root.set_attribute("data-rs-state", "expanded").ok();
                    if let Some(btn) = root.query_selector("[data-rs-sidebar-pin-toggle]").ok().flatten() {
                        btn.set_text_content(Some("📌"));
                    }
                }
            }
        }


        // === GROUP COLLAPSIBLE ===
        let groups = root.query_selector_all("[data-rs-sidebar-group]").unwrap();
        for i in 0..groups.length() {
            let group = match groups.item(i) {
                Some(n) => match n.dyn_into::<web_sys::Element>() {
                    Ok(el) => el,
                    Err(_) => continue,
                },
                None => continue,
            };

            // Gera key única por índice
            let key = format!("sidebar-group-{}", i);

            // Restore do localStorage
            if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
                if let Ok(Some(val)) = storage.get_item(&key) {
                    group.set_attribute("data-rs-state", &val).ok();
                    if let Some(content) = group.query_selector("[data-rs-sidebar-group-content]").ok().flatten() {
                        if val == "collapsed" {
                            content.set_attribute("hidden", "").ok();
                        } else {
                            content.remove_attribute("hidden").ok();
                        }
                    }
                }
            }

            let group_clone = group.clone();
            let key_clone = key.clone();
            let toggle_closure = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
                let target = match e.target() {
                    Some(t) => t,
                    None => return,
                };
                let el = match target.dyn_into::<web_sys::Element>() {
                    Ok(el) => el,
                    Err(_) => return,
                };
                let is_toggle = el.has_attribute("data-rs-sidebar-group-toggle")
                    || el.closest("[data-rs-sidebar-group-toggle]").ok().flatten().is_some();
                if !is_toggle { return; }

                // Verifica se o toggle pertence a este grupo
                let belongs = el.closest("[data-rs-sidebar-group]").ok().flatten()
                    .as_ref() == Some(&group_clone);
                if !belongs { return; }

                let current = group_clone.get_attribute("data-rs-state").unwrap_or_else(|| "expanded".to_string());
                let next = if current == "collapsed" { "expanded" } else { "collapsed" };
                group_clone.set_attribute("data-rs-state", next).ok();

                if let Some(content) = group_clone.query_selector("[data-rs-sidebar-group-content]").ok().flatten() {
                    if next == "collapsed" {
                        content.set_attribute("hidden", "").ok();
                    } else {
                        content.remove_attribute("hidden").ok();
                    }
                }

                // Persist
                if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
                    let _ = storage.set_item(&key_clone, next);
                }
            }) as Box<dyn FnMut(_)>);

            root.add_event_listener_with_callback("click", toggle_closure.as_ref().unchecked_ref()).ok();
            toggle_closure.forget();
        }

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}

#[cfg(feature = "hydrate")]
pub fn init_sidebar() { register(); }
#[cfg(not(feature = "hydrate"))]
pub fn init_sidebar() {}
