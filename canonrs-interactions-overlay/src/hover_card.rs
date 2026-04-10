//! HoverCard Interaction Engine

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state};

fn resolve(e: &web_sys::PointerEvent) -> Option<(Element, Element)> {
    let root = e.current_target()?.dyn_into::<Element>().ok()?;
    let content = root.query_selector("[data-rs-hover-card-content]").ok().flatten()?;
    Some((root, content))
}

fn is_leaving_root(e: &web_sys::PointerEvent, root: &Element) -> bool {
    let related = e.related_target().and_then(|t| t.dyn_into::<Element>().ok());
    match related {
        Some(rel) => !root.contains(Some(rel.as_ref())),
        None => true,
    }
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    let Ok(Some(_)) = root.query_selector("[data-rs-hover-card-trigger]") else { return };

    {
        let cb = Closure::<dyn Fn(web_sys::PointerEvent)>::wrap(Box::new(move |e: web_sys::PointerEvent| {
            let Some((_, c)) = resolve(&e) else { return };
            state::open(&c);
        }));
        let _ = root.add_event_listener_with_callback("pointerenter", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    {
        let cb = Closure::<dyn Fn(web_sys::PointerEvent)>::wrap(Box::new(move |e: web_sys::PointerEvent| {
            let Some((r, c)) = resolve(&e) else { return };
            if is_leaving_root(&e, &r) { state::close(&c); }
        }));
        let _ = root.add_event_listener_with_callback("pointerleave", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    {
        let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::wrap(Box::new(move |e: web_sys::FocusEvent| {
            let Some(r) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(c) = r.query_selector("[data-rs-hover-card-content]").ok().flatten() else { return };
            state::open(&c);
        }));
        let _ = root.add_event_listener_with_callback("focusin", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    {
        let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::wrap(Box::new(move |e: web_sys::FocusEvent| {
            let Some(r) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(c) = r.query_selector("[data-rs-hover-card-content]").ok().flatten() else { return };
            state::close(&c);
        }));
        let _ = root.add_event_listener_with_callback("focusout", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}
