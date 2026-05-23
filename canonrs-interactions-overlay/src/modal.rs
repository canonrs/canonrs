//! Modal Interaction Engine
//! Core: dom/{lifecycle, state, query}

use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::{lifecycle, state, query};
use canonrs_interactions_core::runtime::listeners;

fn move_to_body(root: &Element) {
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    if uid.is_empty() { return; }
    let Some(portal) = root.query_selector("[data-rs-modal-portal]").ok().flatten() else { return };
    let Some(body) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.body()) else { return };
    if portal.parent_element().map(|p| p.tag_name() == "BODY").unwrap_or(false) { return; }
    let _ = body.append_child(&portal);
    if let Ok(nodes) = portal.query_selector_all("[data-rs-modal-overlay], [data-rs-modal-content]") {
        for i in 0..nodes.length() {
            if let Some(n) = nodes.item(i).and_then(|n| n.dyn_into::<Element>().ok()) {
                let _ = n.set_attribute("data-rs-owner", &uid);
            }
        }
    }
    let _ = portal.set_attribute("data-rs-owner", &uid);
}

fn sync_state(root: &Element, s: &str) {
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
    let sel = format!(
        "[data-rs-modal-portal][data-rs-owner='{uid}'] [data-rs-modal-overlay],          [data-rs-modal-portal][data-rs-owner='{uid}'] [data-rs-modal-content],          [data-rs-modal-overlay][data-rs-owner='{uid}'],          [data-rs-modal-content][data-rs-owner='{uid}']"
    );
    if let Ok(nodes) = doc.query_selector_all(&sel) {
        for i in 0..nodes.length() {
            if let Some(n) = nodes.item(i).and_then(|n| n.dyn_into::<Element>().ok()) {
                if s == "open" { state::open(&n); } else { state::close(&n); }
            }
        }
    }
}

fn open(root: &Element)  { state::open(root);  sync_state(root, "open");   state::set_scroll_lock(true); }
fn close(root: &Element) { state::close(root); sync_state(root, "closed"); state::set_scroll_lock(false); }

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    move_to_body(&root);

    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

    // click no root — trigger abre
    {
        let uid2 = uid.clone();
        listeners::listen(&uid, &root, "click", move |e: web_sys::Event| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if query::closest(&target, "[data-rs-modal-trigger]") {
                let Some(root_live) = query::root_of("data-rs-modal", &uid2) else { return };
                open(&root_live);
            }
        });
    }

    // click no document — overlay/close fecha
    {
        let uid2 = uid.clone();
        listeners::listen_document(&uid, "click", move |e: web_sys::Event| {
            let Some(root_live) = query::root_of("data-rs-modal", &uid2) else { return };
            if !state::is_open(&root_live) { return; }
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let owner = target.get_attribute("data-rs-owner")
                .or_else(|| target.closest("[data-rs-owner]").ok().flatten()
                    .and_then(|el| el.get_attribute("data-rs-owner")));
            if owner.as_deref() != Some(&uid2) { return; }
            if query::closest(&target, "[data-rs-modal-overlay]") { close(&root_live); return; }
            if query::closest(&target, "[data-rs-modal-close]")   { close(&root_live); }
        });
    }

    // keydown na window — ESC fecha
    {
        let uid2 = uid.clone();
        listeners::listen_window(&uid, "keydown", move |e: web_sys::Event| {
            let Some(ke) = e.dyn_into::<web_sys::KeyboardEvent>().ok() else { return };
            if ke.key() != "Escape" { return; }
            let Some(root_live) = query::root_of("data-rs-modal", &uid2) else { return };
            if state::is_open(&root_live) { close(&root_live); }
        });
    }
}
