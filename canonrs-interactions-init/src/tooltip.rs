//! Tooltip Init — DOM micro-interactions para [data-rs-tooltip]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state};

fn get_delay(root: &Element, attr: &str, default: i32) -> i32 {
    let mut el = root.parent_element();
    loop {
        match el {
            Some(ref e) if e.has_attribute("data-rs-tooltip-provider") => {
                return e.get_attribute(attr)
                    .and_then(|v| v.parse::<i32>().ok())
                    .unwrap_or(default);
            }
            Some(ref e) => { el = e.parent_element(); }
            None => break,
        }
    }
    default
}

fn open_content(root: &Element) {
    if let Ok(Some(c)) = root.query_selector("[data-rs-tooltip-content]") {
        state::open(&c);
    }
    state::open(root);
}

fn close_content(root: &Element) {
    if let Ok(Some(c)) = root.query_selector("[data-rs-tooltip-content]") {
        state::close(&c);
    }
    state::close(root);
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    #[cfg(target_arch = "wasm32")]
    {
        let uid = root.get_attribute("data-rs-uid").unwrap_or_else(|| "NO-UID".to_string());
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("[tooltip:init] uid={}", uid)));
    }

    let delay_open  = get_delay(&root, "data-rs-delay-open", 400);
    let delay_close = get_delay(&root, "data-rs-delay-close", 100);

    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
            let root_clone = root_cb.clone();
            let cb2 = Closure::once(move || { open_content(&root_clone); });
            if let Some(win) = web_sys::window() {
                let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(cb2.as_ref().unchecked_ref(), delay_open);
            }
            cb2.forget();
        });
        let _ = root.add_event_listener_with_callback("mouseenter", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
            let root_clone = root_cb.clone();
            let cb2 = Closure::once(move || { close_content(&root_clone); });
            if let Some(win) = web_sys::window() {
                let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(cb2.as_ref().unchecked_ref(), delay_close);
            }
            cb2.forget();
        });
        let _ = root.add_event_listener_with_callback("mouseleave", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::new(move |_: web_sys::FocusEvent| {
            open_content(&root_cb);
        });
        let _ = root.add_event_listener_with_callback("focusin", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::new(move |_: web_sys::FocusEvent| {
            close_content(&root_cb);
        });
        let _ = root.add_event_listener_with_callback("focusout", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}
