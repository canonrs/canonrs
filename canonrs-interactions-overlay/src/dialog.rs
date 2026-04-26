//! Dialog Interaction Engine — Tier S
//! 100% stack-driven: sem listeners locais, ESC via top-of-stack

use wasm_bindgen::prelude::*;
use web_sys::Element;
use crate::runtime::{lifecycle, state, stack, focus, inert, portal, transition, aria};

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
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    web_sys::console::log_1(&format!("dialog::open uid={}", uid).into());

    prev_focus.set(focus::active_element());

    if let Some(p) = portal::portal_of(root, PORTAL_ATTR, &uid) {
        portal::propagate_owner(&p, &uid, CHILDREN_SEL);
        portal::move_to_body(&p, &uid);
    }

    state::open(root);

    // push no stack — gerencia z-index e scroll_lock
    stack::push(&uid, KIND);
    state::set_scroll_lock(true);
    stack::apply_z(
        &uid,
        &format!("[{}][data-rs-owner='{}']", OVERLAY_ATTR, uid),
        &format!("[{}][data-rs-owner='{}']", CONTENT_ATTR, uid),
    );

    let (overlay, content) = portal::portal_nodes(&uid, OVERLAY_ATTR, CONTENT_ATTR);

    transition::set_state_nodes(&overlay, &content, "entering");
    {
        let o2 = overlay.clone();
        let c2 = content.clone();
        let cb = Closure::once(move || transition::set_state_nodes(&o2, &c2, "open"));
        let _ = web_sys::window().unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), 16);
        cb.forget();
    }

    // foco ANTES do inert
    if let Some(ref c) = content { focus::focus_first(c); }

    {
        let uid2 = uid.clone();
        let cb = Closure::once(move || inert::set_inert_background(true, &uid2, PORTAL_ATTR));
        let _ = web_sys::window().unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), 32);
        cb.forget();
    }
}

fn close(root: &Element, prev_focus: &std::rc::Rc<std::cell::Cell<Option<Element>>>) {
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    web_sys::console::log_1(&format!("dialog::close uid={}", uid).into());

    let (overlay, content) = portal::portal_nodes(&uid, OVERLAY_ATTR, CONTENT_ATTR);
    let duration = transition::duration_ms(root, CSS_VAR);

    // remove inert imediatamente
    inert::set_inert_background(false, &uid, PORTAL_ATTR);

    transition::set_state_nodes(&overlay, &content, "exiting");
    state::close(root);

    // pop do stack — libera scroll_lock se vazio
    stack::pop(&uid);
    stack::unregister(&uid);

    {
        let o2 = overlay.clone();
        let c2 = content.clone();
        let pf = prev_focus.clone();
        let cb = Closure::once(move || {
            transition::set_state_nodes(&o2, &c2, "closed");
            if let Some(el) = pf.take() {
                if let Ok(html) = el.dyn_into::<web_sys::HtmlElement>() {
                    let _ = html.focus();
                }
            }
        });
        let _ = web_sys::window().unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(), (duration + 16) as i32
            );
        cb.forget();
    }
}

#[wasm_bindgen(js_name = dialog_open)]
pub fn dialog_open(uid: &str) {
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
    if let Ok(Some(root)) = doc.query_selector(&format!(
        "[data-rs-dialog][data-rs-uid='{}']", uid
    )) {
        let pf = std::rc::Rc::new(std::cell::Cell::new(focus::active_element()));
        open(&root, &pf);
    }
}

#[wasm_bindgen(js_name = dialog_close)]
pub fn dialog_close(uid: &str) {
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
    if let Ok(Some(root)) = doc.query_selector(&format!(
        "[data-rs-dialog][data-rs-uid='{}']", uid
    )) {
        let pf = std::rc::Rc::new(std::cell::Cell::new(None::<Element>));
        close(&root, &pf);
    }
}

pub fn init(root: Element) {
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    web_sys::console::log_1(&format!("dialog::init uid={}", uid).into());

    if !lifecycle::init_guard(&root) {
        web_sys::console::log_1(&"dialog::init SKIPPED".into());
        return;
    }
    web_sys::console::log_1(&"dialog::init RUNNING".into());

    // garante 1 listener global para todos os overlays
    stack::ensure_global_listeners();

    if let Some(p) = portal::portal_of(&root, PORTAL_ATTR, &uid) {
        portal::propagate_owner(&p, &uid, CHILDREN_SEL);
        portal::move_to_body(&p, &uid);
        let (overlay, content) = portal::portal_nodes(&uid, OVERLAY_ATTR, CONTENT_ATTR);
        transition::set_state_nodes(&overlay, &content, "closed");
        aria::link_aria_from_portal(&uid, &p, CONTENT_ATTR, TITLE_ATTR, DESC_ATTR);
    }

    let prev_focus = std::rc::Rc::new(std::cell::Cell::new(None::<Element>));

    // click — via stack global (não listener local)
    {
        let root_cb = root.clone();
        let uid2    = uid.clone();
        let pf      = prev_focus.clone();
        stack::register_click(&uid, move |target| {
            // trigger: dentro do root OU data-rs-target aponta para este uid
            if let Some(trigger) = target.closest(&format!("[{}]", TRIGGER_ATTR)).ok().flatten() {
                let in_root     = root_cb.contains(Some(&trigger));
                let targets_uid = trigger.get_attribute("data-rs-target").as_deref() == Some(&uid2);
                if in_root || targets_uid {
                    open(&root_cb, &pf);
                    return;
                }
            }
            if !state::is_open(&root_cb) { return; }

            // overlay fecha
            if target.closest(&format!("[{}]", OVERLAY_ATTR)).ok().flatten().is_some() {
                close(&root_cb, &pf);
                return;
            }

            // close button: verifica owner
            let owner = target.closest("[data-rs-owner]").ok().flatten()
                .and_then(|el| el.get_attribute("data-rs-owner"))
                .or_else(|| target.get_attribute("data-rs-owner"));
            if owner.as_deref() != Some(&uid2) { return; }

            if target.closest(&format!("[{}]", CLOSE_ATTR)).ok().flatten().is_some() {
                close(&root_cb, &pf);
            }
        });
    }

    // keydown — via stack global, ESC fecha apenas top-of-stack
    {
        let root_cb = root.clone();
        let uid2    = uid.clone();
        let pf      = prev_focus.clone();
        stack::register_keydown(&uid, move |e| {
            if !state::is_open(&root_cb) { return; }
            // ESC: apenas se este dialog for o top do stack
            if e.key() == "Escape" && stack::is_top(&uid2) {
                e.prevent_default();
                close(&root_cb, &pf);
                return;
            }
            // Tab trap
            if e.key() == "Tab" {
                let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
                let sel = format!("[{}][data-rs-owner='{}']", CONTENT_ATTR, uid2);
                let Some(content) = doc.query_selector(&sel).ok().flatten() else { return };
                focus::trap_tab(e, &content);
            }
        });
    }
}
