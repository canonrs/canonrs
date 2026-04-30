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

fn open(root: &Element, prev_focus: &std::rc::Rc<std::cell::Cell<Option<Element>>>) {
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    prev_focus.set(focus::active_element());

    state::open(root);
    stack::push(&uid, KIND);

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

    // posicionamento
    {
        let root2 = root.clone();
        let cb = Closure::once(move || {
            positioning::auto_side(&root2, &format!("[{}]", CONTENT_ATTR));
        });
        let _ = web_sys::window().unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), 20);
        cb.forget();
    }

    // inicializa elementos interativos dentro do content via runtime
    if let Ok(Some(content_el)) = root.query_selector(&format!("[{}]", CONTENT_ATTR)) {
        canonrs_runtime::scan_children(&content_el);
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

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    stack::ensure_global_listeners();

    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    let prev_focus = std::rc::Rc::new(std::cell::Cell::new(None::<Element>));

    // rs:popover:close — fecha programaticamente
    {
        let root_live = root.clone();
        let pf = std::rc::Rc::new(std::cell::Cell::new(None::<Element>));
        let cb = Closure::<dyn Fn(web_sys::Event)>::new(move |_: web_sys::Event| {
            close(&root_live, &pf);
        });
        let _ = root.add_event_listener_with_callback("rs:popover:close", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // click — toggle trigger, close button, click fora
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
            if !root_live.contains(Some(target.as_ref())) && state::is_open(&root_live) {
                close(&root_live, &pf);
            }
        });
    }

    // keydown — Escape fecha
    {
        let uid2 = uid.clone();
        let pf = prev_focus.clone();
        stack::register_keydown(&uid, move |e| {
            let Some(root_live) = query::root_of("data-rs-popover", &uid2) else { return };
            if !state::is_open(&root_live) { return; }
            if e.key() == "Escape" && stack::is_top(&uid2) {
                e.prevent_default();
                close(&root_live, &pf);
            }
        });
    }

    // focusout — fecha quando foco sai do popover (non-modal: nao prende foco)
    {
        let root2 = root.clone();
        let pf = prev_focus.clone();
        let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::new(move |e: web_sys::FocusEvent| {
            if !state::is_open(&root2) { return; }
            let related = e.related_target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok());
            let focus_left = match related {
                Some(ref el) => !root2.contains(Some(el as &web_sys::Element)),
                None => true,
            };
            if focus_left {
                close(&root2, &pf);
            }
        });
        let _ = root.add_event_listener_with_callback("focusout", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}
