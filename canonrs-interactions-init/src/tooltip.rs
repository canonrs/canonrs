//! Tooltip Init

use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::state;
use canonrs_interactions_core::runtime::{listeners, timers};

fn get_delay(root: &Element, attr: &str, default: i32) -> i32 {
    let mut el = root.parent_element();
    loop {
        match el {
            Some(ref e) if e.has_attribute("data-rs-tooltip-provider") => {
                return e.get_attribute(attr).and_then(|v| v.parse::<i32>().ok()).unwrap_or(default);
            }
            Some(ref e) => { el = e.parent_element(); }
            None => break,
        }
    }
    default
}

fn close_siblings(root: &Element) {
    let doc = match web_sys::window().and_then(|w| w.document()) { Some(d) => d, None => return };
    if let Ok(nodes) = doc.query_selector_all("[data-rs-tooltip]") {
        for i in 0..nodes.length() {
            if let Some(el) = nodes.item(i).and_then(|n| n.dyn_into::<web_sys::Element>().ok()) {
                if el != *root {
                    if let Ok(Some(c)) = el.query_selector("[data-rs-tooltip-content]") { state::close(&c); }
                    state::close(&el);
                }
            }
        }
    }
}

fn open_content(root: &Element) {
    close_siblings(root);
    if let Ok(Some(c)) = root.query_selector("[data-rs-tooltip-content]") { state::open(&c); }
    state::open(root);
}

fn close_content(root: &Element) {
    if let Ok(Some(c)) = root.query_selector("[data-rs-tooltip-content]") { state::close(&c); }
    state::close(root);
}

pub fn init(root: Element) {
    let delay_open  = get_delay(&root, "data-rs-delay-open",  400);
    let delay_close = get_delay(&root, "data-rs-delay-close", 100);
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

    listeners::listen(&uid, &root, "mouseenter", {
        let root_c = root.clone();
        move |_: web_sys::Event| {
            let r = root_c.clone();
            timers::timeout(delay_open, move || { open_content(&r); });
        }
    });
    listeners::listen(&uid, &root, "mouseleave", {
        let root_c = root.clone();
        move |_: web_sys::Event| {
            let r = root_c.clone();
            timers::timeout(delay_close, move || { close_content(&r); });
        }
    });
    listeners::listen(&uid, &root, "focusin", {
        let root_c = root.clone();
        move |_: web_sys::Event| { open_content(&root_c); }
    });
    listeners::listen(&uid, &root, "focusout", {
        let root_c = root.clone();
        move |_: web_sys::Event| { close_content(&root_c); }
    });
}
