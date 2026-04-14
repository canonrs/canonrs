//! Avatar Init — image load/error fallback + pulse

use web_sys::Element;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::runtime::{lifecycle, state, query};

pub fn init(root: Element) {
    web_sys::console::log_1(&"[avatar] init called".into());
    if !lifecycle::init_guard(&root) { return; }

    // se nao tem imagem — mostra fallback imediatamente
    let img = query::first(&root, "[data-rs-avatar-image]");
    if img.is_none() {
        state::add_state(&root, "fallback-open");
        state::remove_state(&root, "fallback-closed");
        pulse_if_online(&root);
        return;
    }

    let img = img.unwrap();
    // on error — mostra fallback
    let root_err = root.clone();
    let _img_err = img.clone();
    let on_error = Closure::<dyn Fn(web_sys::Event)>::new(move |_: web_sys::Event| {
        state::remove_state(&root_err, "loading");
        state::add_state(&root_err, "error");
        state::add_state(&root_err, "image-closed");
        state::remove_state(&root_err, "image-open");
        state::add_state(&root_err, "fallback-open");
        state::remove_state(&root_err, "fallback-closed");
    });
    let _ = img.add_event_listener_with_callback("error", on_error.as_ref().unchecked_ref());
    on_error.forget();

    // on load — remove loading
    let root_load = root.clone();
    let on_load = Closure::<dyn Fn(web_sys::Event)>::new(move |_: web_sys::Event| {
        state::remove_state(&root_load, "loading");
    });
    let _ = img.add_event_listener_with_callback("load", on_load.as_ref().unchecked_ref());
    on_load.forget();

    pulse_if_online(&root);
}

fn pulse_if_online(root: &Element) {
    let state = root.get_attribute("data-rs-state").unwrap_or_default();
    if state.contains("online") {
        if let Some(dot) = query::first(root, "[data-rs-status-dot]") {
            crate::runtime::state::add_state(&dot, "pulse");
        }
    }
}
