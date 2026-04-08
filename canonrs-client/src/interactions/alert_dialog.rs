//! AlertDialog Interaction — Canon Rule #342
//! Toda a lógica vive aqui. Island apenas chama init_all().

use web_sys::HtmlElement;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-alert-dialog]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<HtmlElement>() { init_alert_dialog(el); }
        }
    }
    bind_escape();
}

fn init_alert_dialog(root: HtmlElement) {
    if let Some(trigger) = root.query_selector("[data-rs-alert-dialog-trigger]").ok().flatten() {
        let root_clone = root.clone();
        let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::MouseEvent| {
            set_open(&root_clone, true);
        }));
        let _ = trigger.dyn_ref::<web_sys::EventTarget>()
            .map(|et| et.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()));
        cb.forget();
    }
    for selector in ["[data-rs-alert-dialog-overlay]", "[data-rs-alert-dialog-cancel]"] {
        if let Some(el) = root.query_selector(selector).ok().flatten() {
            let root_clone = root.clone();
            let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::MouseEvent| {
                set_open(&root_clone, false);
            }));
            let _ = el.dyn_ref::<web_sys::EventTarget>()
                .map(|et| et.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()));
            cb.forget();
        }
    }
    if let Some(confirm) = root.query_selector("[data-rs-alert-dialog-confirm]").ok().flatten() {
        let root_clone = root.clone();
        let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::MouseEvent| {
            set_open(&root_clone, false);
        }));
        let _ = confirm.dyn_ref::<web_sys::EventTarget>()
            .map(|et| et.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()));
        cb.forget();
    }
}

fn set_open(root: &HtmlElement, open: bool) {
    let _ = root.set_attribute("data-rs-state", if open { "open" } else { "closed" });
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    if let Some(body) = doc.body() {
        if open { let _ = body.style().set_property("overflow", "hidden"); }
        else    { let _ = body.style().remove_property("overflow"); }
    }
}

fn bind_escape() {
    let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
        if e.key() != "Escape" { return; }
        let win = match web_sys::window() { Some(w) => w, None => return };
        let doc = match win.document() { Some(d) => d, None => return };
        let nodes = match doc.query_selector_all("[data-rs-alert-dialog][data-rs-state='open']") { Ok(n) => n, Err(_) => return };
        for i in 0..nodes.length() {
            if let Some(node) = nodes.item(i) {
                if let Ok(el) = node.dyn_into::<HtmlElement>() { set_open(&el, false); }
            }
        }
    }));
    let win = match web_sys::window() { Some(w) => w, None => return };
    let _ = win.dyn_ref::<web_sys::EventTarget>()
        .map(|et| et.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref()));
    cb.forget();
}
