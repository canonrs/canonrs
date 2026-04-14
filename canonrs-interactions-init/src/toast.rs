//! Toast Init — auto-dismiss + close button

use web_sys::Element;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::runtime::{lifecycle, dismiss};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    let variant = root.get_attribute("data-rs-variant").unwrap_or_default();

    // error nunca some automaticamente
    if variant == "error" {
        dismiss::init(&root, "[data-rs-toast-close]");
        return;
    }

    // timer por tipo
    let default_duration = match variant.as_str() {
        "success" => 3000,
        "info"    => 4000,
        "warning" => 6000,
        _         => 5000,
    };

    let duration_ms = root.get_attribute("data-rs-duration")
        .and_then(|d| d.parse::<i32>().ok())
        .unwrap_or(default_duration);

    dismiss::init_with_timer(&root, "[data-rs-toast-close]", duration_ms);

    // pause on hover/focus
    let root_hover = root.clone();
    let root_leave = root.clone();
    let pause_cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
        crate::runtime::state::add_state(&root_hover, "paused");
    });
    let resume_cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
        crate::runtime::state::remove_state(&root_leave, "paused");
    });
    let _ = root.add_event_listener_with_callback("mouseenter", pause_cb.as_ref().unchecked_ref());
    let _ = root.add_event_listener_with_callback("mouseleave", resume_cb.as_ref().unchecked_ref());
    pause_cb.forget();
    resume_cb.forget();
}
