//! Markdown Init — scroll spy, anchor nav, TOC sync

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state, query};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    // smooth scroll nos anchors internos
    for link in query::all(&root, "[data-rs-md-a][href^='#']") {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            e.prevent_default();
            let Some(t) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let href = t.get_attribute("href").unwrap_or_default();
            if href.starts_with('#') {
                let id = &href[1..];
                let win = match web_sys::window() { Some(w) => w, None => return };
                let doc = match win.document() { Some(d) => d, None => return };
                if let Ok(Some(target)) = doc.query_selector(&format!("#{}", id)) {
                    let rect = target.get_bounding_client_rect();
                    let scroll_y = win.scroll_y().unwrap_or(0.0);
                    let top = rect.top() + scroll_y - 80.0;
                    let opts = web_sys::ScrollToOptions::new();
                    opts.set_top(top);
                    opts.set_behavior(web_sys::ScrollBehavior::Smooth);
                    win.scroll_to_with_scroll_to_options(&opts);
                }
            }
        });
        let _ = link.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // hover nos links
    for link in query::all(&root, "[data-rs-md-a]") {
        let l_enter = link.clone();
        let enter = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
            state::add_state(&l_enter, "hover");
        });
        let l_leave = link.clone();
        let leave = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
            state::remove_state(&l_leave, "hover");
        });
        let _ = link.add_event_listener_with_callback("mouseenter", enter.as_ref().unchecked_ref());
        let _ = link.add_event_listener_with_callback("mouseleave", leave.as_ref().unchecked_ref());
        enter.forget();
        leave.forget();
    }
}
