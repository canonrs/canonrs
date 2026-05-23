//! Popover Interaction Engine
//! Core: dom/{state, query} + Overlay: stack, focus, transition, positioning

use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::{state, query};
use canonrs_interactions_core::runtime::{listeners, timers};
use crate::runtime::{stack, focus, transition, positioning};

const KIND:         &str = "popover";
const CONTENT_ATTR: &str = "data-rs-popover-content";
const TRIGGER_ATTR: &str = "data-rs-popover-trigger";
const CLOSE_ATTR:   &str = "data-rs-popover-close";
const CSS_VAR:      &str = "--popover-transition-duration";

fn open(root: &Element, prev_focus: &std::rc::Rc<std::cell::Cell<Option<Element>>>) {
    if stack::has_modal_open() { return; }
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    prev_focus.set(focus::active_element());
    state::open(root);
    stack::push(&uid, KIND);
    if let Ok(Some(trigger)) = root.query_selector(&format!("[{}]", TRIGGER_ATTR)) {
        if let Some(v) = trigger.get_attribute("data-rs-value") { let _ = root.set_attribute("data-rs-current-value", &v); }
        if let Some(l) = trigger.get_attribute("data-rs-label") { let _ = root.set_attribute("data-rs-current-label", &l); }
    }
    if let Some(name) = root.get_attribute("data-rs-name") {
        if !name.is_empty() {
            let val = root.get_attribute("data-rs-current-value").unwrap_or_default();
            canonrs_interactions_core::integration::form::sync_hidden_input(root, &val);
        }
    }
    let content = root.query_selector(&format!("[{}]", CONTENT_ATTR)).ok().flatten();
    transition::set_state_nodes(&None, &content, "entering");
    { let c2 = root.query_selector(&format!("[{}]", CONTENT_ATTR)).ok().flatten();
      timers::next_frame(move || { if let Some(ref c) = c2 { state::open(c); } }); }
    { let root2 = root.clone();
      timers::timeout(20, move || { positioning::auto_side(&root2, &format!("[{}]", CONTENT_ATTR)); }); }
}

fn close(root: &Element, prev_focus: &std::rc::Rc<std::cell::Cell<Option<Element>>>) {
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    let content = root.query_selector(&format!("[{}]", CONTENT_ATTR)).ok().flatten();
    let duration = transition::duration_ms(root, CSS_VAR);
    transition::set_state_nodes(&None, &content, "exiting");
    state::close(root);
    stack::pop(&uid);
    { let c2 = content.clone(); let pf = prev_focus.clone();
      timers::after_duration(duration as i32, move || {
          if let Some(ref c) = c2 { state::close(c); }
          if let Some(el) = pf.take() {
              if let Ok(html) = el.dyn_into::<web_sys::HtmlElement>() { let _ = html.focus(); }
          }
      }); }
}

pub fn init(root: Element) {
    stack::ensure_global_listeners();
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    let prev_focus = std::rc::Rc::new(std::cell::Cell::new(None::<Element>));

    {
        let root_live = root.clone();
        let pf = std::rc::Rc::new(std::cell::Cell::new(None::<Element>));
        listeners::listen(&uid, &root, "rs:popover:close", move |_: web_sys::Event| {
            close(&root_live, &pf);
        });
    }
    {
        let uid2 = uid.clone(); let pf = prev_focus.clone();
        stack::register_click(&uid, move |target| {
            let Some(root_live) = query::root_of("data-rs-popover", &uid2) else { return };
            if query::closest(&target, &format!("[{}]", TRIGGER_ATTR)) {
                if state::is_open(&root_live) { close(&root_live, &pf); } else { open(&root_live, &pf); }
                return;
            }
            if query::closest(&target, &format!("[{}]", CLOSE_ATTR)) { close(&root_live, &pf); return; }
            if !root_live.contains(Some(target.as_ref())) && state::is_open(&root_live) {
                close(&root_live, &pf);
            }
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                if let Ok(others) = doc.query_selector_all("[data-rs-popover]") {
                    for i in 0..others.length() {
                        if let Some(node) = others.item(i).and_then(|n| n.dyn_into::<Element>().ok()) {
                            let other_uid = node.get_attribute("data-rs-uid").unwrap_or_default();
                            if other_uid != uid2 && state::is_open(&node) && !node.contains(Some(target.as_ref())) {
                                let pf_other = std::rc::Rc::new(std::cell::Cell::new(None::<Element>));
                                close(&node, &pf_other);
                            }
                        }
                    }
                }
            }
        });
    }
    {
        let uid2 = uid.clone(); let pf = prev_focus.clone();
        stack::register_keydown(&uid, move |e| {
            let Some(root_live) = query::root_of("data-rs-popover", &uid2) else { return };
            if !state::is_open(&root_live) { return; }
            if e.key() == "Escape" && stack::is_top(&uid2) { e.prevent_default(); close(&root_live, &pf); }
        });
    }
    {
        let root2 = root.clone(); let pf = prev_focus.clone();
        listeners::listen(&uid, &root, "focusout", move |_: web_sys::Event| {
            if !state::is_open(&root2) { return; }
            let root3 = root2.clone(); let pf2 = pf.clone();
            timers::timeout(0, move || {
                let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
                if let Some(active) = doc.active_element() {
                    if root3.contains(Some(&active)) { return; }
                }
                close(&root3, &pf2);
            });
        });
    }
}
