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
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("[popover] OPENED"));

    // propaga value e label do trigger para o root — DOM como fonte de verdade
    if let Ok(Some(trigger)) = root.query_selector(&format!("[{}]", TRIGGER_ATTR)) {
        if let Some(v) = trigger.get_attribute("data-rs-value") {
            let _ = root.set_attribute("data-rs-current-value", &v);
        }
        if let Some(l) = trigger.get_attribute("data-rs-label") {
            let _ = root.set_attribute("data-rs-current-label", &l);
        }
    }

    // sync hidden input para form submission nativa
    if let Some(name) = root.get_attribute("data-rs-name") {
        if !name.is_empty() {
            let val = root.get_attribute("data-rs-current-value").unwrap_or_default();
            crate::runtime::form::sync_hidden_input(root, &val);
        }
    }

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
    // usa setTimeout(0) para aguardar o focus do proximo elemento antes de fechar
    {
        let root2 = root.clone();
        let pf = prev_focus.clone();
        let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::new(move |_: web_sys::FocusEvent| {
            if !state::is_open(&root2) { return; }
            web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("[popover] focusout disparado"));
            let root3 = root2.clone();
            let pf2 = pf.clone();
            let timeout = Closure::once(move || {
                let doc = match web_sys::window().and_then(|w| w.document()) {
                    Some(d) => d,
                    None => return,
                };
                if let Some(active) = doc.active_element() {
                    let tag = active.tag_name();
                    let state = active.get_attribute("data-rs-state").unwrap_or_default();
                    let inside = root3.contains(Some(&active));
                    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(
                        &format!("[popover] active_element after focusout: tag={} state={} inside={}", tag, state, inside)
                    ));
                    if root3.contains(Some(&active)) {
                        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("[popover] foco ainda dentro — NAO fecha"));
                        return;
                    }
                } else {
                    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("[popover] active_element = None — fechando"));
                }
                close(&root3, &pf2);
            });
            let _ = web_sys::window().unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    timeout.as_ref().unchecked_ref(), 0
                );
            timeout.forget();
        });
        let _ = root.add_event_listener_with_callback("focusout", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}
