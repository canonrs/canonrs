//! TableOfContents Init — scroll, hover, active spy, expand, nested

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state, query};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    let mode = root.get_attribute("data-rs-mode").unwrap_or_default();

    // hover no root — mostra scrollbar
    {
        let r_enter = root.clone();
        let enter = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
            state::add_state(&r_enter, "hover");
        });
        let r_leave = root.clone();
        let leave = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
            state::remove_state(&r_leave, "hover");
        });
        let _ = root.add_event_listener_with_callback("mouseenter", enter.as_ref().unchecked_ref());
        let _ = root.add_event_listener_with_callback("mouseleave", leave.as_ref().unchecked_ref());
        enter.forget();
        leave.forget();
    }

    // hover nos links
    for link in query::all(&root, "[data-rs-toc-link]") {
        let l_enter = link.clone();
        let enter = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
            state::add_state(&l_enter, "hover");
        });
        let l_leave = link.clone();
        let leave = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
            state::remove_state(&l_leave, "hover");
        });
        let _ = link.add_event_listener_with_callback("mouseenter", enter.as_ref().unchecked_ref());
        let _ = link.add_event_listener_with_callback("mouseleave", leave.as_ref().unchecked_ref());
        enter.forget();
        leave.forget();
    }

    // click nos links — smooth scroll
    for link in query::all(&root, "[data-rs-toc-link]") {
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            e.prevent_default();
            let Some(t) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let href = t.get_attribute("href").unwrap_or_default();
            if href.starts_with('#') {
                let id = &href[1..];
                let win = match web_sys::window() { Some(w) => w, None => return };
                let doc = match win.document() { Some(d) => d, None => return };
                if let Ok(Some(target)) = doc.query_selector(&format!("#{}", id)) {
                    let rect = target.get_bounding_client_rect();
                    let scroll_y = win.scroll_y().unwrap_or(0.0);
                    let top = rect.top() + scroll_y - 80.0;
                    let opts = web_sys::ScrollToOptions::new();
                    opts.set_top(top);
                    opts.set_behavior(web_sys::ScrollBehavior::Smooth);
                    win.scroll_to_with_scroll_to_options(&opts);
                }
            }
        });
        let _ = link.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // expand mode — click no item pai mostra/esconde filhos
    if mode == "expand" {
        let root_exp = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(item) = t.closest("[data-rs-toc-item]").ok().flatten() else { return };
            if item.get_attribute("data-child").as_deref() == Some("true") { return; }
            let item_target = item.get_attribute("data-rs-target").unwrap_or_default();
            let item_level = item.get_attribute("data-rs-level")
                .unwrap_or_default().parse::<u32>().unwrap_or(1);
            // coleta filhos — itens de nível maior que vêm depois deste item no DOM
            let all_items = query::all(&root_exp, "[data-rs-toc-item]");
            let mut found = false;
            let mut show = false;
            for child in &all_items {
                if !found {
                    if child.get_attribute("data-rs-target").as_deref() == Some(&item_target) {
                        found = true;
                    }
                    continue;
                }
                let child_level = child.get_attribute("data-rs-level")
                    .unwrap_or_default().parse::<u32>().unwrap_or(1);
                if child_level <= item_level { break; }
                // primeira iteração determina se vai mostrar ou esconder
                let st = child.get_attribute("data-rs-state").unwrap_or_default();
                if !show && !st.contains("visible") { show = true; }
                if show {
                    state::add_state(child, "visible");
                } else {
                    state::remove_state(child, "visible");
                }
            }
        });
        let _ = root_exp.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // nested mode — expand button abre/fecha subtree
    if mode == "nested" {
        for btn in query::all(&root, "[data-rs-toc-expand-btn]") {
            let b_enter = btn.clone();
            let enter = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
                state::add_state(&b_enter, "hover");
            });
            let b_leave = btn.clone();
            let leave = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
                state::remove_state(&b_leave, "hover");
            });
            let _ = btn.add_event_listener_with_callback("mouseenter", enter.as_ref().unchecked_ref());
            let _ = btn.add_event_listener_with_callback("mouseleave", leave.as_ref().unchecked_ref());
            enter.forget();
            leave.forget();

            let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
                e.stop_propagation();
                let Some(t) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
                let Some(item) = t.closest("[data-rs-toc-item]").ok().flatten() else { return };
                let Some(subtree) = query::first(&item, "[data-rs-toc-subtree]") else { return };
                let st = subtree.get_attribute("data-rs-state").unwrap_or_default();
                if st.contains("open") {
                    state::remove_state(&subtree, "open");
                    state::add_state(&subtree, "closed");
                    state::remove_state(&t, "expanded");
                } else {
                    state::remove_state(&subtree, "closed");
                    state::add_state(&subtree, "open");
                    state::add_state(&t, "expanded");
                }
            });
            let _ = btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
            cb.forget();
        }
    }

    // scroll spy — porta do markdown.rs
    {
        let root_spy = root.clone();
        let heading_ids: Vec<String> = query::all(&root, "[data-rs-toc-item][data-rs-target]")
            .iter()
            .filter_map(|el| el.get_attribute("data-rs-target"))
            .filter(|id| !id.is_empty())
            .collect();

        if !heading_ids.is_empty() {
            let last_active = std::rc::Rc::new(std::cell::RefCell::new(None::<String>));
            let la_cb = last_active.clone();

            let on_scroll = Closure::<dyn Fn()>::new(move || {
                let doc = match web_sys::window().and_then(|w| w.document()) {
                    Some(d) => d, None => return,
                };
                let threshold = 80.0;
                let mut closest: Option<(f64, String)> = None;
                for id in &heading_ids {
                    if let Ok(Some(el)) = doc.query_selector(&format!("#{}", id)) {
                        let top = el.get_bounding_client_rect().top();
                        if top <= threshold {
                            match &closest {
                                None => { closest = Some((top, id.clone())); }
                                Some((ct, _)) => {
                                    if top > *ct { closest = Some((top, id.clone())); }
                                }
                            }
                        }
                    }
                }
                let active_id = closest.map(|(_, id)| id)
                    .or_else(|| heading_ids.first().cloned());
                let Some(id) = active_id else { return };
                if la_cb.borrow().as_deref() == Some(&id) { return; }
                *la_cb.borrow_mut() = Some(id.clone());
                for item in query::all(&root_spy, "[data-rs-toc-item]") {
                    item.set_attribute("data-rs-state", "idle").ok();
                }
                let selector = format!("[data-rs-toc-item][data-rs-target='{}']", id);
                if let Some(item) = query::first(&root_spy, &selector) {
                    state::add_state(&item, "active");
                }
            });

            if let Some(win) = web_sys::window() {
                let _ = win.add_event_listener_with_callback_and_bool(
                    "scroll", on_scroll.as_ref().unchecked_ref(), true,
                );
            }
            on_scroll.forget();
        }
    }
}
