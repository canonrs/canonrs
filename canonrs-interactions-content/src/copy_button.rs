//! CopyButton Interaction Engine

use wasm_bindgen::prelude::*;
use crate::shared::{remove_state, is_initialized, mark_initialized};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::Element;

fn add_state(el: &Element, state: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    if !current.split_whitespace().any(|s| s == state) {
        let next = if current.is_empty() { state.to_string() } else { format!("{} {}", current, state) };
        el.set_attribute("data-rs-state", &next).ok();
    }
}


fn copy_to_clipboard(text: String, el_ok: Element, el_err: Element, reset_delay: i32) {
    let json_text = js_sys::JSON::stringify(&JsValue::from_str(&text))
        .unwrap_or_else(|_| js_sys::JsString::from("").into());
    let script = format!("return navigator.clipboard.writeText({})", String::from(json_text));
    let func = js_sys::Function::new_no_args(&script);
    match func.call0(&JsValue::NULL) {
        Ok(p) => {
            let promise = js_sys::Promise::from(p);
            spawn_local(async move {
                let result = wasm_bindgen_futures::JsFuture::from(promise).await;
                if result.is_ok() {
                    remove_state(&el_ok, "idle");
                    remove_state(&el_ok, "error");
                    add_state(&el_ok, "copied");
                    let el_reset = el_ok.clone();
                    let cb = Closure::once(Box::new(move || {
                        remove_state(&el_reset, "copied");
                        add_state(&el_reset, "idle");
                    }) as Box<dyn FnOnce()>);
                    if let Some(win) = web_sys::window() {
                        win.set_timeout_with_callback_and_timeout_and_arguments_0(
                            cb.as_ref().unchecked_ref(), reset_delay,
                        ).ok();
                    }
                    cb.forget();
                } else {
                    remove_state(&el_err, "idle");
                    remove_state(&el_err, "copied");
                    add_state(&el_err, "error");
                }
            });
        }
        Err(_) => {
            remove_state(&el_err, "idle");
            remove_state(&el_err, "copied");
            add_state(&el_err, "error");
        }
    }
}

pub fn init(el: Element) {
    if is_initialized(&el) { return; }
    mark_initialized(&el);
    let reset_delay = el.get_attribute("data-rs-reset-delay")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(2000);

    let text   = el.get_attribute("data-rs-text").unwrap_or_default();
    let target = el.get_attribute("data-rs-target").unwrap_or_default();

    let el_cb = el.clone();
    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |_| {
        let copy_text = if !text.is_empty() {
            Some(text.clone())
        } else if !target.is_empty() {
            let selector = if target.starts_with('#') { target.clone() } else { format!("#{}", target) };
            web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.query_selector(&selector).ok().flatten())
                .and_then(|e| e.text_content())
        } else { None };

        match copy_text {
            Some(t) if !t.is_empty() => {
                copy_to_clipboard(t, el_cb.clone(), el_cb.clone(), reset_delay);
            }
            _ => {
                remove_state(&el_cb, "idle");
                remove_state(&el_cb, "copied");
                add_state(&el_cb, "error");
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
