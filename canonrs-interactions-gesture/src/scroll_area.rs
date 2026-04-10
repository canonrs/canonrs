//! ScrollArea Interaction Engine

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, PointerEvent, MouseEvent};
use crate::runtime::{lifecycle, state, drag};
use std::cell::RefCell;
use std::rc::Rc;

fn update_thumb(viewport: &HtmlElement, scrollbar: &Element, thumb: &HtmlElement, is_vertical: bool) {
    let (scroll_size, client_size, scroll_pos, bar_size) = if is_vertical {
        (viewport.scroll_height() as f64, viewport.client_height() as f64,
         viewport.scroll_top() as f64, scrollbar.client_height() as f64)
    } else {
        (viewport.scroll_width() as f64, viewport.client_width() as f64,
         viewport.scroll_left() as f64, scrollbar.client_width() as f64)
    };

    if scroll_size <= client_size { state::add(&thumb.clone().into(), "hidden"); return; }
    state::remove(&thumb.clone().into(), "hidden");

    let ratio = client_size / scroll_size;
    let thumb_size = (bar_size * ratio).max(40.0);
    let max_scroll = scroll_size - client_size;
    let thumb_offset = if max_scroll > 0.0 { (scroll_pos / max_scroll) * (bar_size - thumb_size) } else { 0.0 };

    let prop_size   = if is_vertical { "--scroll-thumb-size"   } else { "--scroll-thumb-size"   };
    let prop_offset = if is_vertical { "--scroll-thumb-offset" } else { "--scroll-thumb-offset" };
    let _ = thumb.style().set_property(prop_size,   &format!("{}px", thumb_size));
    let _ = thumb.style().set_property(prop_offset, &format!("{}px", thumb_offset));
}

fn setup_scrollbar(root: &Element, viewport: &HtmlElement, orientation: &str) {
    let sb_sel = format!("[data-rs-scrollbar][data-rs-orientation='{}']", orientation);
    let Ok(Some(sb_node)) = root.query_selector(&sb_sel) else { return };
    let Ok(scrollbar) = sb_node.clone().dyn_into::<HtmlElement>() else { return };
    let th_sel = format!("[data-rs-scroll-thumb][data-rs-orientation='{}']", orientation);
    let Ok(Some(th_node)) = root.query_selector(&th_sel) else { return };
    let Ok(thumb) = th_node.dyn_into::<HtmlElement>() else { return };
    let is_vertical = orientation == "vertical";

    update_thumb(viewport, &sb_node, &thumb, is_vertical);

    {
        let vp = viewport.clone(); let sb = sb_node.clone(); let th = thumb.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if !state::is_valid(&vp.clone().into()) { return; }
            update_thumb(&vp, &sb, &th, is_vertical);
        }) as Box<dyn FnMut(_)>);
        viewport.add_event_listener_with_callback("scroll", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }

    let ptr = drag::new();
    let drag_data: Rc<RefCell<(f64, f64, f64, f64)>> = Rc::new(RefCell::new((0.0, 0.0, 0.0, 0.0)));
    let doc = web_sys::window().and_then(|w| w.document()).unwrap();

    {
        let p = ptr.clone(); let dd = drag_data.clone();
        let vp = viewport.clone(); let th = thumb.clone();
        let cb = Closure::wrap(Box::new(move |e: PointerEvent| {
            e.prevent_default();
            let start_pos    = if is_vertical { e.client_y() as f64 } else { e.client_x() as f64 };
            let start_scroll = if is_vertical { vp.scroll_top() as f64 } else { vp.scroll_left() as f64 };
            let scroll_size  = if is_vertical { vp.scroll_height() as f64 } else { vp.scroll_width() as f64 };
            let client_size  = if is_vertical { vp.client_height() as f64 } else { vp.client_width() as f64 };
            *dd.borrow_mut() = (start_pos, start_scroll, scroll_size, client_size);
            drag::start(&p, e.pointer_id());
            th.set_pointer_capture(e.pointer_id()).ok();
            state::add(&th.clone().into(), "active");
        }) as Box<dyn FnMut(_)>);
        thumb.add_event_listener_with_callback("pointerdown", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }

    {
        let p = ptr.clone(); let dd = drag_data.clone(); let vp = viewport.clone();
        let cb = Closure::wrap(Box::new(move |e: PointerEvent| {
            if !drag::is_active(&p, e.pointer_id()) { return; }
            if !state::is_valid(&vp.clone().into()) { return; }
            let (start_pos, start_scroll, scroll_size, client_size) = *dd.borrow();
            let pos = if is_vertical { e.client_y() as f64 } else { e.client_x() as f64 };
            let delta = pos - start_pos;
            let ratio = delta / (client_size - 40.0);
            let new_scroll = start_scroll + ratio * (scroll_size - client_size);
            if is_vertical { vp.set_scroll_top(new_scroll as i32); }
            else           { vp.set_scroll_left(new_scroll as i32); }
        }) as Box<dyn FnMut(_)>);
        doc.add_event_listener_with_callback("pointermove", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }

    {
        let p = ptr.clone(); let th = thumb.clone();
        let cb = Closure::wrap(Box::new(move |e: PointerEvent| {
            if !drag::is_active(&p, e.pointer_id()) { return; }
            drag::stop(&p);
            state::remove(&th.clone().into(), "active");
        }) as Box<dyn FnMut(_)>);
        doc.add_event_listener_with_callback("pointerup", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }

    {
        let vp = viewport.clone(); let th = thumb.clone();
        let cb = Closure::wrap(Box::new(move |e: MouseEvent| {
            if !state::is_valid(&vp.clone().into()) { return; }
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if t.has_attribute("data-rs-scroll-thumb") { return; }
            let rect = th.get_bounding_client_rect();
            let click_pos = if is_vertical { e.client_y() as f64 } else { e.client_x() as f64 };
            let thumb_mid = if is_vertical { rect.top()+rect.height()/2.0 } else { rect.left()+rect.width()/2.0 };
            let client_size = if is_vertical { vp.client_height() as f64 } else { vp.client_width() as f64 };
            let dir = if click_pos > thumb_mid { 1.0 } else { -1.0 };
            if is_vertical { vp.set_scroll_top((vp.scroll_top() as f64 + dir * client_size * 0.9) as i32); }
            else           { vp.set_scroll_left((vp.scroll_left() as f64 + dir * client_size * 0.9) as i32); }
        }) as Box<dyn FnMut(_)>);
        scrollbar.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    let Ok(Some(vp_node)) = root.query_selector("[data-rs-scroll-viewport]") else { return };
    let Ok(viewport) = vp_node.dyn_into::<HtmlElement>() else { return };
    setup_scrollbar(&root, &viewport, "vertical");
    setup_scrollbar(&root, &viewport, "horizontal");
}
