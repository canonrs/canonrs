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
        if let Some(fallback) = query::first(&root, "[data-rs-avatar-fallback]") {
            state::remove_state(&fallback, "closed");
            state::add_state(&fallback, "open");
        }
        pulse_if_online(&root);
        return;
    }

    let img = img.unwrap();
    let fallback = query::first(&root, "[data-rs-avatar-fallback]");

    // on error — mostra fallback
    let root_err = root.clone();
    let img_err = img.clone();
    let fallback_err = fallback.clone();
    let on_error = Closure::<dyn Fn(web_sys::Event)>::new(move |_: web_sys::Event| {
        state::remove_state(&root_err, "loading");
        state::add_state(&root_err, "error");
        state::remove_state(&img_err, "open");
        state::add_state(&img_err, "closed");
        if let Some(ref fb) = fallback_err {
            state::remove_state(fb, "closed");
            state::add_state(fb, "open");
        }
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
        if let Some(dot) = query::first(root, "[data-rs-avatar-status]") {
            let _ = dot.set_attribute("data-rs-pulse", "");
        }
    }
}
