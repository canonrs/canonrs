//! InputOtp Init

use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::{query, state};
use canonrs_interactions_core::runtime::listeners;

fn update_slots(root: &Element) {
    let input = match query::first(root, "[data-rs-input-otp]") { Some(el) => el, None => return };
    let input_el = match input.clone().dyn_into::<web_sys::HtmlInputElement>() { Ok(el) => el, Err(_) => return };
    let value = input_el.value();
    let slots = query::all(root, "[data-rs-input-otp-slot]");
    let cursor = value.len().min(slots.len().saturating_sub(1).max(0));
    for (i, slot) in slots.iter().enumerate() {
        let inner = slot.query_selector("[data-rs-slot-inner]").ok().flatten();
        let ch = value.chars().nth(i).map(|c| c.to_string()).unwrap_or_default();
        if let Some(inner) = inner { inner.set_text_content(Some(&ch)); }
        if i == cursor { state::add_state(slot, "active"); } else { state::remove_state(slot, "active"); }
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
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

    listeners::listen(&uid, &root, "click", {
        let root_c = root.clone();
        move |_: web_sys::Event| { focus_input_at_end(&root_c); }
    });

    if let Some(input) = query::first(&root, "[data-rs-input-otp]") {
        let uid_input = format!("{}:input", uid);
        listeners::listen(&uid_input, &input, "input", {
            let root_c = root.clone();
            move |_: web_sys::Event| { update_slots(&root_c); }
        });
        let uid_keyup = format!("{}:keyup", uid);
        listeners::listen(&uid_keyup, &input, "keyup", {
            let root_c = root.clone();
            move |_: web_sys::Event| { update_slots(&root_c); }
        });
    }

    listeners::listen(&uid, &root, "focusin", {
        let root_c = root.clone();
        move |_: web_sys::Event| { update_slots(&root_c); }
    });

    listeners::listen(&uid, &root, "focusout", {
        let root_c = root.clone();
        move |_: web_sys::Event| {
            for slot in query::all(&root_c, "[data-rs-input-otp-slot]") {
                state::remove_state(&slot, "active");
            }
        }
    });

    update_slots(&root);
}
