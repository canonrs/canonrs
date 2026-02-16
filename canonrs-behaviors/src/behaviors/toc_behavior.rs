//! TableOfContents behavior
//! Handles 3 modes: simple | expand | nested
//! Scroll-spy via IntersectionObserver

#[cfg(feature = "hydrate")]
use super::register_behavior;
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;
#[cfg(feature = "hydrate")]
use leptos::leptos_dom::helpers::document;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::Element;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-toc", Box::new(|element_id, _state| {
        let Some(toc) = document().get_element_by_id(element_id) else {
            return Ok(());
        };

        let mode = toc.get_attribute("data-toc-mode")
            .unwrap_or_else(|| "simple".to_string());

        setup_scroll_spy(&toc, &mode)?;

        if mode == "nested" {
            setup_nested_expand(&toc)?;
        }

        Ok(())
    }));
}

// ─── SCROLL SPY ──────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn setup_scroll_spy(toc: &Element, mode: &str) -> BehaviorResult<()> {
    if toc.get_attribute("data-spy-attached").as_deref() == Some("1") {
        return Ok(());
    }
    let _ = toc.set_attribute("data-spy-attached", "1");

    // Find all heading targets from toc items
    let items = toc.query_selector_all("[data-toc-item][data-target]")
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "query failed".into() })?;

    if items.length() == 0 { return Ok(()); }

    let toc_clone = toc.clone();
    let mode_owned = mode.to_string();

    let callback = Closure::wrap(Box::new(move |entries: js_sys::Array, _: web_sys::IntersectionObserver| {
        for i in 0..entries.length() {
            let entry = entries.get(i);
            let Ok(entry) = entry.dyn_into::<web_sys::IntersectionObserverEntry>() else { continue };
            if !entry.is_intersecting() { continue }

            let target = entry.target();
            let Some(id): Option<String> = target.get_attribute("id") else { continue };
            if id.is_empty() { continue }

            // Reset all items
            if let Ok(all_items) = toc_clone.query_selector_all("[data-toc-item]") {
                for j in 0..all_items.length() {
                    if let Some(item) = all_items.item(j) {
                        if let Ok(el) = item.dyn_into::<Element>() {
                            let _ = el.set_attribute("data-state", "idle");
                        }
                    }
                }
            }

            // Set active on matching item
            let selector = format!("[data-toc-item][data-target='{}']", id);
            if let Ok(Some(item)) = toc_clone.query_selector(&selector) {
                let _ = item.set_attribute("data-state", "active");

                // Mode: expand — reveal siblings (same parent level)
                if mode_owned == "expand" {
                    reveal_siblings_expand(&toc_clone, &item);
                }

                // Mode: nested — open parent subtrees
                if mode_owned == "nested" {
                    expand_parent_subtrees(&item);
                }
            }
        }
    }) as Box<dyn FnMut(js_sys::Array, web_sys::IntersectionObserver)>);

    let mut opts = web_sys::IntersectionObserverInit::new();
    opts.root_margin("-20% 0px -70% 0px");
    let threshold = js_sys::Array::new();
    threshold.push(&JsValue::from_f64(0.0));
    opts.threshold(&threshold);

    let observer = web_sys::IntersectionObserver::new_with_options(
        callback.as_ref().unchecked_ref(),
        &opts,
    ).map_err(|_| canonrs_shared::BehaviorError::JsError { message: "observer failed".into() })?;

    callback.forget();

    // Observe heading elements from document
    for i in 0..items.length() {
        if let Some(item) = items.item(i) {
            if let Ok(el) = item.dyn_into::<Element>() {
                if let Some(target_id) = el.get_attribute("data-target") {
                    let heading_selector = format!("#{}", target_id);
                    if let Ok(Some(heading)) = document().query_selector(&heading_selector) {
                        observer.observe(&heading);
                    }
                }
            }
        }
    }

    Ok(())
}

// ─── MODE: EXPAND ────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn reveal_siblings_expand(toc: &Element, active_item: &Element) {
    let active_level = active_item
        .get_attribute("data-level")
        .and_then(|l| l.parse::<u8>().ok())
        .unwrap_or(2);

    // Hide all children
    if let Ok(all_children) = toc.query_selector_all("[data-toc-item][data-child='true']") {
        for i in 0..all_children.length() {
            if let Some(child) = all_children.item(i) {
                if let Ok(el) = child.dyn_into::<Element>() {
                    let _ = el.remove_attribute("data-visible");
                }
            }
        }
    }

    // Show children of active parent
    // Find the nearest parent at level <= active_level - 1
    if active_level <= 2 { return; }

    // Show all children that follow the active item at deeper levels
    if let Ok(all_items) = toc.query_selector_all("[data-toc-item]") {
        let mut found_active = false;
        let parent_level = active_level - 1;

        for i in 0..all_items.length() {
            if let Some(item) = all_items.item(i) {
                if let Ok(el) = item.dyn_into::<Element>() {
                    if el == *active_item {
                        found_active = true;
                    }

                    if found_active {
                        let item_level = el.get_attribute("data-level")
                            .and_then(|l| l.parse::<u8>().ok())
                            .unwrap_or(2);

                        if item_level <= parent_level && el != *active_item {
                            break; // left the parent group
                        }

                        if item_level > parent_level {
                            let _ = el.set_attribute("data-visible", "true");
                        }
                    }
                }
            }
        }
    }
}

// ─── MODE: NESTED — AUTO EXPAND PARENTS ──────────────────────────────────────

#[cfg(feature = "hydrate")]
fn expand_parent_subtrees(active_item: &Element) {
    // Walk up DOM — open any data-toc-subtree ancestor
    let mut current = active_item.parent_element();
    while let Some(parent) = current {
        if parent.has_attribute("data-toc-subtree") {
            let _ = parent.set_attribute("data-state", "open");
            // Update expand button
            if let Some(grandparent) = parent.parent_element() {
                if let Ok(Some(btn)) = grandparent.query_selector("[data-toc-expand-btn]") {
                    let _ = btn.set_attribute("aria-expanded", "true");
                }
            }
        }
        current = parent.parent_element();
    }
}

// ─── MODE: NESTED — MANUAL EXPAND ────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn setup_nested_expand(toc: &Element) -> BehaviorResult<()> {
    if toc.get_attribute("data-nested-attached").as_deref() == Some("1") {
        return Ok(());
    }
    let _ = toc.set_attribute("data-nested-attached", "1");

    let buttons = toc.query_selector_all("[data-toc-expand-btn]")
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "query failed".into() })?;

    for i in 0..buttons.length() {
        if let Some(btn) = buttons.item(i) {
            if let Ok(btn_el) = btn.dyn_into::<Element>() {
                let btn_clone = btn_el.clone();
                let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    let is_expanded = btn_clone.get_attribute("aria-expanded")
                        .as_deref() == Some("true");
                    let next = if is_expanded { "false" } else { "true" };
                    let _ = btn_clone.set_attribute("aria-expanded", next);

                    // Toggle sibling subtree
                    if let Some(parent) = btn_clone.parent_element() {
                        if let Ok(Some(subtree)) = parent.query_selector("[data-toc-subtree]") {
                            let state = if is_expanded { "closed" } else { "open" };
                            let _ = subtree.set_attribute("data-state", state);
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                btn_el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                    .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "btn listener failed".into() })?;
                closure.forget();
            }
        }
    }

    Ok(())
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
