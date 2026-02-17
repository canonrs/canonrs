//! ScrollArea Behavior - Custom scrollbar enterprise
//! Thumb sync, drag, auto-hide, vertical + horizontal

#[cfg(feature = "hydrate")]
use super::register_behavior;
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;
#[cfg(feature = "hydrate")]
use leptos::leptos_dom::helpers::document;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-scroll-area", Box::new(|element_id, _state| {
        let Some(root) = document().get_element_by_id(element_id) else { return Ok(()); };
        let Some(viewport) = root.query_selector("[data-scroll-viewport]").ok().flatten() else { return Ok(()); };
        let viewport_el: web_sys::HtmlElement = viewport.clone().dyn_into().unwrap();

        setup_scrollbar(&root, &viewport_el, "vertical")?;
        setup_scrollbar(&root, &viewport_el, "horizontal")?;

        Ok(())
    }));
}

#[cfg(feature = "hydrate")]
fn setup_scrollbar(root: &web_sys::Element, viewport: &web_sys::HtmlElement, orientation: &str) -> BehaviorResult<()> {
    let selector = format!("[data-scrollbar][data-orientation='{}']", orientation);
    let Some(scrollbar) = root.query_selector(&selector).ok().flatten() else { return Ok(()); };
    let thumb_sel = format!("[data-scroll-thumb][data-orientation='{}']", orientation);
    let Some(thumb) = root.query_selector(&thumb_sel).ok().flatten() else { return Ok(()); };
    // track para click-to-jump
    let track_sel = format!("[data-scrollbar][data-orientation='{}'] [data-scroll-track]", orientation);
    let track = root.query_selector(&track_sel).ok().flatten();
    let thumb_el: web_sys::HtmlElement = thumb.clone().dyn_into().unwrap();
    let is_vertical = orientation == "vertical";

    // Sync thumb on scroll
    {
        let viewport_c  = viewport.clone();
        let scrollbar_c = scrollbar.clone();
        let thumb_c     = thumb_el.clone();

        let on_scroll = Closure::wrap(Box::new(move |_: web_sys::Event| {
            update_thumb(&viewport_c, &scrollbar_c, &thumb_c, is_vertical);
        }) as Box<dyn FnMut(_)>);

        viewport.add_event_listener_with_callback("scroll", on_scroll.as_ref().unchecked_ref())
            .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "scroll listener".into() })?;
        on_scroll.forget();
    }

    // Initial thumb size
    update_thumb(viewport, &scrollbar, &thumb_el, is_vertical);

    // Drag thumb
    setup_drag(&thumb_el, viewport, is_vertical)?;

    // Click on track → jump
    if let Some(ref track_el) = track {
        setup_track_click(track_el, viewport, &thumb_el, is_vertical)?;
    }

    Ok(())
}

#[cfg(feature = "hydrate")]
fn update_thumb(viewport: &web_sys::HtmlElement, scrollbar: &web_sys::Element, thumb: &web_sys::HtmlElement, is_vertical: bool) {
    if is_vertical {
        let scroll_h   = viewport.scroll_height() as f64;
        let client_h   = viewport.client_height() as f64;
        let scroll_top = viewport.scroll_top() as f64;
        let bar_h      = scrollbar.client_height() as f64;

        if scroll_h <= client_h {
            let _ = thumb.set_attribute("data-state", "hidden");
            return;
        }
        let _ = thumb.remove_attribute("data-state");

        let ratio      = client_h / scroll_h;
        let thumb_h    = (bar_h * ratio).max(40.0);
        let max_scroll = scroll_h - client_h;
        let thumb_top  = if max_scroll > 0.0 { (scroll_top / max_scroll) * (bar_h - thumb_h) } else { 0.0 };
        let ratio_pos  = if max_scroll > 0.0 { scroll_top / max_scroll } else { 0.0 };

        // CSS custom properties — zero layout thrash
        thumb.unchecked_ref::<web_sys::HtmlElement>().style()
            .set_property("--scroll-thumb-size", &format!("{}px", thumb_h)).ok();
        thumb.unchecked_ref::<web_sys::HtmlElement>().style()
            .set_property("--scroll-thumb-offset", &format!("{}px", thumb_top)).ok();

        // Dispatch canon:scroll
        dispatch_scroll_event(viewport, scroll_top, 0.0, max_scroll, 0.0, ratio_pos, 0.0);
    } else {
        let scroll_w    = viewport.scroll_width() as f64;
        let client_w    = viewport.client_width() as f64;
        let scroll_left = viewport.scroll_left() as f64;
        let bar_w       = scrollbar.client_width() as f64;

        if scroll_w <= client_w {
            let _ = thumb.set_attribute("data-state", "hidden");
            return;
        }
        let _ = thumb.remove_attribute("data-state");

        let ratio      = client_w / scroll_w;
        let thumb_w    = (bar_w * ratio).max(40.0);
        let max_scroll = scroll_w - client_w;
        let thumb_left = if max_scroll > 0.0 { (scroll_left / max_scroll) * (bar_w - thumb_w) } else { 0.0 };
        let ratio_pos  = if max_scroll > 0.0 { scroll_left / max_scroll } else { 0.0 };

        thumb.unchecked_ref::<web_sys::HtmlElement>().style()
            .set_property("--scroll-thumb-size", &format!("{}px", thumb_w)).ok();
        thumb.unchecked_ref::<web_sys::HtmlElement>().style()
            .set_property("--scroll-thumb-offset", &format!("{}px", thumb_left)).ok();

        dispatch_scroll_event(viewport, 0.0, scroll_left, 0.0, max_scroll, 0.0, ratio_pos);
    }
}

#[cfg(feature = "hydrate")]
fn dispatch_scroll_event(viewport: &web_sys::HtmlElement, scroll_top: f64, scroll_left: f64, max_v: f64, max_h: f64, ratio_v: f64, ratio_h: f64) {
    let detail = js_sys::Object::new();
    js_sys::Reflect::set(&detail, &JsValue::from_str("scrollTop"),   &JsValue::from_f64(scroll_top)).ok();
    js_sys::Reflect::set(&detail, &JsValue::from_str("scrollLeft"),  &JsValue::from_f64(scroll_left)).ok();
    js_sys::Reflect::set(&detail, &JsValue::from_str("maxScrollV"),  &JsValue::from_f64(max_v)).ok();
    js_sys::Reflect::set(&detail, &JsValue::from_str("maxScrollH"),  &JsValue::from_f64(max_h)).ok();
    js_sys::Reflect::set(&detail, &JsValue::from_str("ratioV"),      &JsValue::from_f64(ratio_v)).ok();
    js_sys::Reflect::set(&detail, &JsValue::from_str("ratioH"),      &JsValue::from_f64(ratio_h)).ok();

    let f = js_sys::Function::new_with_args(
        "el,detail",
        "el.dispatchEvent(new CustomEvent('canon:scroll', {bubbles:true, detail:detail}));"
    );
    f.call2(&JsValue::NULL, viewport, &detail).ok();
}

#[cfg(feature = "hydrate")]
fn setup_drag(thumb: &web_sys::HtmlElement, viewport: &web_sys::HtmlElement, is_vertical: bool) -> BehaviorResult<()> {
    let thumb_c    = thumb.clone();
    let viewport_c = viewport.clone();

    let on_mousedown = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
        e.prevent_default();
        let start_pos    = if is_vertical { e.client_y() as f64 } else { e.client_x() as f64 };
        let start_scroll = if is_vertical { viewport_c.scroll_top() as f64 } else { viewport_c.scroll_left() as f64 };
        let scroll_size  = if is_vertical { viewport_c.scroll_height() as f64 } else { viewport_c.scroll_width() as f64 };
        let client_size  = if is_vertical { viewport_c.client_height() as f64 } else { viewport_c.client_width() as f64 };
        let thumb_size   = if is_vertical { thumb_c.offset_height() as f64 } else { thumb_c.offset_width() as f64 };
        let bar_size     = client_size;

        thumb_c.set_attribute("data-state", "dragging").ok();

        let viewport_move = viewport_c.clone();
        let thumb_move    = thumb_c.clone();

        let on_mousemove = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let pos   = if is_vertical { e.client_y() as f64 } else { e.client_x() as f64 };
            let delta = pos - start_pos;
            let ratio = delta / (bar_size - thumb_size);
            let new_scroll = start_scroll + ratio * (scroll_size - client_size);
            if is_vertical {
                viewport_move.set_scroll_top(new_scroll as i32);
            } else {
                viewport_move.set_scroll_left(new_scroll as i32);
            }
        }) as Box<dyn FnMut(_)>);

        let thumb_up = thumb_c.clone();
        let on_mouseup = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            thumb_up.set_attribute("data-state", "idle").ok();
        }) as Box<dyn FnMut(_)>);

        let doc = document();
        doc.add_event_listener_with_callback("mousemove", on_mousemove.as_ref().unchecked_ref()).ok();
        doc.add_event_listener_with_callback("mouseup",   on_mouseup.as_ref().unchecked_ref()).ok();
        on_mousemove.forget();
        on_mouseup.forget();
    }) as Box<dyn FnMut(_)>);

    thumb.add_event_listener_with_callback("mousedown", on_mousedown.as_ref().unchecked_ref())
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "mousedown listener".into() })?;
    on_mousedown.forget();
    Ok(())
}

#[cfg(feature = "hydrate")]
fn setup_track_click(scrollbar: &web_sys::Element, viewport: &web_sys::HtmlElement, thumb: &web_sys::HtmlElement, is_vertical: bool) -> BehaviorResult<()> {
    let viewport_c = viewport.clone();
    let thumb_c    = thumb.clone();

    let on_click = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
        let target: web_sys::Element = e.target().unwrap().dyn_into().unwrap();
        if target.has_attribute("data-scroll-thumb") { return; }

        let rect       = thumb_c.get_bounding_client_rect();
        let click_pos  = if is_vertical { e.client_y() as f64 } else { e.client_x() as f64 };
        let thumb_mid  = if is_vertical { rect.top() + rect.height() / 2.0 } else { rect.left() + rect.width() / 2.0 };
        let scroll_size= if is_vertical { viewport_c.scroll_height() as f64 } else { viewport_c.scroll_width() as f64 };
        let client_size= if is_vertical { viewport_c.client_height() as f64 } else { viewport_c.client_width() as f64 };
        let page       = client_size * 0.9;
        let direction  = if click_pos > thumb_mid { 1.0 } else { -1.0 };

        if is_vertical {
            viewport_c.set_scroll_top((viewport_c.scroll_top() as f64 + direction * page) as i32);
        } else {
            viewport_c.set_scroll_left((viewport_c.scroll_left() as f64 + direction * page) as i32);
        }
    }) as Box<dyn FnMut(_)>);

    scrollbar.add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref())
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "click listener".into() })?;
    on_click.forget();
    Ok(())
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
