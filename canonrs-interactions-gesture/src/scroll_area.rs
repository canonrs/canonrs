//! ScrollArea Interaction Engine

use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, PointerEvent};
use canonrs_interactions_core::dom::state;
use canonrs_interactions_core::runtime::listeners;
use crate::runtime::drag;

fn sync_thumb(root: &Element, orientation: &str) {
    let is_v = orientation == "vertical";
    let sb_sel = if is_v { "[data-rs-scrollbar][data-rs-orientation='vertical']" }
                 else    { "[data-rs-scrollbar][data-rs-orientation='horizontal']" };
    let th_sel = if is_v { "[data-rs-scroll-thumb][data-rs-orientation='vertical']" }
                 else    { "[data-rs-scroll-thumb][data-rs-orientation='horizontal']" };

    let Ok(Some(vp_el)) = root.query_selector("[data-rs-scroll-viewport]") else { return };
    let Ok(vp) = vp_el.dyn_into::<HtmlElement>() else { return };
    let Ok(Some(_sb)) = root.query_selector(sb_sel) else { return };
    let Ok(Some(th_el)) = root.query_selector(th_sel) else { return };
    let Ok(th) = th_el.dyn_into::<HtmlElement>() else { return };

    let (scroll_size, client_size, scroll_pos, bar_size) = if is_v {
        let sb = root.query_selector(sb_sel).ok().flatten().unwrap();
        (vp.scroll_height() as f64, vp.client_height() as f64, vp.scroll_top() as f64, sb.client_height() as f64)
    } else {
        let sb = root.query_selector(sb_sel).ok().flatten().unwrap();
        (vp.scroll_width() as f64, vp.client_width() as f64, vp.scroll_left() as f64, sb.client_width() as f64)
    };

    if scroll_size <= client_size {
        state::add(&th.clone().into(), canonrs_interactions_core::dom::state::State::Hidden.as_str());
        return;
    }
    state::remove(&th.clone().into(), canonrs_interactions_core::dom::state::State::Hidden.as_str());

    let ratio = client_size / scroll_size;
    let thumb_size = (bar_size * ratio).max(40.0);
    let max_scroll = scroll_size - client_size;
    let thumb_offset = if max_scroll > 0.0 { (scroll_pos / max_scroll) * (bar_size - thumb_size) } else { 0.0 };
    let _ = th.style().set_property("--scroll-thumb-size", &format!("{}px", thumb_size));
    let _ = th.style().set_property("--scroll-thumb-offset", &format!("{}px", thumb_offset));
}

pub fn init(root: Element) {
    sync_thumb(&root, "vertical");
    sync_thumb(&root, "horizontal");

    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

    // scroll sync — on viewport scroll event
    {
        let root_c = root.clone();
        let vp_target = root.query_selector("[data-rs-scroll-viewport]")
            .ok().flatten()
            .unwrap_or_else(|| root.clone());
        listeners::listen_opts(
            &uid,
            &vp_target.unchecked_into::<web_sys::EventTarget>(),
            "scroll",
            canonrs_interactions_core::runtime::listeners::ListenOpts { capture: true, passive: false },
            move |_: web_sys::Event| {
                sync_thumb(&root_c, "vertical");
                sync_thumb(&root_c, "horizontal");
            }
        );
    }

    // Get thumb elements for pointer capture
    let thumb_v = root.query_selector("[data-rs-scroll-thumb][data-rs-orientation='vertical']").ok().flatten();
    let thumb_h = root.query_selector("[data-rs-scroll-thumb][data-rs-orientation='horizontal']").ok().flatten();

    let init_thumb = |thumb: Element| {
        let uid_t = format!("{}:scroll-thumb", uid);

        // pointerdown on thumb — capture pointer
        listeners::listen(&uid_t, &thumb, "pointerdown", {
            let root_c = root.clone();
            move |e: web_sys::Event| {
                let e = e.dyn_into::<PointerEvent>().unwrap();
                let Some(target) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
                let is_v = target.get_attribute("data-rs-orientation").as_deref() == Some("vertical");
                let Some(vp) = root_c.query_selector("[data-rs-scroll-viewport]").ok().flatten()
                    .and_then(|n| n.dyn_into::<HtmlElement>().ok()) else { return };
                e.prevent_default();
                let start_scroll = if is_v { vp.scroll_top() as f64 } else { vp.scroll_left() as f64 };
                let scroll_size  = if is_v { vp.scroll_height() as f64 } else { vp.scroll_width() as f64 };
                let client_size  = if is_v { vp.client_height() as f64 } else { vp.client_width() as f64 };
                let start_pos    = if is_v { e.client_y() as f64 } else { e.client_x() as f64 };
                drag::set_drag(&target, e.pointer_id(), scroll_size, start_pos);
                let _ = target.set_attribute("data-rs-drag-start-scroll", &start_scroll.to_string());
                let _ = target.set_attribute("data-rs-drag-client-size",  &client_size.to_string());
                if let Some(h) = target.dyn_ref::<HtmlElement>() { h.set_pointer_capture(e.pointer_id()).ok(); }
                state::add(&target, canonrs_interactions_core::dom::state::State::Active.as_str());
            }
        });

        // pointermove on thumb — pointer capture routes here
        listeners::listen(&uid_t, &thumb, "pointermove", {
            let root_c = root.clone();
            move |e: web_sys::Event| {
                let e = e.dyn_into::<PointerEvent>().unwrap();
                let Some(thumb_el) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
                if !drag::drag_active(&thumb_el, e.pointer_id()) { return; }
                let is_v = thumb_el.get_attribute("data-rs-orientation").as_deref() == Some("vertical");
                let start_pos    = drag::drag_offset(&thumb_el);
                let scroll_size  = drag::drag_size(&thumb_el);
                let start_scroll = thumb_el.get_attribute("data-rs-drag-start-scroll").and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                let client_size  = thumb_el.get_attribute("data-rs-drag-client-size").and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                let Some(vp) = root_c.query_selector("[data-rs-scroll-viewport]").ok().flatten()
                    .and_then(|n| n.dyn_into::<HtmlElement>().ok()) else { return };
                let pos = if is_v { e.client_y() as f64 } else { e.client_x() as f64 };
                let delta = pos - start_pos;
                let ratio = delta / (client_size - 40.0);
                let new_scroll = start_scroll + ratio * (scroll_size - client_size);
                if is_v { vp.set_scroll_top(new_scroll as i32); } else { vp.set_scroll_left(new_scroll as i32); }
            }
        });

        // pointerup on thumb
        listeners::listen(&uid_t, &thumb, "pointerup", move |e: web_sys::Event| {
            let e = e.dyn_into::<PointerEvent>().unwrap();
            let Some(thumb_el) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if !drag::drag_active(&thumb_el, e.pointer_id()) { return; }
            drag::clear_drag(&thumb_el);
            state::remove(&thumb_el, canonrs_interactions_core::dom::state::State::Active.as_str());
            if let Ok(h) = thumb_el.dyn_into::<HtmlElement>() { let _ = h.release_pointer_capture(e.pointer_id()); }
        });
    };

    if let Some(th) = thumb_v { init_thumb(th); }
    if let Some(th) = thumb_h { init_thumb(th); }

    // click on track — page scroll
    listeners::listen(&uid, &root, "click", move |e: web_sys::Event| {
        let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
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
    });
}
