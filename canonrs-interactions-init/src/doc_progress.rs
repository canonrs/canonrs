//! DocProgress Init — scroll tracking para [data-rs-doc-progress]

use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::runtime::{listeners, timers};

fn update_progress(root: &Element, container: Option<&web_sys::HtmlElement>) {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let (scroll_top, scroll_height, client_height) = if let Some(c) = container {
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
    let _ = root.set_attribute("data-rs-progress", &pct_str);
    let _ = root.set_attribute("aria-valuenow", &pct_str);
    let _ = root.set_attribute("aria-valuetext", &format!("{}% read", pct_str));
    if let Ok(root_el) = root.clone().dyn_into::<web_sys::HtmlElement>() {
        let _ = root_el.style().set_property("--progress", &pct_str);
    }
}

pub fn init(root: Element) {
    let scroll_target = root.get_attribute("data-rs-scroll-target").unwrap_or_default();
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };

    let container: Option<web_sys::HtmlElement> = if !scroll_target.is_empty() {
        doc.get_element_by_id(&scroll_target)
            .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
    } else {
        None
    };

    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

    // scroll listener — use RAF via timers::raf
    let scroll_target_el: web_sys::EventTarget = match &container {
        Some(c) => c.clone().unchecked_into(),
        None    => win.clone().unchecked_into(),
    };

    listeners::listen_opts(
        &uid,
        &scroll_target_el,
        "scroll",
        canonrs_interactions_core::runtime::listeners::ListenOpts { capture: false, passive: true },
        {
            let root_c = root.clone();
            let container_c = container.clone();
            move |_: web_sys::Event| {
                update_progress(&root_c, container_c.as_ref());
            }
        },
    );

    // initial update after DOM stable
    let root_c = root.clone();
    let container_c = container.clone();
    timers::timeout(100, move || {
        update_progress(&root_c, container_c.as_ref());
    });
}
