//! VirtualList Interaction Engine — scroll-driven virtualization

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};
use crate::shared::{is_initialized, mark_initialized};

fn find_scroll_viewport(el: &Element) -> Option<Element> {
    let mut current = el.parent_element();
    while let Some(e) = current {
        if e.has_attribute("data-rs-scroll-viewport") {
            return Some(e);
        }
        current = e.parent_element();
    }
    None
}

pub fn init(root: Element) {
    if is_initialized(&root) { return; }
    mark_initialized(&root);

    let items_count = root.get_attribute("data-items-count")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);
    let item_height = root.get_attribute("data-item-height")
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(40.0);

    let Some(viewport) = find_scroll_viewport(&root) else { return };
    let Ok(viewport_el) = viewport.clone().dyn_into::<HtmlElement>() else { return };

    let total_height = items_count as f64 * item_height;

    let doc = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d, None => return,
    };

    let content = match doc.create_element("div") {
        Ok(el) => el, Err(_) => return,
    };
    content.set_attribute("data-rs-virtual-list-content", "").ok();
    let content_el = content.clone().dyn_into::<HtmlElement>().unwrap();
    let _ = content_el.style().set_property("height", &format!("{}px", total_height));
    let _ = content_el.style().set_property("position", "relative");
    let _ = content_el.style().set_property("width", "100%");
    viewport_el.append_child(&content).ok();

    let render = {
        let vp = viewport_el.clone();
        let ct = content.clone();
        move || {
            let st = vp.scroll_top() as f64;
            let vh = vp.client_height() as f64;
            let start = ((st / item_height) as usize).saturating_sub(2);
            let end = (((st + vh) / item_height) as usize + 2).min(items_count);
            ct.set_inner_html("");
            let doc = match web_sys::window().and_then(|w| w.document()) {
                Some(d) => d, None => return,
            };
            for i in start..end {
                let Ok(el) = doc.create_element("div") else { continue };
                el.set_attribute("data-rs-virtual-list-item", "").ok();
                el.set_attribute("data-rs-index", &i.to_string()).ok();
                el.set_attribute("role", "listitem").ok();
                el.set_attribute("data-rs-state", "idle").ok();
                let Ok(el_h) = el.clone().dyn_into::<HtmlElement>() else { continue };
                let _ = el_h.style().set_property("position", "absolute");
                let _ = el_h.style().set_property("top", &format!("{}px", i as f64 * item_height));
                let _ = el_h.style().set_property("height", &format!("{}px", item_height));
                let _ = el_h.style().set_property("width", "100%");

                let el_enter = el.clone();
                let enter = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
                    let st = el_enter.get_attribute("data-rs-state").unwrap_or_default();
                    let mut parts: Vec<&str> = st.split_whitespace().collect();
                    if !parts.contains(&"hover") {
                        parts.push("hover");
                        el_enter.set_attribute("data-rs-state", &parts.join(" ")).ok();
                    }
                });
                let el_leave = el.clone();
                let leave = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
                    let st = el_leave.get_attribute("data-rs-state").unwrap_or_default();
                    let next: Vec<&str> = st.split_whitespace().filter(|s| *s != "hover").collect();
                    el_leave.set_attribute("data-rs-state", &next.join(" ")).ok();
                });
                let _ = el_h.add_event_listener_with_callback("mouseenter", enter.as_ref().unchecked_ref());
                let _ = el_h.add_event_listener_with_callback("mouseleave", leave.as_ref().unchecked_ref());
                enter.forget();
                leave.forget();

                el.set_text_content(Some(&format!("Item {}", i + 1)));
                ct.append_child(&el).ok();
            }
        }
    };

    render();

    let render_cb = Closure::<dyn Fn()>::new(render);
    let _ = viewport_el.add_event_listener_with_callback_and_bool(
        "scroll", render_cb.as_ref().unchecked_ref(), false,
    );
    render_cb.forget();
}

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-virtual-list]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init(el); }
        }
    }
}
