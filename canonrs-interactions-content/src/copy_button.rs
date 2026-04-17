//! CopyButton Interaction Engine

use wasm_bindgen::prelude::*;
use crate::shared::{add_state, remove_state, is_initialized, mark_initialized};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::Element;

fn copy_to_clipboard(text: String, el: Element, reset_delay: i32) {
    let window = match web_sys::window() { Some(w) => w, None => return };
    let clipboard = window.navigator().clipboard();

    let promise = clipboard.write_text(&text);
    spawn_local(async move {
        let result = wasm_bindgen_futures::JsFuture::from(promise).await;
        if result.is_ok() {
            remove_state(&el, "idle");
            remove_state(&el, "error");
            add_state(&el, "copied");
        } else {
            remove_state(&el, "idle");
            remove_state(&el, "copied");
            add_state(&el, "error");
        }
        schedule_reset(el, reset_delay);
    });
}

fn schedule_reset(el: Element, delay: i32) {
    let cb = Closure::once(Box::new(move || {
        remove_state(&el, "copied");
        remove_state(&el, "error");
        add_state(&el, "idle");
    }) as Box<dyn FnOnce()>);
    if let Some(win) = web_sys::window() {
        win.set_timeout_with_callback_and_timeout_and_arguments_0(
            cb.as_ref().unchecked_ref(), delay,
        ).ok();
    }
    cb.forget();
}

pub fn init(el: Element) {
    if is_initialized(&el) { return; }
    mark_initialized(&el);

    let reset_delay = el.get_attribute("data-rs-reset-delay")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(2000);

    let el_cb = el.clone();
    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |_| {
        let text   = el_cb.get_attribute("data-rs-copy-text").unwrap_or_default();
        let target = el_cb.get_attribute("data-rs-copy-target").unwrap_or_default();

        let copy_text = if !text.is_empty() {
            Some(text)
        } else if !target.is_empty() {
            let selector = if target.starts_with('#') { target } else { format!("#{}", target) };
            web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.query_selector(&selector).ok().flatten())
                .and_then(|e| e.text_content())
        } else {
            None
        };

        match copy_text {
            Some(t) if !t.is_empty() => {
                copy_to_clipboard(t, el_cb.clone(), reset_delay);
            }
            _ => {
                remove_state(&el_cb, "idle");
                remove_state(&el_cb, "copied");
                add_state(&el_cb, "error");
                schedule_reset(el_cb.clone(), reset_delay);
            }
        }
    }));
    let _ = el.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
    cb.forget();
}

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-copy-button]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init(el); }
        }
    }
}
