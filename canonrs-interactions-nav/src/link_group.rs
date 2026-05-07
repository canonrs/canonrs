//! LinkGroup Interaction Engine
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use canonrs_interactions_core::dom::{lifecycle, state, query};
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
    if !lifecycle::init_guard(&root) { return; }

    init_state(&root, &config());

    // click
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(item) = target.closest(ITEM_SEL).ok().flatten() else { return };
            if state::has(&item, "disabled") { return; }
            activate(&root_cb, &item, &config());
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // keyboard
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest(ITEM_SEL).ok().flatten().is_none() { return; }
            let items: Vec<Element> = query::all(&root_cb, ITEM_SEL)
                .into_iter()
                .filter(|el| !state::has(el, "disabled"))
                .collect();
            let len = items.len();
            if len == 0 { return; }
            let pos = items.iter().position(|el| el.contains(Some(target.as_ref())));
            match e.key().as_str() {
                "ArrowDown" | "ArrowRight" => {
                    e.prevent_default();
                    let next = pos.map(|p| (p + 1).min(len - 1)).unwrap_or(0);
                    if let Ok(h) = items[next].clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); }
                }
                "ArrowUp" | "ArrowLeft" => {
                    e.prevent_default();
                    let prev = pos.map(|p| if p == 0 { 0 } else { p - 1 }).unwrap_or(0);
                    if let Ok(h) = items[prev].clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); }
                }
                "Home" => {
                    e.prevent_default();
                    if let Ok(h) = items[0].clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); }
                }
                "End" => {
                    e.prevent_default();
                    if let Ok(h) = items[len - 1].clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); }
                }
                _ => {}
            }
        }));
        let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}
