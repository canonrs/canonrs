//! DocProgress Init — scroll tracking para [data-rs-doc-progress]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::lifecycle;

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    let scroll_target = root.get_attribute("data-rs-scroll-target").unwrap_or_default();

    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };

    let container: Option<web_sys::HtmlElement> = if !scroll_target.is_empty() {
        doc.get_element_by_id(&scroll_target)
            .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
    } else {
        None
    };

    let update = {
        let root_cb = root.clone();
        let container = container.clone();
        move || {
            let win = match web_sys::window() { Some(w) => w, None => return };
            let doc = match win.document() { Some(d) => d, None => return };
            let (scroll_top, scroll_height, client_height) = if let Some(ref c) = container {
                (c.scroll_top() as f64, c.scroll_height() as f64, c.client_height() as f64)
            } else {
                let doc_el = match doc.document_element() { Some(d) => d, None => return };
                (
                    win.scroll_y().unwrap_or(0.0),
                    doc_el.scroll_height() as f64,
                    win.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(0.0),
                )
            };
            let max_scroll = scroll_height - client_height;
            if max_scroll <= 0.0 { return; }
            let pct = ((scroll_top / max_scroll) * 100.0).clamp(0.0, 100.0);
            let pct_str = format!("{:.0}", pct);
            let _ = root_cb.set_attribute("data-rs-progress", &pct_str);
            let _ = root_cb.set_attribute("aria-valuenow", &pct_str);
            let _ = root_cb.set_attribute("aria-valuetext", &format!("{}% read", pct_str));
            if let Ok(root_el) = root_cb.clone().dyn_into::<web_sys::HtmlElement>() {
                let _ = root_el.style().set_property("--progress", &pct_str);
            }
        }
    };

    let update_cb = update.clone();
    let handler = Closure::<dyn Fn()>::new(move || {
        let update_raf = update_cb.clone();
        if let Some(win) = web_sys::window() {
            let raf_cb = Closure::once(move || { update_raf(); });
            let _ = win.request_animation_frame(raf_cb.as_ref().unchecked_ref());
            raf_cb.forget();
        }
    });
    if let Some(ref c) = container {
        let _ = c.add_event_listener_with_callback("scroll", handler.as_ref().unchecked_ref());
    } else {
        let _ = win.add_event_listener_with_callback("scroll", handler.as_ref().unchecked_ref());
    }
    handler.forget();

    let init_cb = Closure::once(move || { update(); });
    let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(init_cb.as_ref().unchecked_ref(), 100);
    init_cb.forget();
}
