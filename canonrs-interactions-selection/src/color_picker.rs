//! ColorPicker Interaction Engine

use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::{state, attrs};
use canonrs_interactions_core::runtime::listeners;
use crate::runtime::{context, popup};


fn set_open(root: &Element, open: bool) {
    if open { state::remove(root, canonrs_interactions_core::dom::state::State::Closed.as_str()); state::add(root, canonrs_interactions_core::dom::state::State::Open.as_str()); }
    else     { state::remove(root, canonrs_interactions_core::dom::state::State::Open.as_str());  state::add(root, canonrs_interactions_core::dom::state::State::Closed.as_str()); }
}

fn close_color_picker(root: &Element) {
    set_open(root, false);
}

fn update_swatch_color(root: &Element, value: &str) {
    if let Some(swatch) = attrs::query_one(root, "[data-rs-color-picker-trigger] [data-rs-color-swatch]") {
        let _ = swatch.set_attribute("data-rs-color", value);
        let _ = swatch.style().set_property("background-color", value);
    } else {
    }
    if let Some(trigger) = attrs::query_one(root, "[data-rs-color-picker-trigger]") {
        let _ = trigger.set_attribute("data-rs-color", value);
    }
    let _ = root.set_attribute("data-rs-value", value);
    // dispara rs-change para bridges DOM → signal
    let init = web_sys::CustomEventInit::new();
    init.set_bubbles(true);
    if let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict("rs-change", &init) {
        let _ = root.dispatch_event(&event);
    }
}

pub fn init(root: Element) {
    register();

    let _uid = root.get_attribute("data-rs-uid").unwrap_or_else(|| "NO-UID".to_string());
    let _is_swatches = root.has_attribute("data-rs-color-picker-swatches");

    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

    listeners::listen(&uid, &root, "click", move |e: web_sys::Event| {
        let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
        let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        let Some(rc) = context::find_root(&t, "[data-rs-color-picker]") else { return };
        if rc.get_attribute("data-rs-disabled").as_deref() == Some("true") { return; }
        if let Ok(Some(swatch_el)) = t.closest("[data-rs-color-swatch]") {
            if rc.has_attribute("data-rs-color-picker-swatches") {
                e.stop_propagation();
                let color = swatch_el.get_attribute("data-rs-color").unwrap_or_default();
                if let Ok(nodes) = rc.query_selector_all("[data-rs-color-swatch]") {
                    for i in 0..nodes.length() {
                        if let Some(n) = nodes.item(i).and_then(|n| n.dyn_into::<Element>().ok()) {
                            state::remove(&n, canonrs_interactions_core::dom::state::State::Selected.as_str());
                        }
                    }
                }
                state::add(&swatch_el, canonrs_interactions_core::dom::state::State::Selected.as_str());
                update_swatch_color(&rc, &color);
                return;
            }
        }
        if t.closest("[data-rs-color-picker-trigger]").ok().flatten().is_some() {
            if !rc.has_attribute("data-rs-color-picker-swatches") {
                e.stop_propagation();
                let o = state::has(&rc, canonrs_interactions_core::dom::state::State::Open.as_str());
                set_open(&rc, !o);
            }
        }
    });

    listeners::listen(&uid, &root, "input", move |e: web_sys::Event| {
        let Some(target) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) else { return };
        if target.get_attribute("data-rs-color-picker-input").is_none() { return; }
        let el = target.clone().dyn_into::<Element>().unwrap();
        let Some(rc) = context::find_root(&el, "[data-rs-color-picker]") else { return };
        let value = target.value();
        update_swatch_color(&rc, &value);
        if let Some(display) = attrs::query_one(&rc, "[data-rs-color-display-value]") {
            display.set_text_content(Some(&value));
            let _ = display.set_attribute("data-rs-color-value", &value);
        }
    });
}

pub fn register() {
    use std::cell::Cell;
    thread_local! { static REGISTERED: Cell<bool> = Cell::new(false); }
    REGISTERED.with(|r| {
        if r.get() { return; }
        r.set(true);
        popup::register_click_outside("[data-rs-color-picker]", close_color_picker);
    });
}


