//! Popover Interaction Engine
//! CR-430 — Overlay Positioning Contract
//! CR-431 — Trigger → Overlay Lifecycle
//! CR-432 — Outside Click + Focus Policy

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state, stack, focus, transition, positioning, query};

const KIND:         &str = "popover";
const CONTENT_ATTR: &str = "data-rs-popover-content";
const TRIGGER_ATTR: &str = "data-rs-popover-trigger";
const CLOSE_ATTR:   &str = "data-rs-popover-close";
const CSS_VAR:      &str = "--popover-transition-duration";

// ---------------------------------------------------------------------------
// Open / Close
// ---------------------------------------------------------------------------

fn open(root: &Element, prev_focus: &std::rc::Rc<std::cell::Cell<Option<Element>>>) {
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    prev_focus.set(focus::active_element());

    state::open(root);
    stack::push(&uid, KIND);
    stack::apply_z(
        &uid,
        &format!("[{}][data-rs-uid='{}']", CONTENT_ATTR, uid),
        &format!("[{}][data-rs-uid='{}']", CONTENT_ATTR, uid),
    );

    let content = root.query_selector(&format!("[{}]", CONTENT_ATTR)).ok().flatten();
    let content_opt = content.clone().map(|c| Some(c));

    transition::set_state_nodes(&None, &content_opt.flatten(), "entering");
    {
        let c2 = root.query_selector(&format!("[{}]", CONTENT_ATTR)).ok().flatten();
        let cb = Closure::once(move || {
            if let Some(ref c) = c2 {
                let _ = c.set_attribute("data-rs-state", "open");
            }
        });
        let _ = web_sys::window().unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), 16);
        cb.forget();
    }

    // posicionamento após abertura
    {
        let root2 = root.clone();
        let cb = Closure::once(move || {
            positioning::auto_side(&root2, &format!("[{}]", CONTENT_ATTR));
        });
        let _ = web_sys::window().unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), 20);
        cb.forget();
    }

    // foco no content
    if let Some(ref c) = root.query_selector(&format!("[{}]", CONTENT_ATTR)).ok().flatten() {
        focus::focus_first(c);
    }
}

fn close(root: &Element, prev_focus: &std::rc::Rc<std::cell::Cell<Option<Element>>>) {
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    let content = root.query_selector(&format!("[{}]", CONTENT_ATTR)).ok().flatten();
    let duration = transition::duration_ms(root, CSS_VAR);

    transition::set_state_nodes(&None, &content, "exiting");
    state::close(root);
    stack::pop(&uid);

    {
        let c2 = content.clone();
        let pf = prev_focus.clone();
        let cb = Closure::once(move || {
            if let Some(ref c) = c2 {
                let _ = c.set_attribute("data-rs-state", "closed");
            }
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
// Init
// ---------------------------------------------------------------------------

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    stack::ensure_global_listeners();

    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    let prev_focus = std::rc::Rc::new(std::cell::Cell::new(None::<Element>));

    // escuta rs:popover:close para fechar programaticamente
    {
        let root_live = root.clone();
        let pf = std::rc::Rc::new(std::cell::Cell::new(None::<Element>));
        let cb = Closure::<dyn Fn(web_sys::Event)>::new(move |_: web_sys::Event| {
            close(&root_live, &pf);
        });
        let _ = root.add_event_listener_with_callback("rs:popover:close", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // click — toggle trigger, close button
    {
        let uid2 = uid.clone();
        let pf = prev_focus.clone();
        stack::register_click(&uid, move |target| {
            let Some(root_live) = query::root_of("data-rs-popover", &uid2) else { return };
            if query::closest(&target, &format!("[{}]", TRIGGER_ATTR)) {
                if state::is_open(&root_live) { close(&root_live, &pf); }
                else { open(&root_live, &pf); }
                return;
            }
            if query::closest(&target, &format!("[{}]", CLOSE_ATTR)) {
                close(&root_live, &pf);
                return;
            }
            // click fora — fecha
            if !root_live.contains(Some(target.as_ref())) && state::is_open(&root_live) {
                close(&root_live, &pf);
            }
        });
    }

    // keydown — Escape fecha, Tab trapa foco
    {
        let uid2 = uid.clone();
        let pf = prev_focus.clone();
        stack::register_keydown(&uid, move |e| {
            let Some(root_live) = query::root_of("data-rs-popover", &uid2) else { return };
            if !state::is_open(&root_live) { return; }
            if e.key() == "Escape" && stack::is_top(&uid2) {
                e.prevent_default();
                close(&root_live, &pf);
                return;
            }
            if e.key() == "Tab" {
                let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
                let sel = format!("[{}]", CONTENT_ATTR);
                let Some(content) = doc.query_selector(&sel).ok().flatten() else { return };
                focus::trap_tab(e, &content);
            }
        });
    }
}
