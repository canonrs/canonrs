//! TableOfContents Init — anchor click + scroll spy

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, query};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    for link in query::all(&root, "[data-rs-toc-link]") {
        let link_cb = link.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            e.prevent_default();
            let href = link_cb.get_attribute("href").unwrap_or_default();
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
}
