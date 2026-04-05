use leptos::prelude::*;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TocIslandItem {
    pub id:    String,
    pub text:  String,
    pub level: u8,
}

#[island]
pub fn TableOfContentsIsland(
    items: Vec<TocIslandItem>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] mode: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let title = title.unwrap_or_else(|| "On this page".to_string());
    let mode  = mode.unwrap_or_else(|| "simple".to_string());
    let container_ref = NodeRef::<leptos::html::Nav>::new();

    #[cfg(feature = "hydrate")]
    {
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;

        let mode_clone = mode.clone();
        Effect::new(move |_| {
            if let Some(nav) = container_ref.get() {
                let el = nav.unchecked_into::<web_sys::Element>();
                let mode = mode_clone.clone();
                setup_anchor_click(&el);
                attempt_scroll_spy(el, mode, 0);
            }
        });
    }

    let initial_state = "idle";

    let items_view = items.iter().map(|item| {
        let href  = format!("#{}", item.id);
        let id    = item.id.clone();
        let text  = item.text.clone();
        let level = item.level.to_string();
        let is_child = item.level > 2;
        view! {
            <li
                data-rs-toc-item=""
                data-rs-target=id
                data-rs-level=level
                data-rs-state=initial_state
                data-child=is_child.to_string()
            >
                <a data-rs-toc-link="" href=href>{text}</a>
            </li>
        }
    }).collect::<Vec<_>>();

    view! {
        <nav
            data-rs-toc=""
            data-rs-component="TableOfContents"
            data-rs-mode=mode
            class=class
            node_ref=container_ref
        >
            <p data-rs-toc-title="">{title}</p>
            <ul data-rs-toc-list="">
                {items_view}
            </ul>
        </nav>
    }
}

// ─── ANCHOR CLICK ────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn setup_anchor_click(toc: &leptos::web_sys::Element) {
    use leptos::wasm_bindgen::JsCast;
    use leptos::wasm_bindgen::closure::Closure;
    use leptos::web_sys;

    if toc.get_attribute("data-rs-toc-anchor-attached").as_deref() == Some("1") { return; }
    let _ = toc.set_attribute("data-rs-toc-anchor-attached", "1");

    let links = match toc.query_selector_all("[data-rs-toc-link]") {
        Ok(nl) => nl, Err(_) => return,
    };

    for i in 0..links.length() {
        if let Some(link) = links.item(i) {
            if let Ok(link_el) = link.dyn_into::<web_sys::HtmlElement>() {
                let link_clone = link_el.clone();
                let cb = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
                    e.prevent_default();
                    let href = link_clone.get_attribute("href").unwrap_or_default();
                    if href.starts_with('#') {
                        let id = &href[1..];
                        let window = match web_sys::window() { Some(w) => w, None => return };
                        let document = window.document().unwrap();
                        if let Ok(Some(target)) = document.query_selector(&format!("#{}", id)) {
                            let rect = target.get_bounding_client_rect();
                            let scroll_y = window.scroll_y().unwrap_or(0.0);
                            let top = rect.top() + scroll_y - 80.0;
                            let opts = web_sys::ScrollToOptions::new();
                            opts.set_top(top);
                            opts.set_behavior(web_sys::ScrollBehavior::Smooth);
                            window.scroll_to_with_scroll_to_options(&opts);
                            if let Ok(history) = window.history() {
                                let _ = history.replace_state_with_url(&leptos::wasm_bindgen::JsValue::NULL, "", Some(&format!("#{}", id)));
                            }
                        }
                    }
                }) as Box<dyn FnMut(_)>);
                let _ = link_el.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
                cb.forget();
            }
        }
    }
}

// ─── RETRY ───────────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn attempt_scroll_spy(toc: leptos::web_sys::Element, mode: String, attempt: u32) {
    use leptos::wasm_bindgen::JsCast;
    use leptos::web_sys;
    let wdoc = leptos::web_sys::window().unwrap().document().unwrap();

    if attempt > 5 { let _ = setup_scroll_spy(&toc, &mode); return; }
    if toc.get_attribute("data-rs-toc-attached").as_deref() == Some("1") { return; }

    let items = match toc.query_selector_all("[data-rs-toc-item][data-rs-target]") {
        Ok(nl) => nl, Err(_) => return,
    };
    if items.length() == 0 { retry_scroll_spy(toc, mode, attempt); return; }

    let mut found_any = false;
    for i in 0..items.length() {
        if let Some(item) = items.item(i) {
            if let Ok(el) = item.dyn_into::<web_sys::Element>() {
                if let Some(target_id) = el.get_attribute("data-rs-target") {
                    if let Ok(Some(h)) = wdoc.query_selector(&format!("#{}", target_id)) {
                        if let Ok(el) = h.dyn_into::<web_sys::Element>() { if el.is_connected() { found_any = true; break; } }
                    }
                }
            }
        }
    }

    if !found_any { retry_scroll_spy(toc, mode, attempt); return; }
    if setup_scroll_spy(&toc, &mode).is_err() { retry_scroll_spy(toc, mode, attempt); }
}

#[cfg(feature = "hydrate")]
fn retry_scroll_spy(toc: leptos::web_sys::Element, mode: String, attempt: u32) {
    use leptos::wasm_bindgen::closure::Closure;
    use leptos::wasm_bindgen::JsCast;

    let window = match leptos::web_sys::window() { Some(w) => w, None => return };
    let delay = match attempt { 0 => 50, 1 => 150, 2 => 300, 3 => 500, _ => 1000 };
    let cb = Closure::once_into_js(move || attempt_scroll_spy(toc, mode, attempt + 1));
    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(cb.unchecked_ref(), delay);
}

// ─── SCROLL SPY ──────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn setup_scroll_spy(toc: &leptos::web_sys::Element, mode: &str) -> Result<(), ()> {
    use leptos::wasm_bindgen::prelude::*;
    use leptos::wasm_bindgen::JsCast;
    use leptos::web_sys;

    if toc.get_attribute("data-rs-toc-attached").as_deref() == Some("1") { return Ok(()); }
    let _ = toc.set_attribute("data-rs-toc-attached", "1");

    let items = toc.query_selector_all("[data-rs-toc-item][data-rs-target]").map_err(|_| ())?;
    if items.length() == 0 { return Ok(()); }

    let mut heading_ids: Vec<String> = Vec::new();
    for i in 0..items.length() {
        if let Some(item) = items.item(i) {
            if let Ok(el) = item.dyn_into::<web_sys::Element>() {
                if let Some(id) = el.get_attribute("data-rs-target") {
                    heading_ids.push(id);
                }
            }
        }
    }

    let toc_clone   = toc.clone();
    let mode_owned  = mode.to_string();
    let hids        = heading_ids.clone();
    let last_active = std::rc::Rc::new(std::cell::RefCell::new(None::<String>));
    let last_scroll = std::rc::Rc::new(std::cell::RefCell::new(0.0f64));
    let la_cb = last_active.clone();
    let ls_cb = last_scroll.clone();

    let callback = Closure::wrap(Box::new(move |entries: js_sys::Array, _: web_sys::IntersectionObserver| {
        let current_y = web_sys::window().and_then(|w| w.scroll_y().ok()).unwrap_or(0.0);
        let scrolling_down = current_y >= *ls_cb.borrow();
        *ls_cb.borrow_mut() = current_y;

        let mut intersecting: Vec<(f64, String)> = Vec::new();
        for i in 0..entries.length() {
            if let Ok(entry) = entries.get(i).dyn_into::<web_sys::IntersectionObserverEntry>() {
                if entry.is_intersecting() {
                    if let Some(id) = entry.target().get_attribute("id") {
                        if !id.is_empty() {
                            intersecting.push((entry.bounding_client_rect().top(), id));
                        }
                    }
                }
            }
        }
        if intersecting.is_empty() { return; }
        intersecting.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        let ids: Vec<String> = intersecting.into_iter().map(|(_, id)| id).collect();

        let active_id = if scrolling_down {
            hids.iter().find(|id| ids.contains(id)).cloned()
        } else {
            hids.iter().rev().find(|id| ids.contains(id)).cloned()
        };
        let Some(id) = active_id else { return };
        if la_cb.borrow().as_deref() == Some(&id) { return; }
        *la_cb.borrow_mut() = Some(id.clone());

        if let Ok(all) = toc_clone.query_selector_all("[data-rs-toc-item]") {
            for j in 0..all.length() {
                if let Some(el) = all.item(j) {
                    if let Ok(e) = el.dyn_into::<web_sys::Element>() {
                        let _ = e.set_attribute("data-rs-state", "idle");
                    }
                }
            }
        }

        let selector = format!("[data-rs-toc-item][data-rs-target='{}']", id);
        if let Ok(Some(item)) = toc_clone.query_selector(&selector) {
            let _ = item.set_attribute("data-rs-state", "active");
            let _ = toc_clone.set_attribute("data-rs-active-heading", &id);
            if let Some(window) = web_sys::window() {
                if let Ok(history) = window.history() {
                    let _ = history.replace_state_with_url(&JsValue::NULL, "", Some(&format!("#{}", id)));
                }
            }
            set_ancestors_active(&toc_clone, &item);
            if mode_owned == "nested" { expand_parent_subtrees(&toc_clone, &item); }
        }
    }) as Box<dyn FnMut(js_sys::Array, web_sys::IntersectionObserver)>);

    let opts = web_sys::IntersectionObserverInit::new();
    opts.set_root_margin("-80px 0px -60% 0px");
    let threshold = js_sys::Array::new();
    threshold.push(&JsValue::from_f64(0.0));
    opts.set_threshold(&threshold);

    if let Some(window) = web_sys::window() {
        if let Ok(existing) = js_sys::Reflect::get(&window, &"__tocObserver".into()) {
            if let Ok(obs) = existing.dyn_into::<web_sys::IntersectionObserver>() { obs.disconnect(); }
        }
    }

    let observer = web_sys::IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &opts).map_err(|_| ())?;
    callback.forget();

    if let Some(window) = web_sys::window() {
        let _ = js_sys::Reflect::set(&window, &"__tocObserver".into(), &JsValue::from(observer.clone()));
    }

    for id in &heading_ids {
        let wdoc2 = leptos::web_sys::window().unwrap().document().unwrap();
        if let Ok(Some(h)) = wdoc2.query_selector(&format!("#{}", id)) {
            if let Ok(el) = h.dyn_into::<web_sys::Element>() { if el.is_connected() { let _ = observer.observe(&el); } }
        }
    }
    Ok(())
}

// ─── ANCESTOR HIGHLIGHT ───────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn set_ancestors_active(toc: &leptos::web_sys::Element, active_item: &leptos::web_sys::Element) {
    use leptos::wasm_bindgen::JsCast;
    use leptos::web_sys;

    let active_level = active_item.get_attribute("data-rs-level")
        .and_then(|l| l.parse::<i32>().ok()).unwrap_or(2);
    if active_level <= 1 { return; }

    let mut last_at_level: [Option<web_sys::Element>; 7] = Default::default();
    if let Ok(all) = toc.query_selector_all("[data-rs-toc-item]") {
        for j in 0..all.length() {
            if let Some(item) = all.item(j) {
                if let Ok(el) = item.dyn_into::<web_sys::Element>() {
                    if el.is_same_node(Some(active_item)) {
                        for pl in 1..active_level {
                            if let Some(anc) = &last_at_level[pl as usize] {
                                let _ = anc.set_attribute("data-rs-state", "ancestor");
                            }
                        }
                        break;
                    }
                    if let Some(lvl) = el.get_attribute("data-rs-level").and_then(|l| l.parse::<i32>().ok()) {
                        if lvl > 0 && lvl < 7 { last_at_level[lvl as usize] = Some(el); }
                    }
                }
            }
        }
    }
}

// ─── NESTED EXPAND ───────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn expand_parent_subtrees(toc: &leptos::web_sys::Element, active_item: &leptos::web_sys::Element) {
    use leptos::wasm_bindgen::JsCast;
    use leptos::web_sys;

    let mut ancestors: Vec<web_sys::Element> = Vec::new();
    let mut current = active_item.parent_element();
    while let Some(parent) = current {
        if parent.has_attribute("data-rs-toc-subtree") { ancestors.push(parent.clone()); }
        current = parent.parent_element();
    }

    if let Ok(all) = toc.query_selector_all("[data-rs-toc-subtree]") {
        for i in 0..all.length() {
            if let Some(st) = all.item(i) {
                if let Ok(el) = st.dyn_into::<web_sys::Element>() {
                    let is_anc = ancestors.iter().any(|a| a.is_same_node(Some(&el)));
                    let state = if is_anc { "open" } else { "closed" };
                    let _ = el.set_attribute("data-rs-state", state);
                    if let Some(gp) = el.parent_element() {
                        if let Ok(Some(btn)) = gp.query_selector("[data-rs-toc-expand-btn]") {
                            let _ = btn.set_attribute("aria-expanded", if is_anc { "true" } else { "false" });
                            let _ = btn.set_attribute("data-rs-state", state);
                        }
                    }
                }
            }
        }
    }
}
