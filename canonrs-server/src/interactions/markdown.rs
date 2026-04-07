//! Markdown Interaction Engine
//! Code block copy buttons, TOC active state

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;

fn setup_copy_buttons(root: &Element) {
    let Ok(btns) = root.query_selector_all("[data-rs-copy-button]") else { return };
    for i in 0..btns.length() {
        let Some(node) = btns.item(i) else { continue };
        let Ok(btn) = node.dyn_into::<Element>() else { continue };
        if btn.has_attribute("data-rs-md-copy-attached") { continue; }
        btn.set_attribute("data-rs-md-copy-attached", "1").ok();

        // get code from parent [data-rs-code-pre]
        let btn_c = btn.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            // find code text from sibling pre
            let code = btn_c.closest("[data-rs-code-block]").ok().flatten()
                .and_then(|block| block.query_selector("[data-rs-code-pre]").ok().flatten())
                .map(|pre| pre.text_content().unwrap_or_default())
                .unwrap_or_default();

            if code.is_empty() { return; }

            let window = web_sys::window().unwrap();
            let f = js_sys::Function::new_with_args(
                "text",
                "return navigator.clipboard.writeText(text);"
            );
            let promise = js_sys::Promise::from(
                f.call1(&wasm_bindgen::JsValue::NULL, &wasm_bindgen::JsValue::from_str(&code)).unwrap_or(wasm_bindgen::JsValue::NULL)
            );
            let _ = window;

            let btn_ok = btn_c.clone();
            let then = Closure::wrap(Box::new(move |_: JsValue| {
                btn_ok.set_attribute("data-rs-state", "copied").ok();
                if let Ok(Some(label)) = btn_ok.query_selector("[data-rs-copy-label]") {
                    label.set_text_content(Some("Copied!"));
                }
                let btn_reset = btn_ok.clone();
                let reset = Closure::wrap(Box::new(move || {
                    btn_reset.set_attribute("data-rs-state", "idle").ok();
                    if let Ok(Some(label)) = btn_reset.query_selector("[data-rs-copy-label]") {
                        label.set_text_content(Some("Copy"));
                    }
                }) as Box<dyn Fn()>);
                web_sys::window().unwrap()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(reset.as_ref().unchecked_ref(), 2000).ok();
                reset.forget();
            }) as Box<dyn FnMut(_)>);

            let _ = promise.then(&then);
            then.forget();
        }) as Box<dyn FnMut(_)>);

        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }
}

pub fn init(root: Element) {
    let key = JsValue::from_str("__rs_markdown_bound");
    let root_val = JsCast::unchecked_ref::<JsValue>(&root);
    if js_sys::Reflect::get(root_val, &key).ok().filter(|v| v.is_truthy()).is_some() { return; }
    js_sys::Reflect::set(root_val, &key, &JsValue::TRUE).ok();

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
                // delay para aguardar inner_html ser injetado
                let toc_delayed = toc.clone();
                let cb = Closure::wrap(Box::new(move || {
                    setup_toc_scroll_spy(&toc_delayed);
                }) as Box<dyn Fn()>);
                web_sys::window().unwrap()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        cb.as_ref().unchecked_ref(), 300
                    ).ok();
                cb.forget();
            }
        }
    }
}

fn setup_toc_anchor_click(toc: &Element) {
    let Ok(links) = toc.query_selector_all("[data-rs-toc-link]") else { return };
    for i in 0..links.length() {
        if let Some(node) = links.item(i) {
            if let Ok(link) = node.dyn_into::<web_sys::HtmlElement>() {
                let link_c = link.clone();
                let cb = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
                    e.prevent_default();
                    let href = link_c.get_attribute("href").unwrap_or_default();
                    if href.starts_with('#') {
                        let id = &href[1..];
                        let win = web_sys::window().unwrap();
                        let doc = win.document().unwrap();
                        if let Ok(Some(target)) = doc.query_selector(&format!("#{}", id)) {
                            let rect = target.get_bounding_client_rect();
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
                }) as Box<dyn FnMut(_)>);
                link.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
                cb.forget();
            }
        }
    }
}

fn setup_toc_scroll_spy(toc: &Element) {
    let _doc = web_sys::window().unwrap().document().unwrap();
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

    // encontra o viewport do markdown-content no mesmo data-rs-markdown pai do TOC
    let scroll_viewport = toc.closest("[data-rs-md-layout]").ok().flatten()
        .and_then(|layout| layout.query_selector("[data-rs-markdown-content]").ok().flatten())
        .and_then(|content_el| {
            let mut el: Option<web_sys::Element> = Some(content_el);
            while let Some(parent) = el {
                if parent.has_attribute("data-rs-scroll-viewport") {
                    return Some(parent);
                }
                el = parent.parent_element();
            }
            None
        });

    let toc_c = toc.clone();
    let hids = heading_ids.clone();
    let last_active = std::rc::Rc::new(std::cell::RefCell::new(None::<String>));
    let la_cb = last_active.clone();
    let vp_c = scroll_viewport.clone();

    let on_scroll = Closure::wrap(Box::new(move || {
        web_sys::console::log_1(&"[toc] scroll fired".into());
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
                    el.set_attribute("data-rs-state", "idle").ok();
                }
            }
        }
        let selector = format!("[data-rs-toc-item][data-rs-target=\'{}\']", id);
        if let Ok(Some(item)) = toc_c.query_selector(&selector) {
            item.set_attribute("data-rs-state", "active").ok();
        }
    }) as Box<dyn Fn()>);

    // debug
    // registra no scroll viewport do conteudo
    let binding = web_sys::window().unwrap();
    let target: &web_sys::EventTarget = match &scroll_viewport {
        Some(el) => el.as_ref(),
        None => binding.as_ref(),
    };
    target.add_event_listener_with_callback_and_bool("scroll", on_scroll.as_ref().unchecked_ref(), true).ok();
    on_scroll.forget();
}

fn _setup_toc_scroll_spy_unused(toc: &Element) {
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

    let toc_c = toc.clone();
    let hids = heading_ids.clone();
    let last_active = std::rc::Rc::new(std::cell::RefCell::new(None::<String>));
    let la_cb = last_active.clone();

    let cb = Closure::wrap(Box::new(move |entries: js_sys::Array, _: web_sys::IntersectionObserver| {
        let mut intersecting: Vec<(f64, String)> = Vec::new();
        for i in 0..entries.length() {
            if let Ok(entry) = entries.get(i).dyn_into::<web_sys::IntersectionObserverEntry>() {
                if entry.is_intersecting() {
                    if let Some(id) = entry.target().get_attribute("id") {
                        intersecting.push((entry.bounding_client_rect().top(), id));
                    }
                }
            }
        }
        if intersecting.is_empty() { return; }
        intersecting.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        let ids: Vec<String> = intersecting.into_iter().map(|(_, id)| id).collect();
        let active_id = hids.iter().find(|id| ids.contains(id)).cloned();
        let Some(id) = active_id else { return };
        if la_cb.borrow().as_deref() == Some(&id) { return; }
        *la_cb.borrow_mut() = Some(id.clone());
        if let Ok(all) = toc_c.query_selector_all("[data-rs-toc-item]") {
            for j in 0..all.length() {
                if let Some(el) = all.item(j).and_then(|n| n.dyn_into::<Element>().ok()) {
                    el.set_attribute("data-rs-state", "idle").ok();
                }
            }
        }
        let selector = format!("[data-rs-toc-item][data-rs-target='{}']", id);
        if let Ok(Some(item)) = toc_c.query_selector(&selector) {
            item.set_attribute("data-rs-state", "active").ok();
        }
    }) as Box<dyn FnMut(js_sys::Array, web_sys::IntersectionObserver)>);

    let opts = web_sys::IntersectionObserverInit::new();
    opts.set_root_margin("0px 0px -50% 0px");
    let threshold = js_sys::Array::new();
    threshold.push(&JsValue::from_f64(0.0));
    opts.set_threshold(&threshold);
    // usa scroll viewport como root se existir
    if let Ok(Some(viewport)) = web_sys::window().unwrap().document().unwrap()
        .query_selector("[data-rs-scroll-viewport]") {
        opts.set_root(Some(&viewport));
        opts.set_root_margin("0px 0px -50% 0px");
    }

    if let Ok(observer) = web_sys::IntersectionObserver::new_with_options(cb.as_ref().unchecked_ref(), &opts) {
        for id in &heading_ids {
            let doc2 = web_sys::window().unwrap().document().unwrap();
            if let Ok(Some(h)) = doc2.query_selector(&format!("#{}", id)) {
                if let Ok(el) = h.dyn_into::<Element>() {
                    observer.observe(&el);
                }
            }
        }
    }
    cb.forget();
}

fn try_init_all(doc: &web_sys::Document) {
    let Ok(nodes) = doc.query_selector_all("[data-rs-markdown]") else { return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init(el); }
        }
    }
}

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    try_init_all(&doc);
    let doc_obs = doc.clone();
    let cb = Closure::wrap(Box::new(move |_: js_sys::Array, _: web_sys::MutationObserver| {
        try_init_all(&doc_obs);
    }) as Box<dyn FnMut(_, _)>);
    let observer = match web_sys::MutationObserver::new(cb.as_ref().unchecked_ref()) {
        Ok(o) => o, Err(_) => { cb.forget(); return }
    };
    let opts = web_sys::MutationObserverInit::new();
    opts.set_child_list(true); opts.set_subtree(true);
    if let Some(body) = doc.body() { observer.observe_with_options(&body, &opts).ok(); }
    cb.forget();
    let obs_clone = observer.clone();
    let disconnect = Closure::wrap(Box::new(move || { obs_clone.disconnect(); }) as Box<dyn Fn()>);
    win.set_timeout_with_callback_and_timeout_and_arguments_0(disconnect.as_ref().unchecked_ref(), 5000).ok();
    disconnect.forget();
}
