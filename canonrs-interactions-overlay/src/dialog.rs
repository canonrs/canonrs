//! Dialog Interaction Engine — Tier S
//! Core: dom/{state, query} + integration/aria
//! Overlay: stack, focus, inert, portal, transition

use web_sys::Element;
use canonrs_interactions_core::dom::{state, query};
use canonrs_interactions_core::integration::aria;
use canonrs_interactions_core::runtime::{listeners, timers};
use wasm_bindgen::prelude::*;
use crate::runtime::{stack, focus, inert, portal, transition};

const KIND:         &str = "dialog";
const PORTAL_ATTR:  &str = "data-rs-dialog-portal";
const OVERLAY_ATTR: &str = "data-rs-dialog-overlay";
const CONTENT_ATTR: &str = "data-rs-dialog-content";
const TITLE_ATTR:   &str = "data-rs-dialog-title";
const DESC_ATTR:    &str = "data-rs-dialog-description";
const CLOSE_ATTR:   &str = "data-rs-dialog-close";
const TRIGGER_ATTR: &str = "data-rs-dialog-trigger";
const CSS_VAR:      &str = "--dialog-transition-duration";
const CHILDREN_SEL: &str = "[data-rs-dialog-overlay], [data-rs-dialog-content]";

fn open(root: &Element, prev_focus: &std::rc::Rc<std::cell::Cell<Option<Element>>>) {
    if root.has_attribute("data-rs-just-closed") { return; }
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

    prev_focus.set(focus::active_element());

    if let Some(p) = portal::portal_of(root, PORTAL_ATTR, &uid) {
        portal::propagate_owner(&p, &uid, CHILDREN_SEL);
        portal::move_to_body(&p, &uid);
    }

    state::open(root);

    stack::push(&uid, KIND);
    state::set_scroll_lock(true);
    stack::apply_z(
        &uid,
        &format!("[{}][data-rs-owner=\'{}\']", OVERLAY_ATTR, uid),
        &format!("[{}][data-rs-owner=\'{}\']", CONTENT_ATTR, uid),
    );

    let (overlay, content) = portal::portal_nodes(&uid, OVERLAY_ATTR, CONTENT_ATTR);

    transition::set_state_nodes(&overlay, &content, "entering");
    {
        let o2 = overlay.clone();
        let c2 = content.clone();
        timers::next_frame(move || transition::set_state_nodes(&o2, &c2, "open"));
    }

    if let Some(ref c) = content { focus::focus_first(c); }

    {
        let uid2 = uid.clone();
        timers::after_transition(move || inert::set_inert_background(true, &uid2, PORTAL_ATTR));
    }
}

fn close(root: &Element, prev_focus: &std::rc::Rc<std::cell::Cell<Option<Element>>>) {
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    let _ = root.set_attribute("data-rs-just-closed", "true");

    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        let sel = format!("[data-rs-dialog-content][data-rs-owner=\'{}\'] [data-rs-input]", uid);
        if let Ok(list) = doc.query_selector_all(&sel) {
            use wasm_bindgen::JsCast;
            for i in 0..list.length() {
                if let Some(el) = list.item(i).and_then(|n| n.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                    el.set_value("");
                }
            }
        }
    }

    {
        let root_jc = root.clone();
        timers::raf(move || { let _ = root_jc.remove_attribute("data-rs-just-closed"); });
    }

    let (overlay, content) = portal::portal_nodes(&uid, OVERLAY_ATTR, CONTENT_ATTR);
    let duration = transition::duration_ms(root, CSS_VAR);

    inert::set_inert_background(false, &uid, PORTAL_ATTR);
    transition::set_state_nodes(&overlay, &content, "exiting");
    state::close(root);
    stack::pop(&uid);

    {
        let o2 = overlay.clone();
        let c2 = content.clone();
        let pf = prev_focus.clone();
        timers::after_duration(duration as i32, move || {
            transition::set_state_nodes(&o2, &c2, "closed");
            if let Some(el) = pf.take() {
                if let Ok(html) = el.dyn_into::<web_sys::HtmlElement>() {
                    let _ = html.focus();
                }
            }
        });
    }
}

#[wasm_bindgen(js_name = dialog_open)]
pub fn dialog_open(uid: &str) {
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
    if let Ok(Some(root)) = doc.query_selector(&format!(
        "[data-rs-dialog][data-rs-uid=\'{}\']", uid
    )) {
        let pf = std::rc::Rc::new(std::cell::Cell::new(focus::active_element()));
        open(&root, &pf);
    }
}

#[wasm_bindgen(js_name = dialog_close)]
pub fn dialog_close(uid: &str) {
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
    if let Ok(Some(root)) = doc.query_selector(&format!(
        "[data-rs-dialog][data-rs-uid=\'{}\']", uid
    )) {
        let pf = std::rc::Rc::new(std::cell::Cell::new(None::<Element>));
        close(&root, &pf);
    }
}

pub fn init(root: Element) {
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();


    stack::ensure_global_listeners();

    if let Some(p) = portal::portal_of(&root, PORTAL_ATTR, &uid) {
        portal::propagate_owner(&p, &uid, CHILDREN_SEL);
        portal::move_to_body(&p, &uid);
        let (overlay, content) = portal::portal_nodes(&uid, OVERLAY_ATTR, CONTENT_ATTR);
        transition::set_state_nodes(&overlay, &content, "closed");
        aria::link_aria_from_portal(&uid, &p, CONTENT_ATTR, TITLE_ATTR, DESC_ATTR);
    }

    let prev_focus = std::rc::Rc::new(std::cell::Cell::new(None::<Element>));

    // rs:dialog:close — programmatic close via custom event
    {
        let root_cb = root.clone();
        let pf_close = std::rc::Rc::new(std::cell::Cell::new(None::<Element>));
        listeners::listen(&uid, &root, "rs:dialog:close", move |e: web_sys::Event| {
            let root_live = e.current_target()
                .and_then(|t| { use wasm_bindgen::JsCast; t.dyn_into::<web_sys::Element>().ok() })
                .unwrap_or_else(|| root_cb.clone());
            close(&root_live, &pf_close);
        });
    }

    // click — via stack global
    {
        let uid2 = uid.clone();
        let pf   = prev_focus.clone();
        stack::register_click(&uid, move |target| {
            if !target.is_connected() { return; }
            let Some(root_live) = query::root_of("data-rs-dialog", &uid2) else { return };
            if let Some(trigger) = target.closest(&format!("[{}]", TRIGGER_ATTR)).ok().flatten() {
                let in_root     = root_live.contains(Some(&trigger as &web_sys::Element));
                let targets_uid = trigger.get_attribute("data-rs-target").as_deref() == Some(&uid2);
                if in_root || targets_uid {
                    if root_live.has_attribute("data-rs-just-closed") { return; }
                    open(&root_live, &pf);
                    return;
                }
            }
            if !state::is_open(&root_live) { return; }
            if target.closest(&format!("[{}]", OVERLAY_ATTR)).ok().flatten().is_some() {
                close(&root_live, &pf);
                return;
            }
            let owner = target.closest("[data-rs-owner]").ok().flatten()
                .and_then(|el| el.get_attribute("data-rs-owner"))
                .or_else(|| target.get_attribute("data-rs-owner"));
            if owner.as_deref() != Some(&uid2) { return; }
            if target.closest(&format!("[{}]", CLOSE_ATTR)).ok().flatten().is_some() {
                close(&root_live, &pf);
            }
        });
    }

    // keydown — via stack global, ESC fecha apenas top-of-stack
    {
        let uid2 = uid.clone();
        let pf   = prev_focus.clone();
        stack::register_keydown(&uid, move |e| {
            let Some(root_live) = query::root_of("data-rs-dialog", &uid2) else { return };
            if !state::is_open(&root_live) { return; }
            let key = e.key();
            if key.is_empty() { return; }
            if key == "Escape" && stack::is_top(&uid2) {
                e.prevent_default();
                close(&root_live, &pf);
                return;
            }
            if key == "Tab" {
                let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
                let sel = format!("[{}][data-rs-owner=\'{}\']", CONTENT_ATTR, uid2);
                let Some(content) = doc.query_selector(&sel).ok().flatten() else { return };
                focus::trap_tab(e, &content);
            }
        });
    }
}
