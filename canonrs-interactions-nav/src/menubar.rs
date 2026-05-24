//! Menubar Interaction Engine
//! Core: dom/{state, query}

use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::{state, query};
use canonrs_interactions_core::runtime::listeners;

fn close_all(root: &Element) {
    for menu in query::all(root, "[data-rs-menubar-menu]") {
        state::remove(&menu, canonrs_interactions_core::dom::state::State::Open.as_str());
        state::add(&menu, canonrs_interactions_core::dom::state::State::Closed.as_str());
        if let Some(trigger) = query::first(&menu, "[data-rs-menubar-trigger]") {
            let _ = trigger.set_attribute("aria-expanded", "false");
        }
    }
}

pub fn init(root: Element) {
    for menu in query::all(&root, "[data-rs-menubar-menu]") {
        state::add(&menu, canonrs_interactions_core::dom::state::State::Closed.as_str());
    }
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

    // click trigger/item
    listeners::listen(&uid, &root, "click", {
        let root_c = root.clone();
        move |e: web_sys::Event| {
            let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if let Some(trigger) = target.closest("[data-rs-menubar-trigger]").ok().flatten() {
                if trigger.get_attribute("aria-disabled").as_deref() == Some("true") { return; }
                let Some(menu) = trigger.closest("[data-rs-menubar-menu]").ok().flatten() else { return };
                e.stop_propagation();
                let is_open = state::has(&menu, canonrs_interactions_core::dom::state::State::Open.as_str());
                close_all(&root_c);
                if !is_open {
                    state::remove(&menu, canonrs_interactions_core::dom::state::State::Closed.as_str());
                    state::add(&menu, canonrs_interactions_core::dom::state::State::Open.as_str());
                    let _ = trigger.set_attribute("aria-expanded", "true");
                }
            } else if target.closest("[data-rs-menubar-item]").ok().flatten().is_some() {
                if target.get_attribute("aria-disabled").as_deref() == Some("true") { return; }
                close_all(&root_c);
            }
        }
    });

    // hover
    listeners::listen(&uid, &root, "mouseover", move |e: web_sys::Event| {
        let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
        let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        if let Some(trigger) = target.closest("[data-rs-menubar-trigger]").ok().flatten() {
            state::add(&trigger, canonrs_interactions_core::dom::state::State::Hover.as_str());
        } else if let Some(item) = target.closest("[data-rs-menubar-item]").ok().flatten() {
            state::add(&item, canonrs_interactions_core::dom::state::State::Hover.as_str());
        }
    });

    listeners::listen(&uid, &root, "mouseout", move |e: web_sys::Event| {
        let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
        let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        if let Some(trigger) = target.closest("[data-rs-menubar-trigger]").ok().flatten() {
            state::remove(&trigger, canonrs_interactions_core::dom::state::State::Hover.as_str());
        } else if let Some(item) = target.closest("[data-rs-menubar-item]").ok().flatten() {
            state::remove(&item, canonrs_interactions_core::dom::state::State::Hover.as_str());
        }
    });

    // click outside — use document listener via listeners_document
    listeners::listen_document(&uid, "click", {
        let root_c = root.clone();
        move |e: web_sys::Event| {
            let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if root_c.contains(Some(&target)) { return; }
            close_all(&root_c);
        }
    });

    // keyboard
    listeners::listen(&uid, &root, "keydown", {
        let root_c = root.clone();
        move |e: web_sys::Event| {
            let e = e.dyn_into::<web_sys::KeyboardEvent>().unwrap();
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-menubar-item]").ok().flatten().is_some() {
                let Some(menu) = target.closest("[data-rs-menubar-menu]").ok().flatten() else { return };
                let items: Vec<Element> = query::all(&menu, "[data-rs-menubar-item]").into_iter()
                    .filter(|el| el.get_attribute("aria-disabled").as_deref() != Some("true"))
                    .collect();
                let len = items.len();
                let pos = items.iter().position(|el| el.contains(Some(target.as_ref())));
                match e.key().as_str() {
                    "ArrowDown" => { e.prevent_default(); let next = pos.map(|p|(p+1).min(len-1)).unwrap_or(0); if let Ok(h) = items[next].clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); } }
                    "ArrowUp"   => { e.prevent_default(); let prev = pos.map(|p|if p==0{0}else{p-1}).unwrap_or(0); if let Ok(h) = items[prev].clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); } }
                    "Enter" | " " => { e.prevent_default(); if let Ok(h) = target.clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.click(); } }
                    "Escape" => {
                        e.prevent_default();
                        state::remove(&menu, canonrs_interactions_core::dom::state::State::Open.as_str());
                        state::add(&menu, canonrs_interactions_core::dom::state::State::Closed.as_str());
                        if let Some(trigger) = query::first(&menu, "[data-rs-menubar-trigger]") {
                            let _ = trigger.set_attribute("aria-expanded", "false");
                            if let Ok(h) = trigger.dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); }
                        }
                    }
                    _ => {}
                }
                return;
            }
            if target.closest("[data-rs-menubar-trigger]").ok().flatten().is_some() {
                let triggers: Vec<Element> = query::all(&root_c, "[data-rs-menubar-trigger]").into_iter()
                    .filter(|el| el.get_attribute("aria-disabled").as_deref() != Some("true"))
                    .collect();
                let len = triggers.len();
                let pos = triggers.iter().position(|el| el.contains(Some(target.as_ref())));
                match e.key().as_str() {
                    "ArrowRight" => { e.prevent_default(); let next = pos.map(|p|(p+1)%len).unwrap_or(0); if let Ok(h) = triggers[next].clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); } }
                    "ArrowLeft"  => { e.prevent_default(); let prev = pos.map(|p|if p==0{len-1}else{p-1}).unwrap_or(0); if let Ok(h) = triggers[prev].clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); } }
                    "ArrowDown" | "Enter" | " " => {
                        e.prevent_default();
                        if let Some(trigger) = target.closest("[data-rs-menubar-trigger]").ok().flatten() {
                            if let Some(menu) = trigger.closest("[data-rs-menubar-menu]").ok().flatten() {
                                let is_open = state::has(&menu, canonrs_interactions_core::dom::state::State::Open.as_str());
                                close_all(&root_c);
                                if !is_open {
                                    state::remove(&menu, canonrs_interactions_core::dom::state::State::Closed.as_str());
                                    state::add(&menu, canonrs_interactions_core::dom::state::State::Open.as_str());
                                    let _ = trigger.set_attribute("aria-expanded", "true");
                                    if let Some(first) = query::all(&menu, "[data-rs-menubar-item]").into_iter().next() {
                                        if let Ok(h) = first.dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); }
                                    }
                                }
                            }
                        }
                    }
                    "Escape" => {
                        e.prevent_default();
                        if let Some(menu) = target.closest("[data-rs-menubar-menu]").ok().flatten() {
                            state::remove(&menu, canonrs_interactions_core::dom::state::State::Open.as_str());
                            state::add(&menu, canonrs_interactions_core::dom::state::State::Closed.as_str());
                            if let Some(trigger) = query::first(&menu, "[data-rs-menubar-trigger]") {
                                let _ = trigger.set_attribute("aria-expanded", "false");
                                if let Ok(h) = trigger.dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    });

    // Escape global
    listeners::listen_window(&uid, "keydown", {
        let root_c = root.clone();
        move |e: web_sys::Event| {
            let e = e.dyn_into::<web_sys::KeyboardEvent>().unwrap();
            if e.key() == "Escape" { close_all(&root_c); }
        }
    });
}
