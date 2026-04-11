//! Button Init — hover/focus/active states

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state};

fn is_disabled(el: &Element) -> bool {
    el.get_attribute("data-rs-state").map(|s| s.contains("disabled")).unwrap_or(false)
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    {
        let r = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
            if is_disabled(&r) { return; }
            state::add_state(&r, "hover");
        });
        let _ = root.add_event_listener_with_callback("mouseenter", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    {
        let r = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
            state::remove_state(&r, "hover");
            state::remove_state(&r, "active");
        });
        let _ = root.add_event_listener_with_callback("mouseleave", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    {
        let r = root.clone();
        let cb = Closure::<dyn Fn(web_sys::PointerEvent)>::new(move |_: web_sys::PointerEvent| {
            if is_disabled(&r) { return; }
            state::add_state(&r, "active");
        });
        let _ = root.add_event_listener_with_callback("pointerdown", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    for ev in ["pointerup", "pointercancel"] {
        let r = root.clone();
        let cb = Closure::<dyn Fn(web_sys::PointerEvent)>::new(move |_: web_sys::PointerEvent| {
            state::remove_state(&r, "active");
        });
        let _ = root.add_event_listener_with_callback(ev, cb.as_ref().unchecked_ref());
        cb.forget();
    }

    {
        let r = root.clone();
        let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::new(move |_: web_sys::FocusEvent| {
            if is_disabled(&r) { return; }
            state::add_state(&r, "focus");
        });
        let _ = root.add_event_listener_with_callback("focus", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    {
        let r = root.clone();
        let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::new(move |_: web_sys::FocusEvent| {
            state::remove_state(&r, "focus");
        });
        let _ = root.add_event_listener_with_callback("blur", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}
