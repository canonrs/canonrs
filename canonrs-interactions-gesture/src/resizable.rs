//! Resizable Interaction Engine

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, PointerEvent};
use crate::runtime::lifecycle;

fn find_drag_root(doc: &web_sys::Document, ptr_id: i32) -> Option<Element> {
    let nodes = doc.query_selector_all("[data-rs-resizable][data-rs-drag-ptr]").ok()?;
    for i in 0..nodes.length() {
        let el = nodes.item(i)?.dyn_into::<Element>().ok()?;
        let ptr = el.get_attribute("data-rs-drag-ptr").and_then(|s| s.parse::<i32>().ok()).unwrap_or(-1);
        if ptr == ptr_id { return Some(el); }
    }
    None
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { web_sys::console::log_1(&"[R] BLOCKED".into()); return; }

    let orientation = root.get_attribute("data-rs-orientation").unwrap_or_else(|| "horizontal".to_string());
    let min_size: f64 = root.get_attribute("data-rs-min-size").and_then(|s| s.parse().ok()).unwrap_or(20.0);
    let max_size: f64 = root.get_attribute("data-rs-max-size").and_then(|s| s.parse().ok()).unwrap_or(80.0);
    let is_horizontal: bool = orientation == "horizontal";

    if let Ok(nodes) = root.query_selector_all("[data-rs-resizable-panel]") {
        for i in 0..nodes.length() {
            if let Some(n) = nodes.item(i).and_then(|n| n.dyn_into::<HtmlElement>().ok()) {
                let sz = n.get_attribute("data-rs-default-size").and_then(|s| s.parse::<f64>().ok()).unwrap_or(50.0);
                let _ = n.style().set_property("--resizable-panel-basis", &format!("{}%", sz));
            }
        }
    }

    let Ok(Some(hn)) = root.query_selector("[data-rs-resizable-handle]") else { return };
    let Ok(handle) = hn.dyn_into::<HtmlElement>() else { return };
    let doc = web_sys::window().and_then(|w| w.document()).unwrap();

    // pointerdown via document delegation
    let cb_down = Closure::wrap(Box::new(move |e: PointerEvent| {
        let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        if target.closest("[data-rs-resizable-handle]").ok().flatten().is_none() { return; }
        let Some(root_el) = target.closest("[data-rs-resizable]").ok().flatten()
            .and_then(|r| r.dyn_into::<HtmlElement>().ok()) else { return };
        let rect = root_el.get_bounding_client_rect();
        let root_orient = root_el.get_attribute("data-rs-orientation").unwrap_or_default();
        let (size, offset) = if root_orient == "horizontal" { (rect.width(), rect.left()) } else { (rect.height(), rect.top()) };
        // so setar se nao ha drag ativo
        if root_el.get_attribute("data-rs-drag-ptr").is_some() { return; }
        let _ = root_el.set_attribute("data-rs-drag-ptr", &e.pointer_id().to_string());
        let _ = root_el.set_attribute("data-rs-drag-size", &size.to_string());
        let _ = root_el.set_attribute("data-rs-drag-offset", &offset.to_string());
    }) as Box<dyn FnMut(_)>);
    doc.add_event_listener_with_callback("pointerdown", cb_down.as_ref().unchecked_ref()).ok();
    cb_down.forget();

    // pointermove
    let cb_move = Closure::wrap(Box::new(move |e: PointerEvent| {
        let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
        let Some(root_el) = find_drag_root(&doc, e.pointer_id()) else { return };
        let size   = root_el.get_attribute("data-rs-drag-size").and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
        let offset = root_el.get_attribute("data-rs-drag-offset").and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
        if size == 0.0 { return; }
        let orient = root_el.get_attribute("data-rs-orientation").unwrap_or_default();
        let pos = if orient == "horizontal" { e.client_x() as f64 } else { e.client_y() as f64 };
        let orient2 = root_el.get_attribute("data-rs-orientation").unwrap_or_default();
        let min_s = root_el.get_attribute("data-rs-min-size").and_then(|s| s.parse::<f64>().ok()).unwrap_or(20.0);
        let max_s = root_el.get_attribute("data-rs-max-size").and_then(|s| s.parse::<f64>().ok()).unwrap_or(80.0);
        let _ = orient2;
        let pct = ((pos - offset) / size * 100.0).max(min_s).min(max_s);
        let Ok(panels) = root_el.query_selector_all("[data-rs-resizable-panel]") else { return };
        let p0 = panels.item(0).and_then(|n| n.dyn_into::<HtmlElement>().ok());
        let p1 = panels.item(1).and_then(|n| n.dyn_into::<HtmlElement>().ok());
        let (Some(p0), Some(p1)) = (p0, p1) else { return };
        let _ = p0.style().set_property("--resizable-panel-basis", &format!("{}%", pct));
        let _ = p1.style().set_property("--resizable-panel-basis", &format!("{}%", 100.0 - pct));
    }) as Box<dyn FnMut(_)>);
    doc.add_event_listener_with_callback("pointermove", cb_move.as_ref().unchecked_ref()).ok();
    cb_move.forget();

    // pointerup
    let cb_up = Closure::wrap(Box::new(move |e: PointerEvent| {
        let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
        let Some(root_el) = find_drag_root(&doc, e.pointer_id()) else { return };
        let _ = root_el.remove_attribute("data-rs-drag-ptr");
        let _ = root_el.remove_attribute("data-rs-drag-size");
        let _ = root_el.remove_attribute("data-rs-drag-offset");
    }) as Box<dyn FnMut(_)>);
    doc.add_event_listener_with_callback("pointerup", cb_up.as_ref().unchecked_ref()).ok();
    cb_up.forget();
}
