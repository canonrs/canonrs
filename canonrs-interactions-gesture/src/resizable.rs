//! Resizable Interaction Engine

use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, PointerEvent};
use canonrs_interactions_core::dom::{state, attrs};
use canonrs_interactions_core::runtime::listeners;
use crate::runtime::drag;

pub fn init(root: Element) {
    let Ok(Some(hn)) = root.query_selector("[data-rs-resizable-handle]") else { return };
    let handle: Element = hn;

    if let Ok(nodes) = root.query_selector_all("[data-rs-resizable-panel]") {
        for i in 0..nodes.length() {
            if let Some(n) = nodes.item(i).and_then(|n| n.dyn_into::<HtmlElement>().ok()) {
                let sz = attrs::get_f64(&n, "data-rs-default-size", 50.0);
                let _ = n.style().set_property("--resizable-panel-basis", &format!("{}%", sz));
            }
        }
    }

    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    let uid_h = format!("{}:handle", uid);

    // pointerdown on handle — capture pointer
    listeners::listen(&uid_h, &handle, "pointerdown", {
        let root_c = root.clone();
        move |e: web_sys::Event| {
            let e = e.dyn_into::<PointerEvent>().unwrap();
            let Some(target) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(root_el) = target.closest("[data-rs-resizable]").ok().flatten()
                .and_then(|r| r.dyn_into::<HtmlElement>().ok()) else { return };
            if root_el.get_attribute("data-rs-drag-ptr").is_some() { return; }
            let orient = root_el.get_attribute("data-rs-orientation").unwrap_or_default();
            let rect = root_el.get_bounding_client_rect();
            let (size, offset) = if orient == "horizontal" { (rect.width(), rect.left()) } else { (rect.height(), rect.top()) };
            drag::set_drag(&root_c, e.pointer_id(), size, offset);
            if let Ok(hh) = target.clone().dyn_into::<HtmlElement>() {
                hh.set_pointer_capture(e.pointer_id()).ok();
            }
            state::add(&target, "active");
        }
    });

    // pointermove on handle — pointer capture routes here
    listeners::listen(&uid_h, &handle, "pointermove", {
        let root_c = root.clone();
        move |e: web_sys::Event| {
            let e = e.dyn_into::<PointerEvent>().unwrap();
            if !drag::drag_active(&root_c, e.pointer_id()) { return; }
            let size = drag::drag_size(&root_c);
            let offset = drag::drag_offset(&root_c);
            if size == 0.0 { return; }
            let orient = root_c.get_attribute("data-rs-orientation").unwrap_or_default();
            let min_s  = attrs::get_f64(&root_c, "data-rs-min-size", 20.0);
            let max_s  = attrs::get_f64(&root_c, "data-rs-max-size", 80.0);
            let pos = if orient == "horizontal" { e.client_x() as f64 } else { e.client_y() as f64 };
            let pct = ((pos - offset) / size * 100.0).max(min_s).min(max_s);
            let Ok(panels) = root_c.query_selector_all("[data-rs-resizable-panel]") else { return };
            let p0 = panels.item(0).and_then(|n| n.dyn_into::<HtmlElement>().ok());
            let p1 = panels.item(1).and_then(|n| n.dyn_into::<HtmlElement>().ok());
            let (Some(p0), Some(p1)) = (p0, p1) else { return };
            let _ = p0.style().set_property("--resizable-panel-basis", &format!("{}%", pct));
            let _ = p1.style().set_property("--resizable-panel-basis", &format!("{}%", 100.0 - pct));
        }
    });

    // pointerup on handle
    listeners::listen(&uid_h, &handle, "pointerup", {
        let root_c = root.clone();
        move |e: web_sys::Event| {
            let e = e.dyn_into::<PointerEvent>().unwrap();
            let Some(handle_el) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if !drag::drag_active(&root_c, e.pointer_id()) { return; }
            drag::clear_drag(&root_c);
            let _ = handle_el.remove_attribute("data-rs-state");
            if let Ok(h) = handle_el.dyn_into::<HtmlElement>() {
                let _ = h.release_pointer_capture(e.pointer_id());
            }
        }
    });

    // hover on all handles
    if let Ok(nodes) = root.query_selector_all("[data-rs-resizable-handle]") {
        for i in 0..nodes.length() {
            if let Some(h) = nodes.item(i).and_then(|n| n.dyn_into::<Element>().ok()) {
                let uid_hh = format!("{}:handle:{}", uid, i);
                listeners::listen(&uid_hh, &h, "mouseenter", move |e: web_sys::Event| {
                    let Some(t) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
                    state::add(&t, canonrs_interactions_core::dom::state::State::Hover.as_str());
                });
                listeners::listen(&uid_hh, &h, "mouseleave", move |e: web_sys::Event| {
                    let Some(t) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
                    state::remove(&t, canonrs_interactions_core::dom::state::State::Hover.as_str());
                });
            }
        }
    }
}
