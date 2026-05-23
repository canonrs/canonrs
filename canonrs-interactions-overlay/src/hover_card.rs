//! HoverCard Interaction Engine
//! Core: dom/{lifecycle, state} + Overlay: stack

use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::{lifecycle, state};
use canonrs_interactions_core::runtime::{listeners, timers};

fn is_leaving_root(e: &web_sys::PointerEvent, root: &Element) -> bool {
    let related = e.related_target().and_then(|t| t.dyn_into::<Element>().ok());
    match related {
        Some(rel) => !root.contains(Some(rel.as_ref())),
        None => true,
    }
}

fn open_card(root: &Element) {
    if crate::runtime::stack::has_modal_open() { return; }
    let Some(c) = root.query_selector("[data-rs-hover-card-content]").ok().flatten() else { return };
    state::open(&c);
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    let Ok(Some(_)) = root.query_selector("[data-rs-hover-card-trigger]") else { return };
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

    {
        let root_cb = root.clone();
        listeners::listen(&uid, &root, "pointerenter", move |_: web_sys::Event| {
            let root2 = root_cb.clone();
            timers::timeout(120, move || { open_card(&root2); });
        });
    }
    {
        let root_cb = root.clone();
        listeners::listen(&uid, &root, "pointerleave", move |e: web_sys::Event| {
            let Some(pe) = e.dyn_into::<web_sys::PointerEvent>().ok() else { return };
            let root2 = root_cb.clone();
            if !is_leaving_root(&pe, &root2) { return; }
            timers::timeout(80, move || {
                let Some(c) = root2.query_selector("[data-rs-hover-card-content]").ok().flatten() else { return };
                if !state::is_open(&c) { return; }
                state::close(&c);
            });
        });
    }
    {
        let root_cb = root.clone();
        listeners::listen(&uid, &root, "focusin", move |_: web_sys::Event| {
            open_card(&root_cb);
        });
    }
    {
        let root_cb = root.clone();
        listeners::listen(&uid, &root, "focusout", move |_: web_sys::Event| {
            let Some(c) = root_cb.query_selector("[data-rs-hover-card-content]").ok().flatten() else { return };
            state::close(&c);
        });
    }
}
