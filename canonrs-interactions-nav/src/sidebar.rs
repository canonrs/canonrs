//! Sidebar Interaction Engine
//! Tier S — usa canonrs-interactions-core

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::{lifecycle, state, query};
use canonrs_interactions_core::behavior::keyboard::{init_nav, NavConfig, Orientation, ElementType};

fn is_pinned(root: &Element) -> bool {
    root.get_attribute("data-rs-pinned").as_deref() == Some("true")
}

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    let is_rail = root.get_attribute("data-rs-variant").as_deref() == Some("rail");

    // Estado inicial: expanded se SSR emitiu "expanded", collapsed caso contrário
    if !state::has(&root, "expanded") && !state::has(&root, "collapsed") {
        state::add(&root, "collapsed");
    }

    // toggle button
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-sidebar-toggle]").ok().flatten().is_none() { return; }
            if is_pinned(&root_cb) { return; }
            if state::is_expanded(&root_cb) {
                state::collapse(&root_cb);
            } else {
                state::expand(&root_cb);
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
                state::expand(&root_cb);
            }
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
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
                state::remove(&el, "active");
                state::add(&el, "inactive");
                let _ = el.remove_attribute("aria-current");
            }
            state::remove(&item, "inactive");
            state::add(&item, "active");
            let _ = item.set_attribute("aria-current", "page");
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // keyboard nav — via core::behavior::keyboard::init_nav
    init_nav(
        &root,
        "[data-rs-sidebar-menu-item]",
        NavConfig {
            orientation:  Orientation::Vertical,
            element_type: ElementType::Link,
            focus_state:  "focused",
            wrap:         false,
        },
        None,
        None,
    );

    // rail hover
    if is_rail {
        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |_| {
                state::expand(&root_cb);
            }));
            let _ = root.add_event_listener_with_callback("mouseenter", cb.as_ref().unchecked_ref());
            cb.forget();
        }
        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |_| {
                if !is_pinned(&root_cb) {
                    state::collapse(&root_cb);
                }
            }));
            let _ = root.add_event_listener_with_callback("mouseleave", cb.as_ref().unchecked_ref());
            cb.forget();
        }
    }
}
