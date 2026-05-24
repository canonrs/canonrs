//! CopyButton Interaction Engine
//! Core: dom/{state} + clipboard

use wasm_bindgen_futures::spawn_local;
use web_sys::Element;
use canonrs_interactions_core::dom::state;
use canonrs_interactions_core::runtime::{listeners, timers};

fn copy_to_clipboard(text: String, el: Element, reset_delay: i32) {
    let window = match web_sys::window() { Some(w) => w, None => return };
    let clipboard = window.navigator().clipboard();
    let promise = clipboard.write_text(&text);
    spawn_local(async move {
        let result = wasm_bindgen_futures::JsFuture::from(promise).await;
        state::remove(&el, "idle");
        if result.is_ok() {
            state::remove(&el, "error");
            state::add(&el, "copied");
        } else {
            state::remove(&el, "copied");
            state::add(&el, "error");
        }
        schedule_reset(el, reset_delay);
    });
}

fn schedule_reset(el: Element, delay: i32) {
    timers::timeout(delay, move || {
        state::remove(&el, "copied");
        state::remove(&el, "error");
        state::add(&el, "idle");
    });
}

pub fn init(el: Element) {
    let uid = el.get_attribute("data-rs-uid").unwrap_or_default();
    let reset_delay = el.get_attribute("data-rs-reset-delay")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(2000);

    listeners::listen(&uid, &el, "click", {
        let el_c = el.clone();
        move |_: web_sys::Event| {
            let text   = el_c.get_attribute("data-rs-copy-text").unwrap_or_default();
            let target = el_c.get_attribute("data-rs-copy-target").unwrap_or_default();
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
                Some(t) if !t.is_empty() => copy_to_clipboard(t, el_c.clone(), reset_delay),
                _ => {
                    state::remove(&el_c, "idle");
                    state::remove(&el_c, "copied");
                    state::add(&el_c, "error");
                    schedule_reset(el_c.clone(), reset_delay);
                }
            }
        }
    });

    listeners::listen(&uid, &el, "mouseover", move |_: web_sys::Event| {
        // hover state tracked via CSS :hover — no state needed
    });
}
