//! ScrollArea Interaction Engine

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, PointerEvent, MouseEvent};
use crate::runtime::{lifecycle, state, drag};

fn sync_thumb(root: &Element, orientation: &str) {
    let is_v = orientation == "vertical";
    let sb_sel = if is_v { "[data-rs-scrollbar][data-rs-orientation='vertical']" }
                 else    { "[data-rs-scrollbar][data-rs-orientation='horizontal']" };
    let th_sel = if is_v { "[data-rs-scroll-thumb][data-rs-orientation='vertical']" }
                 else    { "[data-rs-scroll-thumb][data-rs-orientation='horizontal']" };

    let Ok(Some(vp_el)) = root.query_selector("[data-rs-scroll-viewport]") else { return };
    let Ok(vp) = vp_el.dyn_into::<HtmlElement>() else { return };
    let Ok(Some(sb)) = root.query_selector(sb_sel) else { return };
    let Ok(Some(th_el)) = root.query_selector(th_sel) else { return };
    let Ok(th) = th_el.dyn_into::<HtmlElement>() else { return };

    let (scroll_size, client_size, scroll_pos, bar_size) = if is_v {
        (vp.scroll_height() as f64, vp.client_height() as f64, vp.scroll_top() as f64, sb.client_height() as f64)
    } else {
        (vp.scroll_width() as f64, vp.client_width() as f64, vp.scroll_left() as f64, sb.client_width() as f64)
    };

    if scroll_size <= client_size { state::add(&th.clone().into(), "hidden"); return; }
    state::remove(&th.clone().into(), "hidden");

    let ratio = client_size / scroll_size;
    let thumb_size = (bar_size * ratio).max(40.0);
    let max_scroll = scroll_size - client_size;
    let thumb_offset = if max_scroll > 0.0 { (scroll_pos / max_scroll) * (bar_size - thumb_size) } else { 0.0 };
    let _ = th.style().set_property("--scroll-thumb-size", &format!("{}px", thumb_size));
    let _ = th.style().set_property("--scroll-thumb-offset", &format!("{}px", thumb_offset));
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    sync_thumb(&root, "vertical");
    sync_thumb(&root, "horizontal");

    let doc = web_sys::window().and_then(|w| w.document()).unwrap();

    // scroll sync
    {
        let cb = Closure::wrap(Box::new(move |e: web_sys::Event| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if !target.has_attribute("data-rs-scroll-viewport") { return; }
            let Some(root_el) = target.closest("[data-rs-scroll-area]").ok().flatten() else { return };
            sync_thumb(&root_el, "vertical");
            sync_thumb(&root_el, "horizontal");
        }) as Box<dyn FnMut(_)>);
        doc.add_event_listener_with_callback_and_bool("scroll", cb.as_ref().unchecked_ref(), true).ok();
        cb.forget();
    }

    // pointerdown — inicia drag no thumb
    {
        let cb = Closure::wrap(Box::new(move |e: PointerEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if !target.has_attribute("data-rs-scroll-thumb") { return; }
            let is_v = target.get_attribute("data-rs-orientation").as_deref() == Some("vertical");
            let Some(root_el) = target.closest("[data-rs-scroll-area]").ok().flatten() else { return };
            let Some(vp) = root_el.query_selector("[data-rs-scroll-viewport]").ok().flatten()
                .and_then(|n| n.dyn_into::<HtmlElement>().ok()) else { return };
            e.prevent_default();
            let start_scroll = if is_v { vp.scroll_top() as f64 } else { vp.scroll_left() as f64 };
            let scroll_size  = if is_v { vp.scroll_height() as f64 } else { vp.scroll_width() as f64 };
            let client_size  = if is_v { vp.client_height() as f64 } else { vp.client_width() as f64 };
            let start_pos    = if is_v { e.client_y() as f64 } else { e.client_x() as f64 };
            // reutiliza drag::set_drag: ptr_id=pointer_id, size=scroll_size, offset=start_pos
            // start_scroll e client_size vão em atributos extras
            drag::set_drag(&target, e.pointer_id(), scroll_size, start_pos);
            let _ = target.set_attribute("data-rs-drag-start-scroll", &start_scroll.to_string());
            let _ = target.set_attribute("data-rs-drag-client-size",  &client_size.to_string());
            if let Some(h) = target.dyn_ref::<HtmlElement>() { h.set_pointer_capture(e.pointer_id()).ok(); }
            state::add(&target, "active");
        }) as Box<dyn FnMut(_)>);
        doc.add_event_listener_with_callback("pointerdown", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }

    // pointermove — arrasta thumb
    {
        let cb = Closure::wrap(Box::new(move |e: PointerEvent| {
            let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
            let Ok(Some(thumb)) = doc.query_selector("[data-rs-scroll-thumb][data-rs-state~='active']") else { return };
            if !drag::drag_active(&thumb, e.pointer_id()) { return; }
            let is_v = thumb.get_attribute("data-rs-orientation").as_deref() == Some("vertical");
            let start_pos    = drag::drag_offset(&thumb);
            let scroll_size  = drag::drag_size(&thumb);
            let start_scroll = thumb.get_attribute("data-rs-drag-start-scroll").and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
            let client_size  = thumb.get_attribute("data-rs-drag-client-size").and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
            let Some(root_el) = thumb.closest("[data-rs-scroll-area]").ok().flatten() else { return };
            let Some(vp) = root_el.query_selector("[data-rs-scroll-viewport]").ok().flatten()
                .and_then(|n| n.dyn_into::<HtmlElement>().ok()) else { return };
            let pos = if is_v { e.client_y() as f64 } else { e.client_x() as f64 };
            let delta = pos - start_pos;
            let ratio = delta / (client_size - 40.0);
            let new_scroll = start_scroll + ratio * (scroll_size - client_size);
            if is_v { vp.set_scroll_top(new_scroll as i32); } else { vp.set_scroll_left(new_scroll as i32); }
        }) as Box<dyn FnMut(_)>);
        doc.add_event_listener_with_callback("pointermove", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }

    // pointerup — encerra drag
    {
        let cb = Closure::wrap(Box::new(move |e: PointerEvent| {
            let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
            let Ok(Some(thumb)) = doc.query_selector("[data-rs-scroll-thumb][data-rs-state~='active']") else { return };
            if !drag::drag_active(&thumb, e.pointer_id()) { return; }
            drag::clear_drag(&thumb);
            state::remove(&thumb, "active");
            if let Ok(h) = thumb.dyn_into::<HtmlElement>() { let _ = h.release_pointer_capture(e.pointer_id()); }
        }) as Box<dyn FnMut(_)>);
        doc.add_event_listener_with_callback("pointerup", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }

    // click na track — page scroll
    {
        let cb = Closure::wrap(Box::new(move |e: MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if !target.has_attribute("data-rs-scrollbar") { return; }
            let is_v = target.get_attribute("data-rs-orientation").as_deref() == Some("vertical");
            let Some(root_el) = target.closest("[data-rs-scroll-area]").ok().flatten() else { return };
            let th_sel = if is_v { "[data-rs-scroll-thumb][data-rs-orientation='vertical']" }
                         else    { "[data-rs-scroll-thumb][data-rs-orientation='horizontal']" };
            let Some(th) = root_el.query_selector(th_sel).ok().flatten()
                .and_then(|n| n.dyn_into::<HtmlElement>().ok()) else { return };
            let Some(vp) = root_el.query_selector("[data-rs-scroll-viewport]").ok().flatten()
                .and_then(|n| n.dyn_into::<HtmlElement>().ok()) else { return };
            let rect = th.get_bounding_client_rect();
            let click_pos = if is_v { e.client_y() as f64 } else { e.client_x() as f64 };
            let thumb_mid = if is_v { rect.top() + rect.height()/2.0 } else { rect.left() + rect.width()/2.0 };
            let client_size = if is_v { vp.client_height() as f64 } else { vp.client_width() as f64 };
            let dir = if click_pos > thumb_mid { 1.0 } else { -1.0 };
            if is_v { vp.set_scroll_top((vp.scroll_top() as f64 + dir * client_size * 0.9) as i32); }
            else    { vp.set_scroll_left((vp.scroll_left() as f64 + dir * client_size * 0.9) as i32); }
        }) as Box<dyn FnMut(_)>);
        doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }
}
