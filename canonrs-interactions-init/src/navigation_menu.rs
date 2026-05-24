//! NavigationMenu Init

use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::{state, query};
use canonrs_interactions_core::runtime::listeners;
use crate::runtime::{interactive, keyboard};

fn close_all(root: &Element) {
    for item in query::all(root, "[data-rs-navigation-menu-item]") {
        state::close(&item);
        if let Some(content) = query::first(&item, "[data-rs-navigation-menu-content]") {
            let _ = content.set_attribute("aria-hidden", "true");
        }
    }
}

fn open_item(item: &Element) {
    state::open(item);
    if let Some(content) = query::first(item, "[data-rs-navigation-menu-content]") {
        let _ = content.set_attribute("aria-hidden", "false");
    }
}

pub fn init(root: Element) {
    for el in query::all(&root, "[data-rs-navigation-menu-trigger]") { interactive::init(&el); }
    for el in query::all(&root, "[data-rs-navigation-menu-link]")    { interactive::init(&el); }
    close_all(&root);

    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

    listeners::listen(&uid, &root, "click", {
        let root_c = root.clone();
        move |e: web_sys::Event| {
            let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-navigation-menu-trigger]").ok().flatten().is_none() { return; }
            let Some(item) = target.closest("[data-rs-navigation-menu-item]").ok().flatten() else { return };
            let is_open = state::is_open(&item);
            close_all(&root_c);
            if !is_open { open_item(&item); }
        }
    });

    let triggers_idx = keyboard::init_nav(
        &root,
        "[data-rs-navigation-menu-trigger]",
        keyboard::NavConfig {
            orientation: keyboard::Orientation::Horizontal,
            element_type: keyboard::ElementType::Button,
            focus_state: "focused",
            wrap: false,
        },
        Some(Box::new(move |idx, items| {
            if let Some(trigger) = items.get(idx) {
                if let Some(item) = trigger.closest("[data-rs-navigation-menu-item]").ok().flatten() {
                    if !state::is_open(&item) { open_item(&item); }
                    let links = query::all(&item, "[data-rs-navigation-menu-link]");
                    if let Some(first) = links.into_iter().next() {
                        if let Ok(el) = first.dyn_into::<web_sys::HtmlElement>() { let _ = el.focus(); }
                    }
                }
            }
        })),
        None,
    );

    listeners::listen(&uid, &root, "click", {
        let root_c = root.clone();
        let idx_sync = triggers_idx.clone();
        move |e: web_sys::Event| {
            let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(trigger) = target.closest("[data-rs-navigation-menu-trigger]").ok().flatten() else { return };
            let triggers = query::all(&root_c, "[data-rs-navigation-menu-trigger]");
            if let Some(idx) = triggers.iter().position(|el| el.contains(Some(trigger.as_ref()))) {
                idx_sync.set(Some(idx));
            }
        }
    });

    listeners::listen(&uid, &root, "keydown", {
        move |e: web_sys::Event| {
            let e = e.dyn_into::<web_sys::KeyboardEvent>().unwrap();
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-navigation-menu-content]").ok().flatten().is_none() { return; }
            let Some(item) = target.closest("[data-rs-navigation-menu-item]").ok().flatten() else { return };
            let links: Vec<Element> = query::all(&item, "[data-rs-navigation-menu-link]");
            let current = links.iter().position(|el| el.contains(Some(target.as_ref())));
            match e.key().as_str() {
                "ArrowDown" => { e.prevent_default(); let next = current.map(|i|(i+1).min(links.len()-1)).unwrap_or(0); if let Ok(el) = links[next].clone().dyn_into::<web_sys::HtmlElement>() { let _ = el.focus(); } }
                "ArrowUp"   => { e.prevent_default(); let prev = current.map(|i|if i==0{0}else{i-1}).unwrap_or(0); if let Ok(el) = links[prev].clone().dyn_into::<web_sys::HtmlElement>() { let _ = el.focus(); } }
                "Escape" => {
                    e.prevent_default(); e.stop_propagation();
                    if let Some(trigger) = query::first(&item, "[data-rs-navigation-menu-trigger]") {
                        if let Ok(el) = trigger.clone().dyn_into::<web_sys::HtmlElement>() { let _ = el.focus(); }
                    }
                    state::close(&item);
                }
                _ => {}
            }
        }
    });

    listeners::listen_document(&uid, "click", {
        let root_c = root.clone();
        move |e: web_sys::Event| {
            let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if root_c.contains(Some(&target)) { return; }
            close_all(&root_c);
        }
    });
}
