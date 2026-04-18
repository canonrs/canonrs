//! Menubar Interaction Engine

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::runtime::{lifecycle, state, query};
use web_sys::Element;

fn close_all(root: &Element) {
    for menu in query::all(root, "[data-rs-menubar-menu]") {
        state::remove_state(&menu, "open");
        state::add_state(&menu, "closed");
        if let Some(trigger) = query::first(&menu, "[data-rs-menubar-trigger]") {
            let _ = trigger.set_attribute("aria-expanded", "false");
        }
    }
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    // init — all menus start closed
    for menu in query::all(&root, "[data-rs-menubar-menu]") {
        state::add_state(&menu, "closed");
    }

    // click trigger → toggle
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(trigger) = target.closest("[data-rs-menubar-trigger]").ok().flatten() else { return };
            if trigger.get_attribute("aria-disabled").as_deref() == Some("true") { return; }
            let Some(menu) = trigger.closest("[data-rs-menubar-menu]").ok().flatten() else { return };
            e.stop_propagation();
            let is_open = state::has(&menu, "open");
            close_all(&root_cb);
            if !is_open {
                state::remove_state(&menu, "closed");
                state::add_state(&menu, "open");
                let _ = trigger.set_attribute("aria-expanded", "true");
            }
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // click item → close all
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-menubar-item]").ok().flatten().is_none() { return; }
            if target.get_attribute("aria-disabled").as_deref() == Some("true") { return; }
            close_all(&root_cb);
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // hover — trigger
    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if let Some(trigger) = target.closest("[data-rs-menubar-trigger]").ok().flatten() {
                state::add_state(&trigger, "hover");
            } else if let Some(item) = target.closest("[data-rs-menubar-item]").ok().flatten() {
                state::add_state(&item, "hover");
            }
        }));
        let _ = root.add_event_listener_with_callback("mouseover", cb.as_ref().unchecked_ref());
        cb.forget();
    }
    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if let Some(trigger) = target.closest("[data-rs-menubar-trigger]").ok().flatten() {
                state::remove_state(&trigger, "hover");
            } else if let Some(item) = target.closest("[data-rs-menubar-item]").ok().flatten() {
                state::remove_state(&item, "hover");
            }
        }));
        let _ = root.add_event_listener_with_callback("mouseout", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // click outside → close all
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if root_cb.contains(Some(&target)) { return; }
            close_all(&root_cb);
        }));
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let _ = doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        }
        cb.forget();
    }

    // keyboard
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };

            // navegação entre items do content aberto
            if let Some(_item) = target.closest("[data-rs-menubar-item]").ok().flatten() {
                let Some(menu) = target.closest("[data-rs-menubar-menu]").ok().flatten() else { return };
                let items: Vec<Element> = query::all(&menu, "[data-rs-menubar-item]")
                    .into_iter()
                    .filter(|el| el.get_attribute("aria-disabled").as_deref() != Some("true"))
                    .collect();
                let len = items.len();
                let pos = items.iter().position(|el| el.contains(Some(target.as_ref())));

                match e.key().as_str() {
                    "ArrowDown" => {
                        e.prevent_default();
                        let next = pos.map(|p| (p + 1).min(len - 1)).unwrap_or(0);
                        if let Ok(h) = items[next].clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); }
                    }
                    "ArrowUp" => {
                        e.prevent_default();
                        let prev = pos.map(|p| if p == 0 { 0 } else { p - 1 }).unwrap_or(0);
                        if let Ok(h) = items[prev].clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); }
                    }
                    "Enter" | " " => {
                        e.prevent_default();
                        if let Ok(h) = target.clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.click(); }
                    }
                    "Escape" => {
                        e.prevent_default();
                        if let Some(menu) = target.closest("[data-rs-menubar-menu]").ok().flatten() {
                            state::remove_state(&menu, "open");
                            state::add_state(&menu, "closed");
                            if let Some(trigger) = query::first(&menu, "[data-rs-menubar-trigger]") {
                                let _ = trigger.set_attribute("aria-expanded", "false");
                                if let Ok(h) = trigger.dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); }
                            }
                        }
                    }
                    _ => {}
                }
                return;
            }

            // navegação entre triggers do menubar
            if let Some(_trigger) = target.closest("[data-rs-menubar-trigger]").ok().flatten() {
                let triggers: Vec<Element> = query::all(&root_cb, "[data-rs-menubar-trigger]")
                    .into_iter()
                    .filter(|el| el.get_attribute("aria-disabled").as_deref() != Some("true"))
                    .collect();
                let len = triggers.len();
                let pos = triggers.iter().position(|el| el.contains(Some(target.as_ref())));

                match e.key().as_str() {
                    "ArrowRight" => {
                        e.prevent_default();
                        let next = pos.map(|p| (p + 1) % len).unwrap_or(0);
                        if let Ok(h) = triggers[next].clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); }
                    }
                    "ArrowLeft" => {
                        e.prevent_default();
                        let prev = pos.map(|p| if p == 0 { len - 1 } else { p - 1 }).unwrap_or(0);
                        if let Ok(h) = triggers[prev].clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); }
                    }
                    "ArrowDown" | "Enter" | " " => {
                        e.prevent_default();
                        if let Some(trigger) = target.closest("[data-rs-menubar-trigger]").ok().flatten() {
                            if let Some(menu) = trigger.closest("[data-rs-menubar-menu]").ok().flatten() {
                                let is_open = state::has(&menu, "open");
                                close_all(&root_cb);
                                if !is_open {
                                    state::remove_state(&menu, "closed");
                                    state::add_state(&menu, "open");
                                    let _ = trigger.set_attribute("aria-expanded", "true");
                                    // foca primeiro item
                                    let items = query::all(&menu, "[data-rs-menubar-item]");
                                    if let Some(first) = items.first() {
                                        if let Ok(h) = first.clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); }
                                    }
                                }
                            }
                        }
                    }
                    "Escape" => {
                        e.prevent_default();
                        if let Some(menu) = target.closest("[data-rs-menubar-menu]").ok().flatten() {
                            state::remove_state(&menu, "open");
                            state::add_state(&menu, "closed");
                            if let Some(trigger) = query::first(&menu, "[data-rs-menubar-trigger]") {
                                let _ = trigger.set_attribute("aria-expanded", "false");
                                if let Ok(h) = trigger.dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }));
        let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // Escape → close all
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            if e.key() == "Escape" { close_all(&root_cb); }
        }));
        if let Some(win) = web_sys::window() {
            let _ = win.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        }
        cb.forget();
    }
}
