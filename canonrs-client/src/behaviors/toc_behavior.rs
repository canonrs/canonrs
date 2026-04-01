//! TableOfContents behavior v2 — enterprise
//! - scroll spy com direction awareness
//! - URL sync (#hash) sem push de history
//! - scroll to anchor com offset fixo (navbar)
//! - active range (heading atual até o próximo)
//! - debounce no scroll listener
//! - highlight progressivo via ancestor state
//! - retry automático com backoff
//! - observer persistido no window (anti-GC)

#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;
#[cfg(feature = "hydrate")]
use leptos::leptos_dom::helpers::document;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::Element;

/// Offset fixo para compensar navbar fixa (px)
#[cfg(feature = "hydrate")]
const SCROLL_OFFSET: f64 = 80.0;

/// Root margin do IntersectionObserver
#[cfg(feature = "hydrate")]
const ROOT_MARGIN: &str = "-80px 0px -60% 0px";

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-toc", Box::new(|root: &web_sys::Element, _state: &ComponentState| {
        let mode = root.get_attribute("data-rs-mode")
            .unwrap_or_else(|| "simple".to_string());

        if mode == "nested" {
            setup_nested_expand(root)?;
        }

        setup_anchor_click(root)?;
        attempt_scroll_spy(root.clone(), mode, 0);

        Ok(())
    }));
}

// ─── ANCHOR CLICK — scroll com offset ────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn setup_anchor_click(toc: &Element) -> BehaviorResult<()> {
    if toc.get_attribute("data-rs-toc-anchor-attached").as_deref() == Some("1") {
        return Ok(());
    }
    let _ = toc.set_attribute("data-rs-toc-anchor-attached", "1");

    let links = toc.query_selector_all("[data-rs-toc-link]")
        .map_err(|_| crate::BehaviorError::JsError { message: "query failed".into() })?;

    for i in 0..links.length() {
        if let Some(link) = links.item(i) {
            if let Ok(link_el) = link.dyn_into::<web_sys::HtmlElement>() {
                let link_el_clone = link_el.clone();
                let closure = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
                    let link_el = &link_el_clone;
                    e.prevent_default();
                    let href = link_el.get_attribute("href").unwrap_or_default();
                    if href.starts_with('#') {
                        let id = &href[1..];
                        let sel = format!("#{}", id);
                        if let Ok(Some(target)) = document().query_selector(&sel) {
                            let rect = target.get_bounding_client_rect();
                            let window = web_sys::window().unwrap();
                            let scroll_y = window.scroll_y().unwrap_or(0.0);
                            let top = rect.top() + scroll_y - SCROLL_OFFSET;
                            let opts = web_sys::ScrollToOptions::new();
                            opts.set_top(top);
                            opts.set_behavior(web_sys::ScrollBehavior::Smooth);
                            window.scroll_to_with_scroll_to_options(&opts);
                            // URL sync sem push de history
                            if let Ok(Some(history)) = window.history().map(|h| Some(h)) {
                                let url = format!("#{}", id);
                                let _ = history.replace_state_with_url(
                                    &JsValue::NULL, "", Some(&url)
                                );
                            }
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                link_el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                    .map_err(|_| crate::BehaviorError::JsError { message: "link listener failed".into() })?;
                closure.forget();
            }
        }
    }

    Ok(())
}

// ─── RETRY LOGIC ─────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn attempt_scroll_spy(toc: Element, mode: String, attempt: u32) {
    if attempt > 5 {
        let _ = setup_scroll_spy(&toc, &mode);
        return;
    }

    if toc.get_attribute("data-rs-toc-attached").as_deref() == Some("1") {
        return;
    }

    let items = match toc.query_selector_all("[data-rs-toc-item][data-rs-target]") {
        Ok(nl) => nl,
        Err(_) => return,
    };

    if items.length() == 0 {
        retry_scroll_spy(toc, mode, attempt);
        return;
    }

    let mut found_any = false;
    for i in 0..items.length() {
        if let Some(item) = items.item(i) {
            if let Ok(el) = item.dyn_into::<Element>() {
                if let Some(target_id) = el.get_attribute("data-rs-target") {
                    let sel = format!("#{}", target_id);
                    if let Ok(Some(h)) = document().query_selector(&sel) {
                        if h.is_connected() {
                            found_any = true;
                            break;
                        }
                    }
                }
            }
        }
    }

    if !found_any {
        retry_scroll_spy(toc, mode, attempt);
        return;
    }

    if let Err(_) = setup_scroll_spy(&toc, &mode) {
        retry_scroll_spy(toc, mode, attempt);
    }
}

#[cfg(feature = "hydrate")]
fn retry_scroll_spy(toc: Element, mode: String, attempt: u32) {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let delay = match attempt {
        0 => 50,
        1 => 150,
        2 => 300,
        3 => 500,
        _ => 1000,
    };
    let closure = Closure::once_into_js(move || {
        attempt_scroll_spy(toc, mode, attempt + 1);
    });
    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
        closure.unchecked_ref(),
        delay,
    );
}

// ─── SCROLL SPY v2 ────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn setup_scroll_spy(toc: &Element, mode: &str) -> BehaviorResult<()> {
    if toc.get_attribute("data-rs-toc-attached").as_deref() == Some("1") {
        return Ok(());
    }
    let _ = toc.set_attribute("data-rs-toc-attached", "1");

    let items = toc.query_selector_all("[data-rs-toc-item][data-rs-target]")
        .map_err(|_| crate::BehaviorError::JsError { message: "query failed".into() })?;

    if items.length() == 0 { return Ok(()); }

    // Cache: coleta todos os heading IDs na ordem do TOC
    let mut heading_ids: Vec<String> = Vec::new();
    for i in 0..items.length() {
        if let Some(item) = items.item(i) {
            if let Ok(el) = item.dyn_into::<Element>() {
                if let Some(id) = el.get_attribute("data-rs-target") {
                    heading_ids.push(id);
                }
            }
        }
    }

    let toc_clone = toc.clone();
    let mode_owned = mode.to_string();
    let heading_ids_clone = heading_ids.clone();

    // Direction awareness — rastreia último Y do scroll
    let last_active: std::rc::Rc<std::cell::RefCell<Option<String>>> =
        std::rc::Rc::new(std::cell::RefCell::new(None));
    let last_active_cb = last_active.clone();
    let last_scroll_y: std::rc::Rc<std::cell::RefCell<f64>> =
        std::rc::Rc::new(std::cell::RefCell::new(0.0));
    let last_scroll_y_cb = last_scroll_y.clone();

    let callback = Closure::wrap(Box::new(move |entries: js_sys::Array, _: web_sys::IntersectionObserver| {
        // Direction awareness
        let current_y = web_sys::window()
            .and_then(|w| w.scroll_y().ok())
            .unwrap_or(0.0);
        let scrolling_down = current_y >= *last_scroll_y_cb.borrow();
        *last_scroll_y_cb.borrow_mut() = current_y;

        // Collect intersecting entries com posição real
        let mut intersecting: Vec<(f64, String)> = Vec::new();
        for i in 0..entries.length() {
            let entry = entries.get(i);
            let Ok(entry) = entry.dyn_into::<web_sys::IntersectionObserverEntry>() else { continue };
            if entry.is_intersecting() {
                if let Some(id) = entry.target().get_attribute("id") {
                    if !id.is_empty() {
                        let top = entry.bounding_client_rect().top();
                        intersecting.push((top, id));
                    }
                }
            }
        }

        if intersecting.is_empty() { return; }

        // Ordenar por posição: scrolling down → primeiro heading visível (menor top)
        // scrolling up → último heading visível (maior top ainda acima do fold)
        intersecting.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

        let intersecting_ids: Vec<String> = intersecting.into_iter().map(|(_, id)| id).collect();

        // Active range: respeitar ordem do documento
        let active_id = if scrolling_down {
            heading_ids_clone.iter().find(|id| intersecting_ids.contains(id)).cloned()
        } else {
            heading_ids_clone.iter().rev().find(|id| intersecting_ids.contains(id)).cloned()
        };

        let Some(id) = active_id else { return };

        // Evitar re-render desnecessário
        {
            let last = last_active_cb.borrow();
            if last.as_deref() == Some(&id) { return; }
        }
        *last_active_cb.borrow_mut() = Some(id.clone());

        // Reset all items
        if let Ok(all_items) = toc_clone.query_selector_all("[data-rs-toc-item]") {
            for j in 0..all_items.length() {
                if let Some(item) = all_items.item(j) {
                    if let Ok(el) = item.dyn_into::<Element>() {
                        let _ = el.set_attribute("data-rs-state", "idle");
                    }
                }
            }
        }

        // Set active
        let selector = format!("[data-rs-toc-item][data-rs-target='{}']", id);
        if let Ok(Some(item)) = toc_clone.query_selector(&selector) {
            let _ = item.set_attribute("data-rs-state", "active");
            let _ = toc_clone.set_attribute("data-rs-active-heading", &id);
            update_section_progress(&item, &id);

            // URL sync — replace state sem push
            if let Some(window) = web_sys::window() {
                if let Ok(history) = window.history() {
                    let url = format!("#{}", id);
                    let _ = history.replace_state_with_url(&JsValue::NULL, "", Some(&url));
                }
            }

            // Ancestor highlight progressivo
            set_ancestors_active(&toc_clone, &item);

            if mode_owned == "expand" {
                reveal_siblings_expand(&toc_clone, &item);
            }

            if mode_owned == "nested" {
                expand_parent_subtrees(&toc_clone, &item);
            }
        }
    }) as Box<dyn FnMut(js_sys::Array, web_sys::IntersectionObserver)>);

    let opts = web_sys::IntersectionObserverInit::new();
    opts.set_root_margin(ROOT_MARGIN);
    let threshold = js_sys::Array::new();
    threshold.push(&JsValue::from_f64(0.0));
    opts.set_threshold(&threshold);

    // Cleanup observer anterior
    if let Some(window) = web_sys::window() {
        if let Ok(existing) = js_sys::Reflect::get(&window, &"__tocObserver".into()) {
            if let Ok(obs) = existing.dyn_into::<web_sys::IntersectionObserver>() {
                obs.disconnect();
            }
        }
    }

    let observer = web_sys::IntersectionObserver::new_with_options(
        callback.as_ref().unchecked_ref(),
        &opts,
    ).map_err(|_| crate::BehaviorError::JsError { message: "observer failed".into() })?;

    callback.forget();

    // Persistir observer
    if let Some(window) = web_sys::window() {
        let observer_ref = JsValue::from(observer.clone());
        let _ = js_sys::Reflect::set(&window, &"__tocObserver".into(), &observer_ref);
    }

    // Observe headings — apenas os conectados
    for id in &heading_ids {
        let sel = format!("#{}", id);
        if let Ok(Some(heading)) = document().query_selector(&sel) {
            if heading.is_connected() {
                observer.observe(&heading);
            }
        }
    }

    Ok(())
}

// ─── SECTION PROGRESS + READING POSITION ────────────────────────────────────

#[cfg(feature = "hydrate")]
fn update_section_progress(toc_item: &Element, heading_id: &str) {
    let window = match web_sys::window() { Some(w) => w, None => return };
    let doc    = document();

    // Heading atual
    let sel = format!("#{}", heading_id);
    let heading = match doc.query_selector(&sel).ok().flatten() { Some(h) => h, None => return };

    let heading_top = heading.get_bounding_client_rect().top()
        + window.scroll_y().unwrap_or(0.0);

    // Próximo heading do mesmo nível ou superior — define fim da seção
    let next_top = find_next_heading_top(&doc, &heading, heading_top);
    let section_height = (next_top - heading_top).max(1.0);

    let scroll_y   = window.scroll_y().unwrap_or(0.0);
    let viewport_h = window.inner_height().ok()
        .and_then(|v| v.as_f64()).unwrap_or(600.0);

    // Quanto da seção foi lida (0-100%)
    let read = ((scroll_y + viewport_h * 0.2 - heading_top) / section_height * 100.0)
        .clamp(0.0, 100.0);

    // Posição do ponto dentro do item do TOC (0-100%)
    let position = read.clamp(0.0, 100.0);

    if let Ok(item_el) = toc_item.clone().dyn_into::<web_sys::HtmlElement>() {
        let style = item_el.style();
        let _ = style.set_property("--toc-section-progress", &format!("{:.1}%", read));
        let _ = style.set_property("--toc-reading-position", &format!("{:.1}%", position));
    }
}

#[cfg(feature = "hydrate")]
fn find_next_heading_top(doc: &web_sys::Document, current: &Element, current_top: f64) -> f64 {
    let headings = match doc.query_selector_all("h1, h2, h3, h4, h5, h6").ok() {
        Some(nl) => nl,
        None => return current_top + 1000.0,
    };

    let mut found_current = false;
    for i in 0..headings.length() {
        if let Some(h) = headings.item(i) {
            if let Ok(el) = h.dyn_into::<Element>() {
                if el.is_same_node(Some(current)) {
                    found_current = true;
                    continue;
                }
                if found_current {
                    let top = el.get_bounding_client_rect().top()
                        + web_sys::window()
                            .and_then(|w| w.scroll_y().ok())
                            .unwrap_or(0.0);
                    return top;
                }
            }
        }
    }
    current_top + 2000.0
}

// ─── ANCESTOR HIGHLIGHT PROGRESSIVO ──────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn set_ancestors_active(toc: &Element, active_item: &Element) {
    let active_level = active_item
        .get_attribute("data-rs-level")
        .and_then(|l| l.parse::<i32>().ok())
        .unwrap_or(2);

    if active_level <= 1 { return; }

    let mut last_at_level: [Option<Element>; 7] = Default::default();

    if let Ok(all_items) = toc.query_selector_all("[data-rs-toc-item]") {
        for j in 0..all_items.length() {
            if let Some(toc_item) = all_items.item(j) {
                if let Ok(el) = toc_item.dyn_into::<Element>() {
                    if el.is_same_node(Some(active_item)) {
                        for parent_level in 1..active_level {
                            if let Some(ancestor) = &last_at_level[parent_level as usize] {
                                let _ = ancestor.set_attribute("data-rs-state", "ancestor");
                            }
                        }
                        break;
                    }
                    if let Some(lvl_str) = el.get_attribute("data-rs-level") {
                        if let Ok(lvl) = lvl_str.parse::<i32>() {
                            if lvl > 0 && lvl < 7 {
                                last_at_level[lvl as usize] = Some(el.clone());
                            }
                        }
                    }
                }
            }
        }
    }
}

// ─── MODE: EXPAND ────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn reveal_siblings_expand(toc: &Element, active_item: &Element) {
    let active_level = active_item
        .get_attribute("data-rs-level")
        .and_then(|l| l.parse::<u8>().ok())
        .unwrap_or(2);

    if let Ok(all_children) = toc.query_selector_all("[data-rs-toc-item][data-child='true']") {
        for i in 0..all_children.length() {
            if let Some(child) = all_children.item(i) {
                if let Ok(el) = child.dyn_into::<Element>() {
                    let _ = el.remove_attribute("data-rs-visible");
                }
            }
        }
    }

    if active_level <= 2 { return; }

    if let Ok(all_items) = toc.query_selector_all("[data-rs-toc-item]") {
        let mut found_active = false;
        let parent_level = active_level - 1;

        for i in 0..all_items.length() {
            if let Some(item) = all_items.item(i) {
                if let Ok(el) = item.dyn_into::<Element>() {
                    if el == *active_item { found_active = true; }
                    if found_active {
                        let item_level = el.get_attribute("data-rs-level")
                            .and_then(|l| l.parse::<u8>().ok())
                            .unwrap_or(2);
                        if item_level <= parent_level && el != *active_item { break; }
                        if item_level > parent_level {
                            let _ = el.set_attribute("data-rs-visible", "true");
                        }
                    }
                }
            }
        }
    }
}

// ─── MODE: NESTED — AUTO EXPAND PARENTS ──────────────────────────────────────

#[cfg(feature = "hydrate")]
fn expand_parent_subtrees(toc: &Element, active_item: &Element) {
    // Coletar subtrees ancestrais
    let mut ancestor_subtrees: Vec<Element> = Vec::new();
    let mut current = active_item.parent_element();
    while let Some(parent) = current {
        if parent.has_attribute("data-rs-toc-subtree") {
            ancestor_subtrees.push(parent.clone());
        }
        current = parent.parent_element();
    }

    // Fechar subtrees não-ancestrais
    if let Ok(all_subtrees) = toc.query_selector_all("[data-rs-toc-subtree]") {
        for i in 0..all_subtrees.length() {
            if let Some(st) = all_subtrees.item(i) {
                if let Ok(el) = st.dyn_into::<Element>() {
                    let is_ancestor = ancestor_subtrees.iter().any(|a| a.is_same_node(Some(&el)));
                    if !is_ancestor {
                        let _ = el.set_attribute("data-rs-state", "closed");
                        if let Some(gp) = el.parent_element() {
                            if let Ok(Some(btn)) = gp.query_selector("[data-rs-toc-expand-btn]") {
                                let _ = btn.set_attribute("aria-expanded", "false");
                                let _ = btn.set_attribute("data-rs-state", "closed");
                            }
                        }
                    }
                }
            }
        }
    }

    // Abrir ancestrais
    for subtree in &ancestor_subtrees {
        let _ = subtree.set_attribute("data-rs-state", "open");
        if let Some(grandparent) = subtree.parent_element() {
            if let Ok(Some(btn)) = grandparent.query_selector("[data-rs-toc-expand-btn]") {
                let _ = btn.set_attribute("aria-expanded", "true");
                let _ = btn.set_attribute("data-rs-state", "open");
            }
        }
    }
}

// ─── MODE: NESTED — MANUAL EXPAND ────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn setup_nested_expand(toc: &Element) -> BehaviorResult<()> {
    if toc.get_attribute("data-rs-toc-nested-attached").as_deref() == Some("1") {
        return Ok(());
    }
    let _ = toc.set_attribute("data-rs-toc-nested-attached", "1");

    if let Ok(all_subtrees) = toc.query_selector_all("[data-rs-toc-subtree]") {
        for i in 0..all_subtrees.length() {
            if let Some(st) = all_subtrees.item(i) {
                if let Ok(el) = st.dyn_into::<Element>() {
                    let _ = el.set_attribute("data-rs-state", "closed");
                }
            }
        }
    }

    let buttons = toc.query_selector_all("[data-rs-toc-expand-btn]")
        .map_err(|_| crate::BehaviorError::JsError { message: "query failed".into() })?;

    for i in 0..buttons.length() {
        if let Some(btn) = buttons.item(i) {
            if let Ok(btn_el) = btn.dyn_into::<Element>() {
                let btn_clone = btn_el.clone();
                let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    let is_expanded = btn_clone.get_attribute("aria-expanded")
                        .as_deref() == Some("true");
                    let next = if is_expanded { "false" } else { "true" };
                    let _ = btn_clone.set_attribute("aria-expanded", next);
                    let _ = btn_clone.set_attribute("data-rs-state",
                        if is_expanded { "closed" } else { "open" });

                    if let Some(parent) = btn_clone.parent_element() {
                        if let Ok(Some(subtree)) = parent.query_selector("[data-rs-toc-subtree]") {
                            let state = if is_expanded { "closed" } else { "open" };
                            let _ = subtree.set_attribute("data-rs-state", state);

                        }
                    }
                }) as Box<dyn FnMut(_)>);

                btn_el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                    .map_err(|_| crate::BehaviorError::JsError { message: "btn listener failed".into() })?;
                closure.forget();
            }
        }
    }

    Ok(())
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
