//! InputOtp Init — foco, digitacao, paste, backspace, cursor real

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, query, state};

fn update_slots(root: &Element) {
    let input = match query::first(root, "[data-rs-input-otp]") {
        Some(el) => el,
        None => return,
    };
    let input_el = match input.clone().dyn_into::<web_sys::HtmlInputElement>() {
        Ok(el) => el,
        Err(_) => return,
    };
    let value = input_el.value();
    let slots = query::all(root, "[data-rs-input-otp-slot]");
    let cursor = value.len().min(slots.len().saturating_sub(1).max(0));

    for (i, slot) in slots.iter().enumerate() {
        let inner = slot.query_selector("[data-rs-slot-inner]").ok().flatten();
        let ch = value.chars().nth(i).map(|c| c.to_string()).unwrap_or_default();
        if let Some(inner) = inner {
            inner.set_text_content(Some(&ch));
        }
        if i == cursor {
            state::add_state(slot, "active");
        } else {
            state::remove_state(slot, "active");
        }
    }
}

fn focus_input_at_end(root: &Element) {
    if let Some(input) = query::first(root, "[data-rs-input-otp]") {
        if let Ok(el) = input.dyn_into::<web_sys::HtmlInputElement>() {
            let len = el.value().len() as u32;
            let _ = el.focus();
            let _ = el.set_selection_range(len, len);
        }
    }
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    // click → foca e posiciona cursor no final
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
            focus_input_at_end(&root_cb);
        });
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // input → atualiza slots (digitacao + paste + backspace)
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::Event)>::new(move |_: web_sys::Event| {

            update_slots(&root_cb);
        });
        if let Some(input) = query::first(&root, "[data-rs-input-otp]") {
            let _ = input.add_event_listener_with_callback("input", cb.as_ref().unchecked_ref());
        }
        cb.forget();
    }

    // keyup → sincroniza cursor real com slot ativo
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |_: web_sys::KeyboardEvent| {
            update_slots(&root_cb);
        });
        if let Some(input) = query::first(&root, "[data-rs-input-otp]") {
            let _ = input.add_event_listener_with_callback("keyup", cb.as_ref().unchecked_ref());
        }
        cb.forget();
    }

    // focusin → marca slot ativo
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::new(move |_: web_sys::FocusEvent| {
            update_slots(&root_cb);
        });
        let _ = root.add_event_listener_with_callback("focusin", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // focusout → remove todos os active
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::new(move |_: web_sys::FocusEvent| {
            let slots = query::all(&root_cb, "[data-rs-input-otp-slot]");
            for slot in slots.iter() {
                state::remove_state(slot, "active");
            }
        });
        let _ = root.add_event_listener_with_callback("focusout", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    update_slots(&root);
}
