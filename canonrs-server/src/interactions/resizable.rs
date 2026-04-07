//! Resizable Interaction Engine

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, PointerEvent};
use std::cell::RefCell;
use std::rc::Rc;

fn add_state(el: &Element, state: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    if !current.split_whitespace().any(|s| s == state) {
        let next = if current.is_empty() { state.to_string() } else { format!("{} {}", current, state) };
        el.set_attribute("data-rs-state", &next).ok();
    }
}

fn remove_state(el: &Element, state: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    let next: Vec<&str> = current.split_whitespace().filter(|s| *s != state).collect();
    el.set_attribute("data-rs-state", &next.join(" ")).ok();
}

pub fn init(root: Element) {
    web_sys::console::log_1(&"[resizable] init called".into());
    // idempotency via JS property (not DOM attribute)
    let key = JsValue::from_str("__rs_resizable_bound");
    let root_val = JsCast::unchecked_ref::<JsValue>(&root);
    if js_sys::Reflect::get(root_val, &key).ok().filter(|v| v.is_truthy()).is_some() { return; }
    js_sys::Reflect::set(root_val, &key, &JsValue::TRUE).ok();

    let orientation = root.get_attribute("data-rs-orientation").unwrap_or_else(|| "horizontal".to_string());
    let min_size = root.get_attribute("data-rs-min-size").and_then(|s| s.parse::<f64>().ok()).unwrap_or(20.0);
    let max_size = root.get_attribute("data-rs-max-size").and_then(|s| s.parse::<f64>().ok()).unwrap_or(80.0);
    let is_horizontal = orientation == "horizontal";

    // init panels
    if let Ok(panels) = root.query_selector_all("[data-rs-resizable-panel]") {
        for i in 0..panels.length() {
            if let Some(node) = panels.item(i) {
                if let Ok(el) = node.dyn_into::<HtmlElement>() {
                    let size = el.get_attribute("data-rs-default-size")
                        .and_then(|s| s.parse::<f64>().ok())
                        .unwrap_or(50.0);
                    el.style().set_property("flex", &format!("0 0 {}%", size)).ok();
                }
            }
        }
    }

    let Ok(Some(hn)) = root.query_selector("[data-rs-resizable-handle]") else { return };
    let Ok(handle) = hn.dyn_into::<HtmlElement>() else { return };

    let panels_qs = match root.query_selector_all("[data-rs-resizable-panel]") { Ok(n) => n, Err(_) => return };
    web_sys::console::log_1(&format!("[resizable] panels={}", panels_qs.length()).into());
    let Some(p0n) = panels_qs.item(0) else { web_sys::console::log_1(&"[resizable] no p0".into()); return };
    let Some(p1n) = panels_qs.item(1) else { return };
    let Ok(p0) = p0n.dyn_into::<HtmlElement>() else { return };
    let Ok(p1) = p1n.dyn_into::<HtmlElement>() else { return };

    let is_dragging: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
    let active_pointer: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(None));
    let container_rect: Rc<RefCell<(f64, f64)>> = Rc::new(RefCell::new((0.0, 0.0)));

    let id_down = is_dragging.clone(); let id_move = is_dragging.clone(); let id_up = is_dragging.clone();
    let ap_down = active_pointer.clone(); let ap_move = active_pointer.clone(); let ap_up = active_pointer.clone();
    let cr_down = container_rect.clone(); let cr_move = container_rect.clone();

    let root_down = root.clone();
    let handle_down = handle.clone();
    let on_down = Closure::wrap(Box::new(move |e: PointerEvent| {
        web_sys::console::log_1(&"[resizable] on_down fired".into());
        e.prevent_default();
        let Ok(c2) = root_down.clone().dyn_into::<HtmlElement>() else { return };
        let r2 = c2.get_bounding_client_rect();
        web_sys::console::log_1(&format!("[resizable] container rect w={} h={} l={} t={}", r2.width(), r2.height(), r2.left(), r2.top()).into());
        let Ok(container) = root_down.clone().dyn_into::<HtmlElement>() else { return };
        let rect = container.get_bounding_client_rect();
        let size   = if is_horizontal { rect.width()  } else { rect.height() };
        let offset = if is_horizontal { rect.left()   } else { rect.top()    };
        *cr_down.borrow_mut() = (size, offset);
        *id_down.borrow_mut() = true;
        *ap_down.borrow_mut() = Some(e.pointer_id());
        handle_down.set_pointer_capture(e.pointer_id()).ok();
        add_state(&handle_down.clone().into(), "active");
        remove_state(&handle_down.clone().into(), "inactive");
    }) as Box<dyn FnMut(_)>);

    let p0_move = p0.clone();
    let p1_move = p1.clone();
    let on_move = Closure::wrap(Box::new(move |e: PointerEvent| {
        web_sys::console::log_1(&format!("[resizable] on_move dragging={}", *id_move.borrow()).into());
        if !*id_move.borrow() { return; }
        if *ap_move.borrow() != Some(e.pointer_id()) { return; }
        let (size, offset) = *cr_move.borrow();
        if size == 0.0 { return; }
        let pos = if is_horizontal { e.client_x() as f64 } else { e.client_y() as f64 };
        let pct = ((pos - offset) / size * 100.0).max(min_size).min(max_size);
web_sys::console::log_1(&format!("[resizable] pct={}", pct).into());
        p0_move.style().set_property("flex", &format!("0 0 {}%", pct)).ok();
        p1_move.style().set_property("flex", &format!("0 0 {}%", 100.0 - pct)).ok();
    }) as Box<dyn FnMut(_)>);

    let handle_up = handle.clone();
    let on_up = Closure::wrap(Box::new(move |e: PointerEvent| {
        if *ap_up.borrow() == Some(e.pointer_id()) {
            *id_up.borrow_mut() = false;
            *ap_up.borrow_mut() = None;
            add_state(&handle_up.clone().into(), "inactive");
            remove_state(&handle_up.clone().into(), "active");
        }
    }) as Box<dyn FnMut(_)>);

    handle.add_event_listener_with_callback("pointerdown", on_down.as_ref().unchecked_ref()).ok();
    handle.add_event_listener_with_callback("pointermove", on_move.as_ref().unchecked_ref()).ok();
    handle.add_event_listener_with_callback("pointerup",   on_up.as_ref().unchecked_ref()).ok();

    on_down.forget();
    on_move.forget();
    on_up.forget();
}

fn try_init_all(doc: &web_sys::Document) {
    web_sys::console::log_1(&"[resizable] try_init_all".into());
    let containers = match doc.query_selector_all("[data-rs-resizable]") { Ok(n) => n, Err(_) => return };
    for i in 0..containers.length() {
        if let Some(node) = containers.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() {
                init(el);
            }
        }
    }
}

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };

    // try immediately
    try_init_all(&doc);

    // observe DOM for late-hydrated islands
    let doc_obs = doc.clone();
    let cb = Closure::wrap(Box::new(move |_: js_sys::Array, _: web_sys::MutationObserver| {
        try_init_all(&doc_obs);
    }) as Box<dyn FnMut(_, _)>);

    let observer = match web_sys::MutationObserver::new(cb.as_ref().unchecked_ref()) {
        Ok(o) => o,
        Err(_) => { cb.forget(); return },
    };

    let opts = web_sys::MutationObserverInit::new();
    opts.set_child_list(true);
    opts.set_subtree(true);

    if let Some(body) = doc.body() {
        observer.observe_with_options(&body, &opts).ok();
    }

    cb.forget();
    // disconnect after 5s
    let obs_clone = observer.clone();
    let win2 = win.clone();
    let disconnect = Closure::wrap(Box::new(move || {
        obs_clone.disconnect();
    }) as Box<dyn Fn()>);
    win2.set_timeout_with_callback_and_timeout_and_arguments_0(disconnect.as_ref().unchecked_ref(), 5000).ok();
    disconnect.forget();
}
