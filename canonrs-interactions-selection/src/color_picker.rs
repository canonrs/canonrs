//! ColorPicker Interaction Engine

use wasm_bindgen::prelude::*;
use crate::runtime::{lifecycle, state, context, popup, attrs};

use wasm_bindgen::JsCast;
use web_sys::Element;


fn set_open(root: &Element, open: bool) {
    if open { state::remove(root, "closed"); state::add(root, "open"); }
    else     { state::remove(root, "open");  state::add(root, "closed"); }
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
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    register();
    context::propagate_owner(&root);

    let uid = root.get_attribute("data-rs-uid").unwrap_or_else(|| "NO-UID".to_string());
    let is_swatches = root.has_attribute("data-rs-color-picker-swatches");

    // click handler — toggle popup (normal) ou swatch select (swatches mode)
    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else {
                return;
            };
            let Some(rc) = context::find_root(&t, "[data-rs-color-picker]") else {
                return;
            };

            // swatch clicado dentro do swatches row
            if let Ok(Some(swatch_el)) = t.closest("[data-rs-color-swatch]") {
                if rc.has_attribute("data-rs-color-picker-swatches") {
                    e.stop_propagation();
                    let color = swatch_el.get_attribute("data-rs-color").unwrap_or_default();
                    // deselect all swatches
                    if let Ok(nodes) = rc.query_selector_all("[data-rs-color-swatch]") {
                        for i in 0..nodes.length() {
                            if let Some(n) = nodes.item(i).and_then(|n| n.dyn_into::<Element>().ok()) {
                                state::remove(&n, "selected");
                            }
                        }
                    }
                    state::add(&swatch_el, "selected");
                    update_swatch_color(&rc, &color);
                    return;
                }
            }

            // trigger normal — abre/fecha popup
            if t.closest("[data-rs-color-picker-trigger]").ok().flatten().is_some() {
                if !rc.has_attribute("data-rs-color-picker-swatches") {
                    e.stop_propagation();
                    let o = state::has(&rc, "open");
                    set_open(&rc, !o);
                }
            }
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // input change → update swatch color (apenas no picker normal)
    {
        let cb = Closure::<dyn Fn(web_sys::Event)>::wrap(Box::new(move |e: web_sys::Event| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) else {
                return;
            };
            if target.get_attribute("data-rs-color-picker-input").is_none() {
                return;
            }
            let el = target.clone().dyn_into::<Element>().unwrap();
            let Some(rc) = context::find_root(&el, "[data-rs-color-picker]") else {
                return;
            };
            let value = target.value();
            update_swatch_color(&rc, &value);
            if let Some(display) = attrs::query_one(&rc, "[data-rs-color-display-value]") {
                display.set_text_content(Some(&value));
                let _ = display.set_attribute("data-rs-color-value", &value);
            } else {
            }
        }));
        let _ = root.add_event_listener_with_callback("input", cb.as_ref().unchecked_ref());
        cb.forget();
    }
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

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-color-picker]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init(el); }
        }
    }
}
