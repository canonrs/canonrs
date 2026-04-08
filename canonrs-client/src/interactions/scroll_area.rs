//! ScrollArea Interaction Engine
//! Thumb sync, drag (pointer events), track click, auto-hide

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, PointerEvent, MouseEvent};
use std::cell::RefCell;
use std::rc::Rc;

fn add_state(el: &Element, state: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    if !current.split_whitespace().any(|s| s == state) {
        let next = if current.is_empty() { state.to_string() } else { format!("{} {}", current, state) };
        el.set_attribute("data-rs-state", &next).ok();
    }
}

fn remove_state(el: &Element, state: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    let next: Vec<&str> = current.split_whitespace().filter(|s| *s != state).collect();
    el.set_attribute("data-rs-state", &next.join(" ")).ok();
}

fn update_thumb(viewport: &HtmlElement, scrollbar: &Element, thumb: &HtmlElement, is_vertical: bool) {
    if is_vertical {
        let scroll_h   = viewport.scroll_height() as f64;
        let client_h   = viewport.client_height() as f64;
        let scroll_top = viewport.scroll_top() as f64;
        let bar_h      = scrollbar.client_height() as f64;
        if scroll_h <= client_h {
            add_state(&thumb.clone().into(), "hidden");
            return;
        }
        remove_state(&thumb.clone().into(), "hidden");
        let ratio     = client_h / scroll_h;
        let thumb_h   = (bar_h * ratio).max(40.0);
        let max_scroll = scroll_h - client_h;
        let thumb_top = if max_scroll > 0.0 { (scroll_top / max_scroll) * (bar_h - thumb_h) } else { 0.0 };
        thumb.style().set_property("--scroll-thumb-size",   &format!("{}px", thumb_h)).ok();
        thumb.style().set_property("--scroll-thumb-offset", &format!("{}px", thumb_top)).ok();
    } else {
        let scroll_w    = viewport.scroll_width() as f64;
        let client_w    = viewport.client_width() as f64;
        let scroll_left = viewport.scroll_left() as f64;
        let bar_w       = scrollbar.client_width() as f64;
        if scroll_w <= client_w {
            add_state(&thumb.clone().into(), "hidden");
            return;
        }
        remove_state(&thumb.clone().into(), "hidden");
        let ratio      = client_w / scroll_w;
        let thumb_w    = (bar_w * ratio).max(40.0);
        let max_scroll = scroll_w - client_w;
        let thumb_left = if max_scroll > 0.0 { (scroll_left / max_scroll) * (bar_w - thumb_w) } else { 0.0 };
        thumb.style().set_property("--scroll-thumb-size",   &format!("{}px", thumb_w)).ok();
        thumb.style().set_property("--scroll-thumb-offset", &format!("{}px", thumb_left)).ok();
    }
}

fn setup_scrollbar(root: &Element, viewport: &HtmlElement, orientation: &str) {
    let selector = format!("[data-rs-scrollbar][data-rs-orientation='{}']", orientation);
    let Ok(Some(scrollbar_node)) = root.query_selector(&selector) else { return };
    let Ok(scrollbar) = scrollbar_node.clone().dyn_into::<HtmlElement>() else { return };
    let thumb_sel = format!("[data-rs-scroll-thumb][data-rs-orientation='{}']", orientation);
    let Ok(Some(thumb_node)) = root.query_selector(&thumb_sel) else { return };
    let Ok(thumb) = thumb_node.dyn_into::<HtmlElement>() else { return };
    let is_vertical = orientation == "vertical";

    // initial sync
    update_thumb(viewport, &scrollbar_node, &thumb, is_vertical);

    // scroll sync
    {
        let vp = viewport.clone();
        let sb = scrollbar_node.clone();
        let th = thumb.clone();
        let on_scroll = Closure::wrap(Box::new(move |_: web_sys::Event| {
            update_thumb(&vp, &sb, &th, is_vertical);
        }) as Box<dyn FnMut(_)>);
        viewport.add_event_listener_with_callback("scroll", on_scroll.as_ref().unchecked_ref()).ok();
        on_scroll.forget();
    }

    // drag — pointer events on document (CR-339)
    {
        let is_dragging: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
        let active_ptr: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(None));
        let drag_state: Rc<RefCell<(f64, f64, f64, f64)>> = Rc::new(RefCell::new((0.0, 0.0, 0.0, 0.0)));

        let id_down = is_dragging.clone(); let id_move = is_dragging.clone(); let id_up = is_dragging.clone();
        let ap_down = active_ptr.clone();  let ap_move = active_ptr.clone();  let ap_up = active_ptr.clone();
        let ds_down = drag_state.clone();  let ds_move = drag_state.clone();

        let thumb_down = thumb.clone();
        let vp_down    = viewport.clone();
        let on_down = Closure::wrap(Box::new(move |e: PointerEvent| {
            e.prevent_default();
            let start_pos    = if is_vertical { e.client_y() as f64 } else { e.client_x() as f64 };
            let start_scroll = if is_vertical { vp_down.scroll_top() as f64 } else { vp_down.scroll_left() as f64 };
            let scroll_size  = if is_vertical { vp_down.scroll_height() as f64 } else { vp_down.scroll_width() as f64 };
            let client_size  = if is_vertical { vp_down.client_height() as f64 } else { vp_down.client_width() as f64 };
            *ds_down.borrow_mut() = (start_pos, start_scroll, scroll_size, client_size);
            *id_down.borrow_mut() = true;
            *ap_down.borrow_mut() = Some(e.pointer_id());
            thumb_down.set_pointer_capture(e.pointer_id()).ok();
            add_state(&thumb_down.clone().into(), "active");
        }) as Box<dyn FnMut(_)>);

        let vp_move   = viewport.clone();
        let on_move = Closure::wrap(Box::new(move |e: PointerEvent| {
            if !*id_move.borrow() { return; }
            if *ap_move.borrow() != Some(e.pointer_id()) { return; }
            let (start_pos, start_scroll, scroll_size, client_size) = *ds_move.borrow();
            let thumb_size = 40.0;
            let pos   = if is_vertical { e.client_y() as f64 } else { e.client_x() as f64 };
            let delta = pos - start_pos;
            let ratio = delta / (client_size - thumb_size);
            let new_scroll = start_scroll + ratio * (scroll_size - client_size);
            if is_vertical { vp_move.set_scroll_top(new_scroll as i32); }
            else           { vp_move.set_scroll_left(new_scroll as i32); }
        }) as Box<dyn FnMut(_)>);

        let thumb_up = thumb.clone();
        let on_up = Closure::wrap(Box::new(move |e: PointerEvent| {
            if *ap_up.borrow() == Some(e.pointer_id()) {
                *id_up.borrow_mut() = false;
                *ap_up.borrow_mut() = None;
                remove_state(&thumb_up.clone().into(), "active");
            }
        }) as Box<dyn FnMut(_)>);

        let win = web_sys::window().unwrap();
        let doc = win.document().unwrap();
        let doc_target: &web_sys::EventTarget = doc.as_ref();

        thumb.add_event_listener_with_callback("pointerdown", on_down.as_ref().unchecked_ref()).ok();
        doc_target.add_event_listener_with_callback("pointermove", on_move.as_ref().unchecked_ref()).ok();
        doc_target.add_event_listener_with_callback("pointerup",   on_up.as_ref().unchecked_ref()).ok();

        on_down.forget(); on_move.forget(); on_up.forget();
    }

    // track click — jump by page
    {
        let vp_click  = viewport.clone();
        let thumb_click = thumb.clone();
        let on_click = Closure::wrap(Box::new(move |e: MouseEvent| {
            let target: Element = e.target().unwrap().dyn_into().unwrap();
            if target.has_attribute("data-rs-scroll-thumb") { return; }
            let rect      = thumb_click.get_bounding_client_rect();
            let click_pos = if is_vertical { e.client_y() as f64 } else { e.client_x() as f64 };
            let thumb_mid = if is_vertical { rect.top() + rect.height() / 2.0 } else { rect.left() + rect.width() / 2.0 };
            let client_size = if is_vertical { vp_click.client_height() as f64 } else { vp_click.client_width() as f64 };
            let page      = client_size * 0.9;
            let direction = if click_pos > thumb_mid { 1.0 } else { -1.0 };
            if is_vertical { vp_click.set_scroll_top((vp_click.scroll_top() as f64 + direction * page) as i32); }
            else           { vp_click.set_scroll_left((vp_click.scroll_left() as f64 + direction * page) as i32); }
        }) as Box<dyn FnMut(_)>);
        scrollbar.add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref()).ok();
        on_click.forget();
    }
}

pub fn init(root: Element) {
    if root.get_attribute("data-rs-initialized").as_deref() == Some("true") { return; }
    let _ = root.set_attribute("data-rs-initialized", "true");

    let Ok(Some(vp_node)) = root.query_selector("[data-rs-scroll-viewport]") else { return };
    let Ok(viewport) = vp_node.dyn_into::<HtmlElement>() else { return };

    setup_scrollbar(&root, &viewport, "vertical");
    setup_scrollbar(&root, &viewport, "horizontal");
}

fn try_init_all(doc: &web_sys::Document) {
    let Ok(nodes) = doc.query_selector_all("[data-rs-scroll-area]") else { return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() {
                init(el);
            }
        }
    }
}

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    try_init_all(&doc);

    let doc_obs = doc.clone();
    let cb = Closure::wrap(Box::new(move |_: js_sys::Array, _: web_sys::MutationObserver| {
        try_init_all(&doc_obs);
    }) as Box<dyn FnMut(_, _)>);
    let observer = match web_sys::MutationObserver::new(cb.as_ref().unchecked_ref()) {
        Ok(o) => o, Err(_) => { cb.forget(); return }
    };
    let opts = web_sys::MutationObserverInit::new();
    opts.set_child_list(true);
    opts.set_subtree(true);
    if let Some(body) = doc.body() {
        observer.observe_with_options(&body, &opts).ok();
    }
    cb.forget();
    let obs_clone = observer.clone();
    let disconnect = Closure::wrap(Box::new(move || { obs_clone.disconnect(); }) as Box<dyn Fn()>);
    win.set_timeout_with_callback_and_timeout_and_arguments_0(disconnect.as_ref().unchecked_ref(), 5000).ok();
    disconnect.forget();
}
