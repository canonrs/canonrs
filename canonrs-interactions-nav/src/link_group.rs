//! LinkGroup Interaction Engine
//! Core: dom/{state, query} + behavior/selection::{activate, init_state}

use wasm_bindgen::JsCast;
use canonrs_interactions_core::dom::{state, query};
use canonrs_interactions_core::runtime::listeners;
use canonrs_interactions_core::behavior::selection::{SelectionConfig, activate, init_state};
use web_sys::Element;

const ITEM_SEL: &str = "[data-rs-nav-item]";

fn config() -> SelectionConfig {
    SelectionConfig {
        item_selector: ITEM_SEL,
        value_attr:    "data-rs-value",
        aria_selected: false,
        aria_current:  true,
    }
}

pub fn init(root: Element) {
    init_state(&root, &config());
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

    listeners::listen(&uid, &root, "click", {
        let root_c = root.clone();
        move |e: web_sys::Event| {
            let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(item) = target.closest(ITEM_SEL).ok().flatten() else { return };
            if state::has(&item, canonrs_interactions_core::dom::state::State::Disabled.as_str()) { return; }
            activate(&root_c, &item, &config());
        }
    });

    listeners::listen(&uid, &root, "keydown", {
        let root_c = root.clone();
        move |e: web_sys::Event| {
            let e = e.dyn_into::<web_sys::KeyboardEvent>().unwrap();
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest(ITEM_SEL).ok().flatten().is_none() { return; }
            let items: Vec<Element> = query::all(&root_c, ITEM_SEL).into_iter()
                .filter(|el| !state::has(el, canonrs_interactions_core::dom::state::State::Disabled.as_str()))
                .collect();
            let len = items.len();
            if len == 0 { return; }
            let pos = items.iter().position(|el| el.contains(Some(target.as_ref())));
            match e.key().as_str() {
                "ArrowDown" | "ArrowRight" => { e.prevent_default(); let next = pos.map(|p|(p+1).min(len-1)).unwrap_or(0); if let Ok(h) = items[next].clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); } }
                "ArrowUp"   | "ArrowLeft"  => { e.prevent_default(); let prev = pos.map(|p|if p==0{0}else{p-1}).unwrap_or(0); if let Ok(h) = items[prev].clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); } }
                "Home" => { e.prevent_default(); if let Ok(h) = items[0].clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); } }
                "End"  => { e.prevent_default(); if let Ok(h) = items[len-1].clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); } }
                _ => {}
            }
        }
    });
}