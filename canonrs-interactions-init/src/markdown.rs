//! Markdown Init — anchor nav + scroll spy + TOC sync

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{lifecycle, state, query};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    let offset = root.get_attribute("data-rs-scroll-offset")
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(80.0);

    // anchor nav — event delegation no root
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(link) = t.closest("[data-rs-md-a]").ok().flatten() else { return };
            let href = link.get_attribute("href").unwrap_or_default();
            if !href.starts_with('#') { return; }
            e.prevent_default();
            let id = &href[1..];
            let win = match web_sys::window() { Some(w) => w, None => return };
            let doc = match win.document() { Some(d) => d, None => return };
            if let Ok(Some(target)) = doc.query_selector(&format!("#{}", id)) {
                let scroll_container = root_cb.closest("[data-rs-scroll-viewport]").ok().flatten();
                if let Some(vp) = scroll_container {
                    let Ok(vp_el) = vp.clone().dyn_into::<web_sys::HtmlElement>() else { return };
                    let target_rect = target.get_bounding_client_rect();
                    let vp_rect = vp.get_bounding_client_rect();
                    let current = vp_el.scroll_top() as f64;
                    let top = current + target_rect.top() - vp_rect.top() - offset;
                    let opts = web_sys::ScrollToOptions::new();
                    opts.set_top(top);
                    opts.set_behavior(web_sys::ScrollBehavior::Smooth);
                    vp_el.scroll_to_with_scroll_to_options(&opts);
                } else {
                    let rect = target.get_bounding_client_rect();
                    let scroll_y = win.scroll_y().unwrap_or(0.0);
                    let top = rect.top() + scroll_y - offset;
                    let opts = web_sys::ScrollToOptions::new();
                    opts.set_top(top);
                    opts.set_behavior(web_sys::ScrollBehavior::Smooth);
                    win.scroll_to_with_scroll_to_options(&opts);
                }
            }
        });
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // scroll spy via IntersectionObserver
    {
        let heading_ids: Vec<String> = query::all(&root, "[data-rs-md-heading][id]")
            .iter()
            .filter_map(|el| el.get_attribute("id"))
            .filter(|id| !id.is_empty())
            .collect();

        if heading_ids.is_empty() { return; }

        let doc = match web_sys::window().and_then(|w| w.document()) {
            Some(d) => d, None => return,
        };

        // clone doc antes de mover para closure
        let doc_obs = doc.clone();
        let heading_ids_obs = heading_ids.clone();

        let visible: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<String, f64>>> =
            std::rc::Rc::new(std::cell::RefCell::new(std::collections::HashMap::new()));
        let visible_cb = visible.clone();

        let on_active = Closure::<dyn Fn(js_sys::Array)>::new(move |entries: js_sys::Array| {
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
                    let mut closest: Option<(f64, String)> = None;
                    for id in &heading_ids_obs {
                        if let Ok(Some(el)) = doc_obs.query_selector(&format!("#{}", id)) {
                            let top = el.get_bounding_client_rect().top();
                            if top <= 120.0 {
                                match &closest {
                                    None => { closest = Some((top, id.clone())); }
                                    Some((ct, _)) => { if top > *ct { closest = Some((top, id.clone())); } }
                                }
                            }
                        }
                    }
                    closest.map(|(_, id)| id).or_else(|| heading_ids_obs.first().cloned())
                } else {
                    heading_ids_obs.iter().find(|id| map.contains_key(*id)).cloned()
                }
            };

            let Some(id) = active_id else { return };

            if let Ok(toc_items) = doc_obs.query_selector_all("[data-rs-toc-item]") {
                for i in 0..toc_items.length() {
                    if let Some(item) = toc_items.item(i).and_then(|n| n.dyn_into::<Element>().ok()) {
                        item.set_attribute("data-rs-state", "idle").ok();
                    }
                }
            }

            let selector = format!("[data-rs-toc-item][data-rs-target='{}']", id);
            if let Ok(Some(item)) = doc_obs.query_selector(&selector) {
                state::add_state(&item, "active");
            }
        });

        let scroll_root = doc.query_selector("[data-rs-scroll-viewport]").ok().flatten();
        let options = web_sys::IntersectionObserverInit::new();
        options.set_threshold(&JsValue::from_f64(0.0));
        options.set_root_margin("0px 0px -40% 0px");
        if let Some(ref sr) = scroll_root {
            options.set_root(Some(sr));
        }

        let observer = match web_sys::IntersectionObserver::new_with_options(
            on_active.as_ref().unchecked_ref(),
            &options,
        ) {
            Ok(o) => o, Err(_) => return,
        };

        for id in &heading_ids {
            if let Ok(Some(el)) = doc.query_selector(&format!("#{}", id)) {
                observer.observe(&el);
            }
        }

        on_active.forget();
        Box::leak(Box::new(observer));
    }
}
