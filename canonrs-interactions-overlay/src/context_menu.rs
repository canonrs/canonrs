//! ContextMenu Interaction — Canon Rule #342
//! Toda a lógica vive aqui. Island apenas chama init_all().

use web_sys::HtmlElement;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-context-menu]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<HtmlElement>() { init_context_menu(el); }
        }
    }
    bind_click_outside(&doc);
}

fn init_context_menu(root: HtmlElement) {
    let root_clone = root.clone();
    let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
        let x = e.client_x();
        let y = e.client_y();
        set_open(&root_clone, true, x, y);
    }));
    let _ = root.dyn_ref::<web_sys::EventTarget>()
        .map(|et| et.add_event_listener_with_callback("contextmenu", cb.as_ref().unchecked_ref()));
    cb.forget();

    // fecha ao clicar em item
    let items = root.query_selector_all("[data-rs-context-menu-item]").ok();
    if let Some(list) = items {
        for i in 0..list.length() {
            if let Some(node) = list.item(i) {
                if let Ok(el) = node.dyn_into::<HtmlElement>() {
                    let root_clone = root.clone();
                    let disabled = el.get_attribute("aria-disabled").as_deref() == Some("true");
                    let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::MouseEvent| {
                        if !disabled { set_open(&root_clone, false, 0, 0); }
                    }));
                    let _ = el.dyn_ref::<web_sys::EventTarget>()
                        .map(|et| et.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()));
                    cb.forget();
                }
            }
        }
    }
}

fn set_open(root: &HtmlElement, open: bool, x: i32, y: i32) {
    let _ = root.set_attribute("data-rs-state", if open { "open" } else { "closed" });
    let content = root.query_selector("[data-rs-context-menu-content]").ok().flatten();
    if let Some(el) = content.and_then(|e| e.dyn_into::<HtmlElement>().ok()) {
        if open {
            let _ = el.style().set_property("left", &format!("{}px", x));
            let _ = el.style().set_property("top",  &format!("{}px", y));
            el.set_hidden(false);
            let _ = el.set_attribute("data-rs-state", "open");
        } else {
            el.set_hidden(true);
            let _ = el.set_attribute("data-rs-state", "closed");
        }
    }
}

fn bind_click_outside(doc: &web_sys::Document) {
    let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::MouseEvent| {
        let win = match web_sys::window() { Some(w) => w, None => return };
        let doc = match win.document() { Some(d) => d, None => return };
        let nodes = match doc.query_selector_all("[data-rs-context-menu][data-rs-state='open']") { Ok(n) => n, Err(_) => return };
        for i in 0..nodes.length() {
            if let Some(node) = nodes.item(i) {
                if let Ok(el) = node.dyn_into::<HtmlElement>() {
                    set_open(&el, false, 0, 0);
                }
            }
        }
    }));
    let _ = doc.dyn_ref::<web_sys::EventTarget>()
        .map(|et| et.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()));
    cb.forget();
}

pub fn init(root: web_sys::Element) {
    use wasm_bindgen::JsCast;
    if let Ok(el) = root.dyn_into::<web_sys::HtmlElement>() {
        init_context_menu(el);
    }
}
