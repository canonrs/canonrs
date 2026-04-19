//! HoverCard Interaction Engine — delay + hover bridge

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state};

fn is_leaving_root(e: &web_sys::PointerEvent, root: &Element) -> bool {
    let related = e.related_target().and_then(|t| t.dyn_into::<Element>().ok());
    match related {
        Some(rel) => !root.contains(Some(rel.as_ref())),
        None => true,
    }
}

fn set_timeout(f: Box<dyn FnOnce()>, ms: i32) {
    let cb = Closure::once(f);
    web_sys::window()
        .unwrap()
        .set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), ms)
        .ok();
    cb.forget();
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    let Ok(Some(_)) = root.query_selector("[data-rs-hover-card-trigger]") else { return };

    // pointerenter — open com delay 120ms
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::PointerEvent)>::wrap(Box::new(move |_: web_sys::PointerEvent| {
            let root2 = root_cb.clone();
            set_timeout(Box::new(move || {
                let Some(c) = root2.query_selector("[data-rs-hover-card-content]").ok().flatten() else { return };
                state::open(&c);
            }), 120);
        }));
        let _ = root.add_event_listener_with_callback("pointerenter", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // pointerleave — close com delay 80ms
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::PointerEvent)>::wrap(Box::new(move |e: web_sys::PointerEvent| {
            let root2 = root_cb.clone();
            if !is_leaving_root(&e, &root2) { return; }
            set_timeout(Box::new(move || {
                let Some(c) = root2.query_selector("[data-rs-hover-card-content]").ok().flatten() else { return };
                if !state::is_open(&c) { return; }
                state::close(&c);
            }), 80);
        }));
        let _ = root.add_event_listener_with_callback("pointerleave", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // focusin — open
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::wrap(Box::new(move |_: web_sys::FocusEvent| {
            let Some(c) = root_cb.query_selector("[data-rs-hover-card-content]").ok().flatten() else { return };
            state::open(&c);
        }));
        let _ = root.add_event_listener_with_callback("focusin", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // focusout — close
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::FocusEvent)>::wrap(Box::new(move |_: web_sys::FocusEvent| {
            let Some(c) = root_cb.query_selector("[data-rs-hover-card-content]").ok().flatten() else { return };
            state::close(&c);
        }));
        let _ = root.add_event_listener_with_callback("focusout", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}
