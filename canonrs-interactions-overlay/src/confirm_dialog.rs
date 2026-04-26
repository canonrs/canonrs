//! ConfirmDialog Interaction Engine — Tier S
//! Focus trap, aria, scroll lock, inert, animation lifecycle, portal-safe

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state, query};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn doc() -> Option<web_sys::Document> {
    web_sys::window().and_then(|w| w.document())
}

fn portal_of(root: &Element) -> Option<Element> {
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    root.query_selector("[data-rs-confirm-dialog-portal]").ok().flatten()
        .or_else(|| doc()?.query_selector(&format!(
            "[data-rs-confirm-dialog-portal][data-rs-owner='{}']", uid
        )).ok().flatten())
        .or_else(|| doc()?.query_selector("[data-rs-confirm-dialog-portal]").ok().flatten())
}

fn propagate_owner(portal: &Element, uid: &str) {
    let _ = portal.set_attribute("data-rs-owner", uid);
    let sel = "[data-rs-confirm-dialog-overlay], [data-rs-confirm-dialog-content]";
    if let Ok(nodes) = portal.query_selector_all(sel) {
        for i in 0..nodes.length() {
            if let Some(n) = nodes.item(i).and_then(|n| n.dyn_into::<Element>().ok()) {
                let _ = n.set_attribute("data-rs-owner", uid);
            }
        }
    }
}

fn portal_nodes(root: &Element) -> (Option<Element>, Option<Element>) {
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    let Some(doc) = doc() else { return (None, None) };
    let overlay = doc.query_selector(&format!(
        "[data-rs-confirm-dialog-overlay][data-rs-owner='{}']", uid
    )).ok().flatten();
    let content = doc.query_selector(&format!(
        "[data-rs-confirm-dialog-content][data-rs-owner='{}']", uid
    )).ok().flatten();
    (overlay, content)
}

// ---------------------------------------------------------------------------
// Animation lifecycle
// ---------------------------------------------------------------------------

fn transition_duration_ms(root: &Element) -> i32 {
    // lê a CSS var --confirm-dialog-transition-duration e converte para ms
    // fallback: 200ms
    if let Some(win) = web_sys::window() {
        if let Ok(style) = win.get_computed_style(root) {
            if let Some(style) = style {
                let val = style.get_property_value("--confirm-dialog-transition-duration")
                    .unwrap_or_default();
                let val = val.trim();
                if val.ends_with("ms") {
                    if let Ok(n) = val.trim_end_matches("ms").trim().parse::<u32>() {
                        return n as i32;
                    }
                } else if val.ends_with('s') {
                    if let Ok(n) = val.trim_end_matches('s').trim().parse::<f32>() {
                        return (n * 1000.0) as i32;
                    }
                }
            }
        }
    }
    200_i32
}

fn set_state_nodes(overlay: &Option<Element>, content: &Option<Element>, s: &str) {
    for el in [overlay, content].iter().filter_map(|e| e.as_ref()) {
        if s == "open" || s == "entering" {
            state::remove_state(el, "closed");
            state::remove_state(el, "exiting");
            state::add_state(el, s);
        } else if s == "exiting" {
            state::remove_state(el, "open");
            state::remove_state(el, "entering");
            state::add_state(el, "exiting");
        } else {
            // closed
            state::remove_state(el, "open");
            state::remove_state(el, "entering");
            state::remove_state(el, "exiting");
            state::add_state(el, "closed");
        }
    }
}

// ---------------------------------------------------------------------------
// inert / aria-hidden background
// ---------------------------------------------------------------------------

fn set_inert_background(active: bool, portal: &Option<Element>) {
    let Some(doc) = doc() else { return };
    let Some(body) = doc.body() else { return };
    let nodes = body.child_nodes();
    for i in 0..nodes.length() {
        let Some(node) = nodes.item(i) else { continue };
        let Some(child) = node.dyn_into::<Element>().ok() else { continue };
        // nunca aplica no portal — checa por atributo (comparacao de elemento pode falhar)
        if child.has_attribute("data-rs-confirm-dialog-portal") { continue; }
        if let Some(portal_el) = portal.as_ref() {
            if child == *portal_el { continue; }
        }
        // nunca aplica se o elemento tem foco ou contém o foco
        if active {
            if let Some(focused) = doc.active_element() {
                if child.contains(Some(&focused)) { continue; }
            }
            // usa apenas inert — mais correto que aria-hidden
            let _ = child.set_attribute("inert", "");
        } else {
            let _ = child.remove_attribute("inert");
            let _ = child.remove_attribute("aria-hidden");
        }
    }
}

// ---------------------------------------------------------------------------
// Focus Trap
// ---------------------------------------------------------------------------

fn focusable_elements(content: &Element) -> Vec<web_sys::HtmlElement> {
    let sel = "a[href]:not([disabled]), button:not([disabled]), input:not([disabled]), \
               select:not([disabled]), textarea:not([disabled]), \
               [tabindex]:not([tabindex='-1'])";
    let mut els = Vec::new();
    if let Ok(nodes) = content.query_selector_all(sel) {
        for i in 0..nodes.length() {
            if let Some(n) = nodes.item(i).and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok()) {
                els.push(n);
            }
        }
    }
    els
}

fn focus_first(content: &Element) {
    let els = focusable_elements(content);
    if let Some(el) = els.first() {
        let _ = el.focus();
    } else if let Ok(html) = content.clone().dyn_into::<web_sys::HtmlElement>() {
        let _ = html.focus();
    }
}

fn active_element() -> Option<Element> {
    doc()?.active_element()
}

// ---------------------------------------------------------------------------
// aria linkage
// ---------------------------------------------------------------------------

fn link_aria(root: &Element) {
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    let Some(portal) = portal_of(root) else { return };
    let content = portal.query_selector("[data-rs-confirm-dialog-content]").ok().flatten();
    let title   = portal.query_selector("[data-rs-confirm-dialog-title]").ok().flatten();
    let desc    = portal.query_selector("[data-rs-confirm-dialog-description]").ok().flatten();
    if let Some(ref content) = content {
        if let Some(title) = title {
            let id = format!("{}-title", uid);
            let _ = title.set_attribute("id", &id);
            let _ = content.set_attribute("aria-labelledby", &id);
        }
        if let Some(desc) = desc {
            let id = format!("{}-desc", uid);
            let _ = desc.set_attribute("id", &id);
            let _ = content.set_attribute("aria-describedby", &id);
        }
    }
}

// ---------------------------------------------------------------------------
// Open / Close
// ---------------------------------------------------------------------------

fn open(root: &Element, prev_focus: &std::rc::Rc<std::cell::Cell<Option<Element>>>) {
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    web_sys::console::log_1(&format!("confirm_dialog::open uid={}", uid).into());

    prev_focus.set(active_element());

    // re-propaga owner (Leptos Show pode ter montado o portal depois do init)
    if let Some(portal) = portal_of(root) {
        propagate_owner(&portal, &uid);
    }

    // root state
    state::open(root);

    let (overlay, content) = portal_nodes(root);
    let portal = portal_of(root);
    
    web_sys::console::log_1(&format!("confirm_dialog::open overlay={} content={}", 
        overlay.is_some(), content.is_some()).into());

    // entering → open após 1 frame
    set_state_nodes(&overlay, &content, "entering");

    {
        let overlay2 = overlay.clone();
        let content2 = content.clone();
        let cb = Closure::once(move || {
            set_state_nodes(&overlay2, &content2, "open");
        });
        let _ = web_sys::window().unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), 16);
        cb.forget();
    }

    state::set_scroll_lock(true);

    // 1. foca imediatamente (tira foco do trigger antes do inert)
    if let Some(ref content_el) = content {
        focus_first(content_el);
    }
    // 2. aplica inert após 1 frame (garante que foco já saiu do background)
    {
        let portal3 = portal.clone();
        let cb = Closure::once(move || {
            set_inert_background(true, &portal3);
        });
        let _ = web_sys::window().unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), 16);
        cb.forget();
    }
}

fn close(root: &Element, prev_focus: &std::rc::Rc<std::cell::Cell<Option<Element>>>) {
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    web_sys::console::log_1(&format!("confirm_dialog::close uid={}", uid).into());

    let (overlay, content) = portal_nodes(root);
    let portal = portal_of(root);
    let duration: i32 = transition_duration_ms(root);

    // exiting
    set_state_nodes(&overlay, &content, "exiting");
    state::close(root);

    // após transition: closed + cleanup
    {
        let overlay2   = overlay.clone();
        let content2   = content.clone();
        let portal2    = portal.clone();
        let pf         = prev_focus.clone();
        let cb = Closure::once(move || {
            set_state_nodes(&overlay2, &content2, "closed");
            state::set_scroll_lock(false);
            set_inert_background(false, &portal2);
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
    let Some(doc) = doc() else { return };
    if let Ok(Some(root)) = doc.query_selector(&format!("[data-rs-confirm-dialog][data-rs-uid='{}']", uid)) {
        let pf = std::rc::Rc::new(std::cell::Cell::new(active_element()));
        open(&root, &pf);
    }
}

#[wasm_bindgen(js_name = confirm_dialog_close)]
pub fn confirm_dialog_close(uid: &str) {
    let Some(doc) = doc() else { return };
    if let Ok(Some(root)) = doc.query_selector(&format!("[data-rs-confirm-dialog][data-rs-uid='{}']", uid)) {
        let pf = std::rc::Rc::new(std::cell::Cell::new(None::<Element>));
        close(&root, &pf);
    }
}

// ---------------------------------------------------------------------------
// Init
// ---------------------------------------------------------------------------

pub fn init(root: Element) {
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    web_sys::console::log_1(&format!("confirm_dialog::init uid={}", uid).into());

    if !lifecycle::init_guard(&root) {
        web_sys::console::log_1(&"confirm_dialog::init SKIPPED".into());
        return;
    }
    web_sys::console::log_1(&"confirm_dialog::init RUNNING".into());

    // portal → body + owner
    if let Some(portal) = portal_of(&root) {
        propagate_owner(&portal, &uid);
        if let Some(body) = doc().and_then(|d| d.body()) {
            let in_body = portal.parent_element()
                .map(|p| p.tag_name() == "BODY")
                .unwrap_or(false);
            if !in_body {
                let _ = body.append_child(&portal);
                web_sys::console::log_1(&format!("confirm_dialog::init portal moved uid={}", uid).into());
            }
        }
        // garante estado inicial closed nos nós do portal
        let (overlay, content) = portal_nodes(&root);
        set_state_nodes(&overlay, &content, "closed");
    }

    link_aria(&root);

    let prev_focus = std::rc::Rc::new(std::cell::Cell::new(None::<Element>));

    // trigger global — escuta no document, identifica por data-rs-target ou pertencimento ao root
    {
        let root_cb = root.clone();
        let uid2    = uid.clone();
        let pf      = prev_focus.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            // verifica se o elemento clicado (ou ancestral) é um trigger
            let trigger = target.closest("[data-rs-confirm-dialog-trigger]").ok().flatten();
            let Some(trigger) = trigger else { return };
            // trigger dentro do root (modelo clássico)
            let in_root = root_cb.contains(Some(&trigger));
            // trigger externo com data-rs-target apontando para este uid
            let targets_uid = trigger.get_attribute("data-rs-target").as_deref() == Some(&uid2);
            if in_root || targets_uid {
                open(&root_cb, &pf);
            }
        });
        if let Some(doc) = doc() {
            let _ = doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        }
        cb.forget();
    }

    // document click: overlay / cancel / confirm
    {
        let root_cb = root.clone();
        let uid2 = uid.clone();
        let pf = prev_focus.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            if !state::is_open(&root_cb) { return; }
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            // overlay: sem owner, detecta por atributo direto
            if query::closest(&target, "[data-rs-confirm-dialog-overlay]") { close(&root_cb, &pf); return; }
            // cancel/confirm: verifica owner via closest
            let owner = target.closest("[data-rs-owner]").ok().flatten()
                .and_then(|el| el.get_attribute("data-rs-owner"))
                .or_else(|| target.get_attribute("data-rs-owner"));
            if owner.as_deref() != Some(&uid2) { return; }
            if query::closest(&target, "[data-rs-confirm-dialog-cancel]")  { close(&root_cb, &pf); return; }
            if query::closest(&target, "[data-rs-confirm-dialog-confirm]") { close(&root_cb, &pf); }
        });
        if let Some(doc) = doc() {
            let _ = doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        }
        cb.forget();
    }

    // keydown: Escape + Tab trap
    {
        let root_cb = root.clone();
        let uid2 = uid.clone();
        let pf = prev_focus.clone();
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
            if !state::is_open(&root_cb) { return; }

            if e.key() == "Escape" {
                e.prevent_default();
                close(&root_cb, &pf);
                return;
            }

            if e.key() != "Tab" { return; }

            let Some(doc) = doc() else { return };
            let sel = format!(
                "[data-rs-confirm-dialog-content][data-rs-owner='{}']", uid2
            );
            let Some(content) = doc.query_selector(&sel).ok().flatten() else { return };
            let els = focusable_elements(&content);
            if els.is_empty() { e.prevent_default(); return; }

            let first = els.first().unwrap();
            let last  = els.last().unwrap();
            let active = doc.active_element();

            let first_el = first.clone().dyn_into::<Element>().ok();
            let last_el  = last.clone().dyn_into::<Element>().ok();

            if e.shift_key() {
                if active == first_el {
                    e.prevent_default();
                    let _ = last.focus();
                }
            } else if active == last_el {
                e.prevent_default();
                let _ = first.focus();
            }
        });
        if let Some(win) = web_sys::window() {
            let _ = win.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        }
        cb.forget();
    }
}
