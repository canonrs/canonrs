//! Dismiss — close button pattern para alert, banner, toast

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{state, query};

/// Registra handler de dismiss no botão de close
/// close_selector: seletor do botão de fechar (ex: "[data-rs-alert-close]")
pub fn init(root: &Element, close_selector: &str) {
    let Some(btn) = query::first(root, close_selector) else { return };
    let root_cb = root.clone();
    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
        state::remove_state(&root_cb, "open");
        state::add_state(&root_cb, "closed");
    });
    let _ = btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
    cb.forget();
}

/// Dismiss com auto-timer (toast pattern)
pub fn init_with_timer(root: &Element, close_selector: &str, duration_ms: i32) {
    init(root, close_selector);
    let root_cb = root.clone();
    let cb = Closure::once(move || {
        // nao fechar se pausado (hover/focus)
        let current = root_cb.get_attribute("data-rs-state").unwrap_or_default();
        if current.contains("paused") { return; }
        state::remove_state(&root_cb, "open");
        state::add_state(&root_cb, "closed");
    });
    if let Some(win) = web_sys::window() {
        let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
            cb.as_ref().unchecked_ref(), duration_ms
        );
    }
    cb.forget();
}
