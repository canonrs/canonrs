//! Markdown Init — anchor nav + scroll spy + TOC sync

use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::{state, query};
use canonrs_interactions_core::runtime::{listeners, scrollspy};

pub fn init(root: Element) {
    let offset = root.get_attribute("data-rs-scroll-offset")
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(80.0);

    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

    // anchor nav — event delegation via listeners kernel
    listeners::listen(&uid, &root, "click", {
        let root_c = root.clone();
        move |e: web_sys::Event| {
            let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(link) = t.closest("[data-rs-md-a]").ok().flatten() else { return };
            let href = link.get_attribute("href").unwrap_or_default();
            if !href.starts_with('#') { return; }
            e.prevent_default();
            let id = &href[1..];
            let win = match web_sys::window() { Some(w) => w, None => return };
            let doc = match win.document() { Some(d) => d, None => return };
            if let Ok(Some(target)) = doc.query_selector(&format!("#{}", id)) {
                let scroll_container = root_c.closest("[data-rs-scroll-viewport]").ok().flatten();
                if let Some(vp) = scroll_container {
                    let Ok(vp_el) = vp.clone().dyn_into::<web_sys::HtmlElement>() else { return };
                    let target_rect = target.get_bounding_client_rect();
                    let vp_rect    = vp.get_bounding_client_rect();
                    let current    = vp_el.scroll_top() as f64;
                    let top = current + target_rect.top() - vp_rect.top() - offset;
                    let opts = web_sys::ScrollToOptions::new();
                    opts.set_top(top);
                    opts.set_behavior(web_sys::ScrollBehavior::Smooth);
                    vp_el.scroll_to_with_scroll_to_options(&opts);
                } else {
                    scrollspy::scroll_to_anchor(id, offset);
                }
            }
        }
    });

    // scroll spy via observer kernel
    let heading_ids: Vec<String> = query::all(&root, "[data-rs-md-heading][id]")
        .iter()
        .filter_map(|el| el.get_attribute("id"))
        .filter(|id| !id.is_empty())
        .collect();

    if heading_ids.is_empty() { return; }

    let doc = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d, None => return,
    };

    // Cache heading elements — no DOM queries in observer callback
    let cached = scrollspy::cache_headings(&heading_ids);
    let cached_rc = std::rc::Rc::new(cached);
    let heading_ids_rc = std::rc::Rc::new(heading_ids.clone());

    let visible: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<String, f64>>> =
        std::rc::Rc::new(std::cell::RefCell::new(std::collections::HashMap::new()));

    let doc_obs     = doc.clone();
    let visible_cb  = visible.clone();
    let cached_cb   = cached_rc.clone();
    let hids_cb     = heading_ids_rc.clone();

    // Collect target elements for observer
    let targets: Vec<Element> = heading_ids.iter().filter_map(|id| {
        doc.query_selector(&format!("#{}", id)).ok().flatten()
    }).collect();

    let scroll_root = doc.query_selector("[data-rs-scroll-viewport]").ok().flatten();
    let opts = web_sys::IntersectionObserverInit::new();
    opts.set_threshold(&wasm_bindgen::JsValue::from_f64(0.0));
    opts.set_root_margin("0px 0px -40% 0px");
    if let Some(ref sr) = scroll_root {
        opts.set_root(Some(sr));
    }

    let uid_obs = format!("{}:scrollspy", uid);
    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |entries: js_sys::Array, _: web_sys::IntersectionObserver| {
        for entry in entries.iter() {
            let entry = match entry.dyn_into::<web_sys::IntersectionObserverEntry>() {
                Ok(e) => e, Err(_) => continue,
            };
            let id = match entry.target().get_attribute("id") {
                Some(id) => id, None => continue,
            };
            if entry.is_intersecting() {
                visible_cb.borrow_mut().insert(id, entry.bounding_client_rect().top());
            } else {
                visible_cb.borrow_mut().remove(&id);
            }
        }

        let active_id = {
            let map = visible_cb.borrow();
            if map.is_empty() {
                // Use cached elements — no DOM query
                scrollspy::active_heading(cached_cb.as_ref(), None, 120.0)
            } else {
                hids_cb.iter().find(|id| map.contains_key(*id)).cloned()
            }
        };

        let Some(id) = active_id else { return };

        if let Ok(toc_items) = doc_obs.query_selector_all("[data-rs-toc-item]") {
            for i in 0..toc_items.length() {
                if let Some(item) = toc_items.item(i).and_then(|n| n.dyn_into::<Element>().ok()) {
                    state::remove(&item, "active");
                }
            }
        }
        let selector = format!("[data-rs-toc-item][data-rs-target='{}']", id);
        if let Ok(Some(item)) = doc_obs.query_selector(&selector) {
            state::add_state(&item, "active");
        }
    }) as Box<dyn FnMut(js_sys::Array, web_sys::IntersectionObserver)>);

    if let Ok(observer_inst) = web_sys::IntersectionObserver::new_with_options(
        closure.as_ref().unchecked_ref(), &opts
    ) {
        for target in &targets {
            observer_inst.observe(target);
        }
        // Store observer + closure on root to prevent GC — owned by element lifecycle
        let key = format!("__canon_io_{}", uid_obs);
        let _ = js_sys::Reflect::set(&root, &key.into(), &observer_inst.into());
        let key_cb = format!("__canon_io_cb_{}", uid_obs);
        let _ = js_sys::Reflect::set(&root, &key_cb.into(), closure.as_ref());
        closure.forget(); // observer-owned: lives as long as element
    } else {
        drop(closure);
    }
}
