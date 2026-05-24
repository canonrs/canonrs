//! Markdown Interaction Engine
//! Core: dom/ + clipboard + toc scrollspy

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::Element;
use canonrs_interactions_core::dom::state;
use canonrs_interactions_core::runtime::{listeners, timers};

fn copy_code_to_clipboard(code: String, btn: Element) {
    let window = match web_sys::window() { Some(w) => w, None => return };
    let promise = window.navigator().clipboard().write_text(&code);
    spawn_local(async move {
        let result = wasm_bindgen_futures::JsFuture::from(promise).await;
        if result.is_ok() {
            state::add(&btn, "copied");
            if let Ok(Some(label)) = btn.query_selector("[data-rs-copy-label]") {
                label.set_text_content(Some("Copied!"));
            }
        } else {
            state::add(&btn, "error");
        }
        let btn_reset = btn.clone();
        timers::timeout(2000, move || {
            state::remove(&btn_reset, "active");
            state::remove(&btn_reset, "copied");
            state::remove(&btn_reset, "error");
            if let Ok(Some(label)) = btn_reset.query_selector("[data-rs-copy-label]") {
                label.set_text_content(Some("Copy"));
            }
        });
    });
}

fn setup_copy_buttons(root: &Element) {
    let Ok(btns) = root.query_selector_all("[data-rs-copy-button]") else { return };
    for i in 0..btns.length() {
        let Some(node) = btns.item(i) else { continue };
        let Ok(btn) = node.dyn_into::<Element>() else { continue };
        if btn.has_attribute("data-rs-md-copy-attached") { continue; }
        btn.set_attribute("data-rs-md-copy-attached", "1").ok();

        let uid = format!("md-copy:{}", btn.get_attribute("data-rs-uid").unwrap_or_else(|| i.to_string()));
        listeners::listen(&uid, &btn, "click", {
            let btn_c = btn.clone();
            move |_: web_sys::Event| {
                let code = btn_c.closest("[data-rs-code-block]").ok().flatten()
                    .and_then(|block| block.query_selector("[data-rs-code-pre]").ok().flatten())
                    .map(|pre| pre.text_content().unwrap_or_default())
                    .unwrap_or_default();
                if code.is_empty() { return; }
                copy_code_to_clipboard(code, btn_c.clone());
            }
        });
    }
}

pub fn init(root: Element) {
    setup_copy_buttons(&root);
    setup_toc(&root);
}

fn setup_toc(root: &Element) {
    let Ok(tocs) = root.query_selector_all("[data-rs-toc]") else { return };
    for i in 0..tocs.length() {
        if let Some(node) = tocs.item(i) {
            if let Ok(toc) = node.dyn_into::<Element>() {
                if toc.has_attribute("data-rs-toc-md-attached") { continue; }
                toc.set_attribute("data-rs-toc-md-attached", "1").ok();
                setup_toc_anchor_click(&toc);
                let toc_delayed = toc.clone();
                timers::timeout(300, move || {
                    setup_toc_scroll_spy(&toc_delayed);
                });
            }
        }
    }
}

fn setup_toc_anchor_click(toc: &Element) {
    let uid = format!("md-toc-click:{}", toc.get_attribute("data-rs-uid").unwrap_or_default());
    listeners::listen(&uid, toc, "click", move |e: web_sys::Event| {
        let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
        let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        let Some(link) = target.closest("[data-rs-toc-link]").ok().flatten() else { return };
        e.prevent_default();
        let href = link.get_attribute("href").unwrap_or_default();
        if href.starts_with('#') {
            let id = &href[1..];
            let win = web_sys::window().unwrap();
            let doc = win.document().unwrap();
            if let Ok(Some(target_el)) = doc.query_selector(&format!("#{}", id)) {
                let rect = target_el.get_bounding_client_rect();
                let scroll_y = win.scroll_y().unwrap_or(0.0);
                let top = rect.top() + scroll_y - 80.0;
                let opts = web_sys::ScrollToOptions::new();
                opts.set_top(top);
                opts.set_behavior(web_sys::ScrollBehavior::Smooth);
                win.scroll_to_with_scroll_to_options(&opts);
                if let Ok(history) = win.history() {
                    history.replace_state_with_url(&JsValue::NULL, "", Some(&format!("#{}", id))).ok();
                }
            }
        }
    });
}

fn setup_toc_scroll_spy(toc: &Element) {
    let Ok(items) = toc.query_selector_all("[data-rs-toc-item][data-rs-target]") else { return };
    let mut heading_ids: Vec<String> = Vec::new();
    for i in 0..items.length() {
        if let Some(node) = items.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() {
                if let Some(id) = el.get_attribute("data-rs-target") {
                    heading_ids.push(id);
                }
            }
        }
    }
    if heading_ids.is_empty() { return; }

    let scroll_viewport = toc.closest("[data-rs-md-layout]").ok().flatten()
        .and_then(|layout| layout.query_selector("[data-rs-markdown-content]").ok().flatten())
        .and_then(|content_el| {
            let mut el: Option<web_sys::Element> = Some(content_el);
            while let Some(parent) = el {
                if parent.has_attribute("data-rs-scroll-viewport") { return Some(parent); }
                el = parent.parent_element();
            }
            None
        })
        .or_else(|| {
            let doc = web_sys::window()?.document()?;
            let content_el = doc.query_selector("[data-rs-markdown-content]").ok()??;
            let mut el: Option<web_sys::Element> = Some(content_el);
            while let Some(parent) = el {
                if parent.has_attribute("data-rs-scroll-viewport") { return Some(parent); }
                el = parent.parent_element();
            }
            None
        });

    let toc_uid = format!("md-scroll:{}", toc.get_attribute("data-rs-uid").unwrap_or_default());
    let toc_c   = toc.clone();
    let hids    = heading_ids.clone();
    let last_active = std::rc::Rc::new(std::cell::RefCell::new(None::<String>));
    let la_cb   = last_active.clone();
    let vp_c    = scroll_viewport.clone();

    let on_scroll_fn = move |_: web_sys::Event| {
        let doc2 = web_sys::window().unwrap().document().unwrap();
        let vp_top = vp_c.as_ref().map(|v| v.get_bounding_client_rect().top()).unwrap_or(0.0);
        let threshold = vp_top + 80.0;
        let mut closest: Option<(f64, String)> = None;
        for id in &hids {
            if let Ok(Some(el)) = doc2.query_selector(&format!("#{}", id)) {
                let top = el.get_bounding_client_rect().top();
                if top <= threshold {
                    match &closest {
                        None => { closest = Some((top, id.clone())); }
                        Some((ct, _)) => { if top > *ct { closest = Some((top, id.clone())); } }
                    }
                }
            }
        }
        let active_id = closest.map(|(_, id)| id).or_else(|| hids.first().cloned());
        let Some(id) = active_id else { return };
        if la_cb.borrow().as_deref() == Some(&id) { return; }
        *la_cb.borrow_mut() = Some(id.clone());
        if let Ok(all) = toc_c.query_selector_all("[data-rs-toc-item]") {
            for j in 0..all.length() {
                if let Some(el) = all.item(j).and_then(|n| n.dyn_into::<Element>().ok()) {
                    state::remove(&el, "active");
                }
            }
        }
        let selector = format!("[data-rs-toc-item][data-rs-target='{}']", id);
        if let Ok(Some(item)) = toc_c.query_selector(&selector) {
            state::add(&item, "active");
        }
    };

    let target: &web_sys::EventTarget = match &scroll_viewport {
        Some(el) => el.as_ref(),
        None => {
            let win = web_sys::window().unwrap();
            listeners::listen_opts(
                &toc_uid,
                win.as_ref(),
                "scroll",
                canonrs_interactions_core::runtime::listeners::ListenOpts { capture: true, passive: false },
                on_scroll_fn,
            );
            return;
        }
    };

    listeners::listen_opts(
        &toc_uid,
        target,
        "scroll",
        canonrs_interactions_core::runtime::listeners::ListenOpts { capture: true, passive: false },
        on_scroll_fn,
    );
}
