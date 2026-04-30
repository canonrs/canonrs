//! ConfirmDialog Interaction Engine — Tier S
//! Usa runtime centralizado: stack, focus, inert, portal, transition, aria

use wasm_bindgen::prelude::*;
use web_sys::Element;
use crate::runtime::{lifecycle, state, stack, focus, inert, portal, transition, aria, query};

const KIND:         &str = "confirm-dialog";
const PORTAL_ATTR:  &str = "data-rs-confirm-dialog-portal";
const OVERLAY_ATTR: &str = "data-rs-confirm-dialog-overlay";
const CONTENT_ATTR: &str = "data-rs-confirm-dialog-content";
const TITLE_ATTR:   &str = "data-rs-confirm-dialog-title";
const DESC_ATTR:    &str = "data-rs-confirm-dialog-description";
const CANCEL_ATTR:  &str = "data-rs-confirm-dialog-cancel";
const TRIGGER_ATTR: &str = "data-rs-confirm-dialog-trigger";
const CSS_VAR:      &str = "--confirm-dialog-transition-duration";
const CHILDREN_SEL: &str = "[data-rs-confirm-dialog-overlay], [data-rs-confirm-dialog-content]";

// ---------------------------------------------------------------------------
// Open / Close
// ---------------------------------------------------------------------------

fn open(root: &Element, prev_focus: &std::rc::Rc<std::cell::Cell<Option<Element>>>) {
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

    prev_focus.set(focus::active_element());

    // re-propaga owner sempre (garante owner mesmo se portal foi remontado)
    if let Some(p) = portal::portal_of(root, PORTAL_ATTR, &uid) {
        portal::propagate_owner(&p, &uid, CHILDREN_SEL);
        // move para body se necessário
        portal::move_to_body(&p, &uid);
    }

    state::open(root);
    stack::push(&uid, KIND);
    stack::apply_z(
        &uid,
        &format!("[{}][data-rs-owner='{}']", OVERLAY_ATTR, uid),
        &format!("[{}][data-rs-owner='{}']", CONTENT_ATTR, uid),
    );

    let (overlay, content) = portal::portal_nodes(&uid, OVERLAY_ATTR, CONTENT_ATTR);
    web_sys::console::log_1(&format!("confirm_dialog::open overlay={} content={}",
        overlay.is_some(), content.is_some()).into());

    // injeta data-rs-root no content para lookup direto sem query global
    if let Some(ref c) = content {
        let _ = c.set_attribute("data-rs-root", &uid);
    }

    transition::set_state_nodes(&overlay, &content, "entering");
    {
        let o2 = overlay.clone();
        let c2 = content.clone();
        let cb = Closure::once(move || transition::set_state_nodes(&o2, &c2, "open"));
        let _ = web_sys::window().unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), 16);
        cb.forget();
    }

    state::set_scroll_lock(true);

    // foca ANTES do inert
    if let Some(ref c) = content { focus::focus_first(c); }

    // inert após foco estar dentro do dialog
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

    let (overlay, content) = portal::portal_nodes(&uid, OVERLAY_ATTR, CONTENT_ATTR);
    let duration = transition::duration_ms(root, CSS_VAR);

    // remove inert imediatamente
    inert::set_inert_background(false, &uid, PORTAL_ATTR);

    transition::set_state_nodes(&overlay, &content, "exiting");
    state::close(root);
    stack::pop(&uid);

    {
        let o2 = overlay.clone();
        let c2 = content.clone();
        let pf = prev_focus.clone();
        let cb = Closure::once(move || {
            transition::set_state_nodes(&o2, &c2, "closed");
            // scroll_lock liberado pelo stack::pop se vazio
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

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

#[wasm_bindgen(js_name = confirm_dialog_open)]
pub fn confirm_dialog_open(uid: &str) {
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
    if let Ok(Some(root)) = doc.query_selector(&format!(
        "[data-rs-confirm-dialog][data-rs-uid='{}']", uid
    )) {
        let pf = std::rc::Rc::new(std::cell::Cell::new(focus::active_element()));
        open(&root, &pf);
    }
}

#[wasm_bindgen(js_name = confirm_dialog_close)]
pub fn confirm_dialog_close(uid: &str) {
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
    if let Ok(Some(root)) = doc.query_selector(&format!(
        "[data-rs-confirm-dialog][data-rs-uid='{}']", uid
    )) {
        let pf = std::rc::Rc::new(std::cell::Cell::new(None::<Element>));
        close(&root, &pf);
    }
}

// ---------------------------------------------------------------------------
// Init
// ---------------------------------------------------------------------------

pub fn init(root: Element) {
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

    if !lifecycle::init_guard(&root) {
        return;
    }

    // garante listeners globais (1 vez para todos os overlays)
    stack::ensure_global_listeners();

    // portal → body + owner
    if let Some(p) = portal::portal_of(&root, PORTAL_ATTR, &uid) {
        portal::propagate_owner(&p, &uid, CHILDREN_SEL);
        portal::move_to_body(&p, &uid);
        let (overlay, content) = portal::portal_nodes(&uid, OVERLAY_ATTR, CONTENT_ATTR);
        transition::set_state_nodes(&overlay, &content, "closed");

        // aria
        aria::link_aria_from_portal(&uid, &p, CONTENT_ATTR, TITLE_ATTR, DESC_ATTR);
    }

    let prev_focus = std::rc::Rc::new(std::cell::Cell::new(None::<Element>));

    // escuta evento rs:confirm-dialog:close para fechar programaticamente
    {
        let root_live = root.clone();
        let pf_close = std::rc::Rc::new(std::cell::Cell::new(None::<Element>));
        let cb = Closure::<dyn Fn(web_sys::Event)>::new(move |_: web_sys::Event| {
            close(&root_live, &pf_close);
        });
        let _ = root.add_event_listener_with_callback("rs:confirm-dialog:close", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // registra click handler no stack global
    {
        let uid2 = uid.clone();
        let pf   = prev_focus.clone();
        stack::register_click(&uid, move |target| {
            let Some(root_live) = query::root_of("data-rs-confirm-dialog", &uid2) else { return };
            if let Some(trigger) = target.closest(&format!("[{}]", TRIGGER_ATTR)).ok().flatten() {
                let in_root     = root_live.contains(Some(&trigger as &web_sys::Element));
                let targets_uid = trigger.get_attribute("data-rs-target").as_deref() == Some(&uid2);
                if in_root || targets_uid {
                    // propaga contexto do trigger para o root — DOM como fonte de verdade
                    if let Some(v) = trigger.get_attribute("data-rs-value") {
                        let _ = root_live.set_attribute("data-rs-current-value", &v);
                    }
                    if let Some(l) = trigger.get_attribute("data-rs-label") {
                        let _ = root_live.set_attribute("data-rs-current-label", &l);
                    }
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
            if target.closest(&format!("[{}]", CANCEL_ATTR)).ok().flatten().is_some() {
                close(&root_live, &pf);
            }
        });
    }

    // registra keydown handler no stack global
    {
        let uid2 = uid.clone();
        let pf   = prev_focus.clone();
        stack::register_keydown(&uid, move |e| {
            let Some(root_live) = query::root_of("data-rs-confirm-dialog", &uid2) else { return };
            if !state::is_open(&root_live) { return; }
            if e.key() == "Escape" && stack::is_top(&uid2) {
                e.prevent_default();
                close(&root_live, &pf);
                return;
            }
            if e.key() == "Tab" {
                let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
                let sel = format!("[{}][data-rs-owner='{}']", CONTENT_ATTR, uid2);
                let Some(content) = doc.query_selector(&sel).ok().flatten() else { return };
                focus::trap_tab(e, &content);
            }
        });
    }
}
