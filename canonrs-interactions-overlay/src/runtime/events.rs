//! Events — factories para listeners padrao de overlay
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use super::{query, state};

/// Toggle — popover, dropdown, hover-card (trigger abre E fecha)
pub fn on_click_root_toggle(root: &Element, triggers: &'static [&'static str], closers: &'static [&'static str]) {
    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
        let Some(current) = query::safe_current(&e) else { return };
        let Some(target)  = query::safe_target(&e)  else { return };
        if triggers.iter().any(|s| query::closest(&target, s)) { state::toggle(&current); return; }
        if closers.iter().any(|s| query::closest(&target, s))  { state::close(&current); }
    });
    let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
    cb.forget();
}

/// Open-only — dialog, modal, sheet (trigger so abre, close e separado)
pub fn on_click_root_open(root: &Element, triggers: &'static [&'static str], closers: &'static [&'static str]) {
    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
        let Some(current) = query::safe_current(&e) else { return };
        let Some(target)  = query::safe_target(&e)  else { return };
        if triggers.iter().any(|s| query::closest(&target, s)) { state::open(&current); return; }
        if closers.iter().any(|s| query::closest(&target, s))  { state::close(&current); }
    });
    let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
    cb.forget();
}

pub fn on_outside_click(root_selector: &'static str) {
    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
        let Some(target) = query::safe_target(&e) else { return };
        query::each(root_selector, |node| {
            if !node.contains(Some(target.as_ref())) && state::is_open(&node) { state::close(&node); }
        });
    });
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        let _ = doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
    }
    cb.forget();
}

pub fn on_escape(root_selector: &'static str) {
    let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
        if e.key() != "Escape" { return; }
        query::each(root_selector, |node| { if state::is_open(&node) { state::close(&node); } });
    });
    if let Some(win) = web_sys::window() {
        let _ = win.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
    }
    cb.forget();
}
