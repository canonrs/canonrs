//! TableOfContents Init — scroll spy via IntersectionObserver, auto-expand nested

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::{state, query};
use canonrs_interactions_core::runtime::{listeners, scrollspy};

pub fn init(root: Element) {
    let mode = root.get_attribute("data-rs-mode").unwrap_or_default();
    let uid  = root.get_attribute("data-rs-uid").unwrap_or_default();

    // hover on root
    listeners::listen(&uid, &root, "mouseenter", {
        let r = root.clone();
        move |_: web_sys::Event| { state::add_state(&r, "hover"); }
    });

    // hover on links — delegated
    listeners::listen(&uid, &root, "mouseover", {
        let r = root.clone();
        move |e: web_sys::Event| {
            let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let link = if t.has_attribute("data-rs-toc-link") { Some(t.clone()) }
                       else { t.closest("[data-rs-toc-link]").ok().flatten() };
            if let Some(l) = link {
                for other in query::all(&r, "[data-rs-toc-link]") { state::remove_state(&other, "hover"); }
                state::add_state(&l, "hover");
            }
        }
    });
    listeners::listen(&uid, &root, "mouseout", {
        move |e: web_sys::Event| {
            let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let link = if t.has_attribute("data-rs-toc-link") { Some(t.clone()) }
                       else { t.closest("[data-rs-toc-link]").ok().flatten() };
            if let Some(l) = link { state::remove_state(&l, "hover"); }
        }
    });

    // click on links — smooth scroll via scrollspy kernel
    let scroll_viewport = {
        let doc = web_sys::window().and_then(|w| w.document());
        doc.and_then(|d| d.query_selector("[data-rs-scroll-viewport]").ok().flatten())
    };
    listeners::listen(&uid, &root, "click", {
        let vp = scroll_viewport.clone();
        move |e: web_sys::Event| {
            let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(link) = t.closest("[data-rs-toc-link]").ok().flatten() else { return };
            e.prevent_default();
            let href = link.get_attribute("href").unwrap_or_default();
            if href.starts_with('#') {
                let id = &href[1..];
                let win = match web_sys::window() { Some(w) => w, None => return };
                let doc = match win.document() { Some(d) => d, None => return };
                if let Ok(Some(target)) = doc.query_selector(&format!("#{}", id)) {
                    if let Some(ref viewport) = vp {
                        let Ok(vp_el) = viewport.clone().dyn_into::<web_sys::HtmlElement>() else { return };
                        let target_rect = target.get_bounding_client_rect();
                        let vp_rect     = viewport.get_bounding_client_rect();
                        let current     = vp_el.scroll_top() as f64;
                        let top = current + target_rect.top() - vp_rect.top() - 80.0;
                        let opts = web_sys::ScrollToOptions::new();
                        opts.set_top(top);
                        opts.set_behavior(web_sys::ScrollBehavior::Smooth);
                        vp_el.scroll_to_with_scroll_to_options(&opts);
                    } else {
                        scrollspy::scroll_to_anchor(id, 80.0);
                    }
                }
            }
        }
    });

    // scroll spy via IntersectionObserver
    let heading_ids: Vec<String> = query::all(&root, "[data-rs-toc-item][data-rs-target]")
        .iter().filter_map(|el| el.get_attribute("data-rs-target"))
        .filter(|id| !id.is_empty()).collect();
    if heading_ids.is_empty() { return; }

    let doc = match web_sys::window().and_then(|w| w.document()) { Some(d) => d, None => return };

    // activate first item immediately
    if let Some(first_id) = heading_ids.first() {
        let selector = format!("[data-rs-toc-item][data-rs-target='{}']", first_id);
        if let Some(item) = query::first(&root, &selector) { state::add_state(&item, "active"); }
    }

    // Cache headings
    let cached = std::rc::Rc::new(scrollspy::cache_headings(&heading_ids));
    let visible: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<String, f64>>> =
        std::rc::Rc::new(std::cell::RefCell::new(std::collections::HashMap::new()));

    let heading_ids_rc = std::rc::Rc::new(heading_ids.clone());
    let root_spy       = root.clone();
    let mode_spy       = mode.clone();
    let visible_cb     = visible.clone();
    let cached_cb      = cached.clone();
    let hids_cb        = heading_ids_rc.clone();

    let targets: Vec<Element> = heading_ids.iter()
        .filter_map(|id| doc.query_selector(&format!("#{}", id)).ok().flatten())
        .collect();

    let scroll_root = doc
        .query_selector("[data-rs-main-viewport] [data-rs-scroll-viewport]").ok().flatten()
        .or_else(|| doc.query_selector("[data-rs-region='center'] [data-rs-scroll-viewport]").ok().flatten())
        .or_else(|| doc.query_selector("[data-rs-scroll-viewport]").ok().flatten());

    let options = web_sys::IntersectionObserverInit::new();
    options.set_threshold(&JsValue::from_f64(0.0));
    if let Some(ref sr) = scroll_root { options.set_root(Some(sr)); }
    options.set_root_margin("0px 0px -40% 0px");

    let closure = Closure::wrap(Box::new(move |entries: js_sys::Array, _: web_sys::IntersectionObserver| {
        for entry in entries.iter() {
            let entry = match entry.dyn_into::<web_sys::IntersectionObserverEntry>() { Ok(e) => e, Err(_) => continue };
            let id = match entry.target().get_attribute("id") { Some(id) => id, None => continue };
            if entry.is_intersecting() { visible_cb.borrow_mut().insert(id, entry.bounding_client_rect().top()); }
            else                        { visible_cb.borrow_mut().remove(&id); }
        }

        let active_id = {
            let map = visible_cb.borrow();
            if map.is_empty() { scrollspy::active_heading(cached_cb.as_ref(), None, 120.0) }
            else { hids_cb.iter().find(|id| map.contains_key(*id)).cloned() }
        };
        let Some(id) = active_id else { return };

        for item in query::all(&root_spy, "[data-rs-toc-item]") { state::remove(&item, "active"); }
        let selector = format!("[data-rs-toc-item][data-rs-target='{}']", id);
        let Some(active_item) = query::first(&root_spy, &selector) else { return };
        state::add_state(&active_item, "active");

        if mode_spy == "expand" {
            let all_items = query::all(&root_spy, "[data-rs-toc-item]");
            let active_level = active_item.get_attribute("data-rs-level").unwrap_or_default().parse::<u32>().unwrap_or(1);
            let active_target = active_item.get_attribute("data-rs-target").unwrap_or_default();
            let parent_target = if active_level > 1 {
                let mut found = false; let mut parent: Option<String> = None;
                for item in all_items.iter().rev() {
                    if !found { if item.get_attribute("data-rs-target").as_deref() == Some(&active_target) { found = true; } continue; }
                    let lvl = item.get_attribute("data-rs-level").unwrap_or_default().parse::<u32>().unwrap_or(1);
                    if lvl < active_level { parent = item.get_attribute("data-rs-target"); break; }
                }
                parent.unwrap_or(active_target.clone())
            } else { active_target.clone() };

            let mut in_parent = false; let mut parent_level = 1u32;
            for item in &all_items {
                let target  = item.get_attribute("data-rs-target").unwrap_or_default();
                let level   = item.get_attribute("data-rs-level").unwrap_or_default().parse::<u32>().unwrap_or(1);
                let is_child = item.get_attribute("data-rs-child").as_deref() == Some("true");
                if target == parent_target { in_parent = true; parent_level = level; continue; }
                if is_child {
                    if in_parent && level > parent_level { state::add_state(item, "visible"); }
                    else { state::remove_state(item, "visible"); }
                } else if level <= parent_level && target != parent_target { in_parent = false; }
            }
        }

        if mode_spy == "nested" {
            for subtree in query::all(&root_spy, "[data-rs-toc-subtree]") {
                state::remove_state(&subtree, "open"); state::add_state(&subtree, "closed");
            }
            for btn in query::all(&root_spy, "[data-rs-toc-expand-btn]") { state::remove_state(&btn, "expanded"); }
            let mut current = active_item.parent_element();
            while let Some(el) = current {
                if el.has_attribute("data-rs-toc-subtree") {
                    state::remove_state(&el, "closed"); state::add_state(&el, "open");
                    if let Some(parent_item) = el.parent_element() {
                        if let Some(btn) = query::first(&parent_item, "[data-rs-toc-expand-btn]") { state::add_state(&btn, "expanded"); }
                        if parent_item.has_attribute("data-rs-toc-item") { state::add_state(&parent_item, "ancestor"); }
                    }
                }
                current = el.parent_element();
                if current.as_ref().map(|e| e == &root_spy).unwrap_or(false) { break; }
            }
        }
    }) as Box<dyn FnMut(js_sys::Array, web_sys::IntersectionObserver)>);

    if let Ok(observer) = web_sys::IntersectionObserver::new_with_options(closure.as_ref().unchecked_ref(), &options) {
        for target in &targets { observer.observe(target); }
        let key = format!("__canon_io_toc_{}", uid);
        let _ = js_sys::Reflect::set(&root, &key.into(), &observer.into());
    }
    closure.forget();
}
