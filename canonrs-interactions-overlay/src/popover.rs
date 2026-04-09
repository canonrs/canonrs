//! Popover Interaction Engine

use wasm_bindgen::prelude::*;
use crate::shared::{add_state, remove_state, is_initialized, mark_initialized};
use wasm_bindgen::JsCast;
use web_sys::Element;

fn is_open(root: &Element) -> bool {
    root.get_attribute("data-rs-state")
        .map(|s| s.split_whitespace().any(|t| t == "open"))
        .unwrap_or(false)
}

fn auto_position(root: &Element) {
    let Some(win) = web_sys::window() else { return };
    let Ok(Some(content)) = root.query_selector("[data-rs-popover-content]") else { return };
    let root_rect   = root.get_bounding_client_rect();
    let content_rect = content.get_bounding_client_rect();
    let viewport_h  = win.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(0.0);
    let viewport_w  = win.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(0.0);
    let content_h   = if content_rect.height() > 0.0 { content_rect.height() } else { 200.0 };
    let content_w   = if content_rect.width() > 0.0 { content_rect.width() } else { 200.0 };
    let space_below = viewport_h - root_rect.bottom();
    let space_above = root_rect.top();
    let space_right = viewport_w - root_rect.left();

    let side = if space_below >= content_h + 8.0 {
        "bottom"
    } else if space_above >= content_h + 8.0 {
        "top"
    } else if space_right >= content_w + 8.0 {
        "right"
    } else {
        "top"
    };

    let _ = content.set_attribute("data-rs-side", side);
}

fn open(root: &Element) {
    auto_position(root);
    remove_state(root, "closed");
    add_state(root, "open");
}

fn close(root: &Element) { remove_state(root, "open"); add_state(root, "closed"); }

fn each_popover<F: Fn(Element)>(doc: &web_sys::Document, f: F) {
    let Ok(nodes) = doc.query_selector_all("[data-rs-popover][data-rs-initialized='true']") else { return };
    for i in 0..nodes.length() {
        let Some(raw) = nodes.item(i) else { continue };
        let Ok(node)  = raw.dyn_into::<Element>() else { continue };
        f(node);
    }
}

pub fn init(root: Element) {
    if is_initialized(&root) { return; }
    mark_initialized(&root);

    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(current) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(target)  = e.target().and_then(|t| t.dyn_ref::<Element>().cloned()) else { return };
            if target.closest("[data-rs-popover-trigger]").ok().flatten().is_some() {
                if is_open(&current) { close(&current); } else { open(&current); }
            } else if target.closest("[data-rs-popover-close]").ok().flatten().is_some() {
                close(&current);
            }
        });
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_ref::<Element>().cloned()) else { return };
            let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
            each_popover(&doc, |node| {
                if !node.contains(Some(target.as_ref())) && is_open(&node) { close(&node); }
            });
        });
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let _ = doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        }
        cb.forget();
    }

    {
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
            if e.key() != "Escape" { return; }
            let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
            each_popover(&doc, |node| { if is_open(&node) { close(&node); } });
        });
        if let Some(win) = web_sys::window() {
            let _ = win.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        }
        cb.forget();
    }
}
