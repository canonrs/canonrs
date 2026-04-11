//! Toolbar Interaction Engine — roving tabindex + action dispatch

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::lifecycle;

fn get_items(root: &Element) -> Vec<Element> {
    let mut result = Vec::new();
    let mut i = 0u32;
    loop {
        let selector = format!("[data-rs-toolbar-item]:nth-of-type({})", i + 1);
        match root.query_selector(&selector) {
            Ok(Some(el)) => { result.push(el); i += 1; }
            _ => break,
        }
    }
    result
}

pub fn init(root: Element) {
    if lifecycle::is_initialized(&root) { return; }
    lifecycle::mark_initialized(&root);
    let is_vertical = root.get_attribute("data-rs-orientation").as_deref() == Some("vertical");

    // roving tabindex init
    let items = get_items(&root);
    for (i, item) in items.iter().enumerate() {
        item.set_attribute("tabindex", if i == 0 { "0" } else { "-1" }).ok();
    }

    // keydown — roving tabindex only
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-toolbar-item]").ok().flatten().is_none() { return; }
            let items = get_items(&root_cb);
            let len = items.len();
            if len == 0 { return; }
            let Some(pos) = items.iter().position(|el| el.contains(Some(&target))) else { return };
            let next = match e.key().as_str() {
                "ArrowRight" if !is_vertical => { e.prevent_default(); Some((pos + 1) % len) }
                "ArrowLeft"  if !is_vertical => { e.prevent_default(); Some((pos + len - 1) % len) }
                "ArrowDown"  if  is_vertical => { e.prevent_default(); Some((pos + 1) % len) }
                "ArrowUp"    if  is_vertical => { e.prevent_default(); Some((pos + len - 1) % len) }
                "Home" => { e.prevent_default(); Some(0) }
                "End"  => { e.prevent_default(); Some(len - 1) }
                _ => None,
            };
            if let Some(next_idx) = next {
                for (i, item) in items.iter().enumerate() {
                    item.set_attribute("tabindex", if i == next_idx { "0" } else { "-1" }).ok();
                }
                if let Ok(el) = items[next_idx].clone().dyn_into::<web_sys::HtmlElement>() {
                    el.focus().ok();
                }
            }
        }));
        root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }

    // click — toggle aria-pressed + dispatch CustomEvent
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(item) = target.closest("[data-rs-toolbar-item]").ok().flatten() else { return };
            let pressed = item.get_attribute("aria-pressed").as_deref() == Some("true");
            let next = if pressed { "false" } else { "true" };
            item.set_attribute("aria-pressed", next).ok();
            let value = item.get_attribute("data-rs-value").unwrap_or_default();
            let script = format!(
                "arguments[0].dispatchEvent(new CustomEvent('canon:toolbar:action',{{bubbles:true,detail:{{value:'{}',pressed:{}}}}}));",
                value, next == "true"
            );
            let f = js_sys::Function::new_with_args("arguments", &script);
            f.call1(&JsValue::NULL, &root_cb).ok();
        }));
        root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }
}

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-toolbar]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init(el); }
        }
    }
}
