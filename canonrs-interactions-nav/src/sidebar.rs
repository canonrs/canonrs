//! Sidebar Interaction Engine

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::runtime::{lifecycle, state, query};
use web_sys::Element;

fn is_expanded(root: &Element) -> bool {
    state::has(root, "expanded")
}

fn is_pinned(root: &Element) -> bool {
    root.get_attribute("data-rs-pinned").as_deref() == Some("true")
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    let is_rail = root.get_attribute("data-rs-variant").as_deref() == Some("rail");

    // toggle button
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-sidebar-toggle]").ok().flatten().is_none() { return; }
            if is_pinned(&root_cb) { return; }
            if is_expanded(&root_cb) {
                state::remove_state(&root_cb, "expanded");
                state::add_state(&root_cb, "collapsed");
            } else {
                state::remove_state(&root_cb, "collapsed");
                state::add_state(&root_cb, "expanded");
            }
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // pin toggle
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-sidebar-pin-toggle]").ok().flatten().is_none() { return; }
            let pinned = is_pinned(&root_cb);
            let _ = root_cb.set_attribute("data-rs-pinned", if pinned { "false" } else { "true" });
            if !pinned {
                state::remove_state(&root_cb, "collapsed");
                state::add_state(&root_cb, "expanded");
            }
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // hover on menu items
    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(item) = target.closest("[data-rs-sidebar-menu-item]").ok().flatten() else { return };
            if !state::has(&item, "disabled") { state::add_state(&item, "hover"); }
        }));
        let _ = root.add_event_listener_with_callback("mouseover", cb.as_ref().unchecked_ref());
        cb.forget();
    }
    {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(item) = target.closest("[data-rs-sidebar-menu-item]").ok().flatten() else { return };
            state::remove_state(&item, "hover");
        }));
        let _ = root.add_event_listener_with_callback("mouseout", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // click menu item → activate
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(item) = target.closest("[data-rs-sidebar-menu-item]").ok().flatten() else { return };
            if state::has(&item, "disabled") { return; }
            for el in query::all(&root_cb, "[data-rs-sidebar-menu-item]") {
                state::remove_state(&el, "active");
                state::add_state(&el, "inactive");
                let _ = el.remove_attribute("aria-current");
            }
            state::remove_state(&item, "inactive");
            state::add_state(&item, "active");
            let _ = item.set_attribute("aria-current", "page");
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // keyboard nav
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-sidebar-menu-item]").ok().flatten().is_none() { return; }
            let items: Vec<Element> = query::all(&root_cb, "[data-rs-sidebar-menu-item]")
                .into_iter()
                .filter(|el| !state::has(el, "disabled"))
                .collect();
            let len = items.len();
            if len == 0 { return; }
            let pos = items.iter().position(|el| el.contains(Some(target.as_ref())));
            match e.key().as_str() {
                "ArrowDown" => {
                    e.prevent_default();
                    let next = pos.map(|p| (p + 1).min(len - 1)).unwrap_or(0);
                    if let Ok(h) = items[next].clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); }
                }
                "ArrowUp" => {
                    e.prevent_default();
                    let prev = pos.map(|p| if p == 0 { 0 } else { p - 1 }).unwrap_or(0);
                    if let Ok(h) = items[prev].clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); }
                }
                "Home" => {
                    e.prevent_default();
                    if let Ok(h) = items[0].clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); }
                }
                "End" => {
                    e.prevent_default();
                    if let Ok(h) = items[len-1].clone().dyn_into::<web_sys::HtmlElement>() { let _ = h.focus(); }
                }
                _ => {}
            }
        }));
        let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // rail hover
    if is_rail {
        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |_| {
                state::remove_state(&root_cb, "collapsed");
                state::add_state(&root_cb, "expanded");
            }));
            let _ = root.add_event_listener_with_callback("mouseenter", cb.as_ref().unchecked_ref());
            cb.forget();
        }
        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |_| {
                if !is_pinned(&root_cb) {
                    state::remove_state(&root_cb, "expanded");
                    state::add_state(&root_cb, "collapsed");
                }
            }));
            let _ = root.add_event_listener_with_callback("mouseleave", cb.as_ref().unchecked_ref());
            cb.forget();
        }
    }
}
