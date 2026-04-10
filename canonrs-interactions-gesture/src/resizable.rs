//! Resizable Interaction Engine

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, PointerEvent};
use crate::runtime::{lifecycle, drag, attrs};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    let Ok(Some(hn)) = root.query_selector("[data-rs-resizable-handle]") else { return };
    let Ok(handle) = hn.dyn_into::<HtmlElement>() else { return };

    if let Ok(nodes) = root.query_selector_all("[data-rs-resizable-panel]") {
        for i in 0..nodes.length() {
            if let Some(n) = nodes.item(i).and_then(|n| n.dyn_into::<HtmlElement>().ok()) {
                let sz = attrs::get_f64(&n, "data-rs-default-size", 50.0);
                let _ = n.style().set_property("--resizable-panel-basis", &format!("{}%", sz));
            }
        }
    }

    let doc = web_sys::window().and_then(|w| w.document()).unwrap();

    let cb_down = Closure::wrap(Box::new(move |e: PointerEvent| {
        let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        if target.closest("[data-rs-resizable-handle]").ok().flatten().is_none() { return; }
        let Some(root_el) = target.closest("[data-rs-resizable]").ok().flatten()
            .and_then(|r| r.dyn_into::<HtmlElement>().ok()) else { return };
        if root_el.get_attribute("data-rs-drag-ptr").is_some() { return; }
        let orient = root_el.get_attribute("data-rs-orientation").unwrap_or_default();
        let rect = root_el.get_bounding_client_rect();
        let (size, offset) = if orient == "horizontal" { (rect.width(), rect.left()) } else { (rect.height(), rect.top()) };
        drag::set_drag(&root_el, e.pointer_id(), size, offset);
        if let Some(h) = target.closest("[data-rs-resizable-handle]").ok().flatten() {
            if let Ok(hh) = h.clone().dyn_into::<HtmlElement>() {
                hh.set_pointer_capture(e.pointer_id()).ok();
            }
            let _ = h.set_attribute("data-rs-state", "active");
        }
    }) as Box<dyn FnMut(_)>);
    doc.add_event_listener_with_callback("pointerdown", cb_down.as_ref().unchecked_ref()).ok();
    cb_down.forget();

    let cb_move = Closure::wrap(Box::new(move |e: PointerEvent| {
        let Some(handle_el) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        let Some(root_el) = handle_el.closest("[data-rs-resizable]").ok().flatten() else { return };
        if !drag::drag_active(&root_el, e.pointer_id()) { return; }
        let size = drag::drag_size(&root_el); let offset = drag::drag_offset(&root_el);
        if size == 0.0 { return; }
        let orient = root_el.get_attribute("data-rs-orientation").unwrap_or_default();
        let min_s  = attrs::get_f64(&root_el, "data-rs-min-size", 20.0);
        let max_s  = attrs::get_f64(&root_el, "data-rs-max-size", 80.0);
        let pos = if orient == "horizontal" { e.client_x() as f64 } else { e.client_y() as f64 };
        let pct = ((pos - offset) / size * 100.0).max(min_s).min(max_s);
        let Ok(panels) = root_el.query_selector_all("[data-rs-resizable-panel]") else { return };
        let p0 = panels.item(0).and_then(|n| n.dyn_into::<HtmlElement>().ok());
        let p1 = panels.item(1).and_then(|n| n.dyn_into::<HtmlElement>().ok());
        let (Some(p0), Some(p1)) = (p0, p1) else { return };
        let _ = p0.style().set_property("--resizable-panel-basis", &format!("{}%", pct));
        let _ = p1.style().set_property("--resizable-panel-basis", &format!("{}%", 100.0 - pct));
    }) as Box<dyn FnMut(_)>);
    handle.add_event_listener_with_callback("pointermove", cb_move.as_ref().unchecked_ref()).ok();
    cb_move.forget();

    let cb_up = Closure::wrap(Box::new(move |e: PointerEvent| {
        let Some(handle_el) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        let Some(root_el) = handle_el.closest("[data-rs-resizable]").ok().flatten() else { return };
        if !drag::drag_active(&root_el, e.pointer_id()) { return; }
        drag::clear_drag(&root_el);
        let _ = handle_el.remove_attribute("data-rs-state");
        if let Ok(h) = handle_el.dyn_into::<HtmlElement>() {
            let _ = h.release_pointer_capture(e.pointer_id());
        }
    }) as Box<dyn FnMut(_)>);
    handle.add_event_listener_with_callback("pointerup", cb_up.as_ref().unchecked_ref()).ok();
    cb_up.forget();
}
