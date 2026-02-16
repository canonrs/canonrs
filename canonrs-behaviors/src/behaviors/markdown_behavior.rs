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
    register_behavior("data-markdown", Box::new(|element_id, _state| {
        let Some(container) = document().get_element_by_id(element_id) else {
            return Ok(());
        };
        setup_toc_toggle(&container)?;
        setup_scroll_spy(&container)?;
        Ok(())
    }));
}

// ─── TOC TOGGLE ──────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn setup_toc_toggle(container: &Element) -> BehaviorResult<()> {
    if container.get_attribute("data-toc-toggle-attached").as_deref() == Some("1") {
        return Ok(());
    }
    let _ = container.set_attribute("data-toc-toggle-attached", "1");

    let Some(btn) = container.query_selector("[data-md-toolbar-item][data-action='toggle-toc']")
        .ok().flatten() else { return Ok(()); };

    let container_clone = container.clone();

    let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
        let Some(toc) = container_clone.query_selector("[data-md-toc]").ok().flatten() else { return };
        let current = toc.get_attribute("data-state").unwrap_or_else(|| "open".to_string());
        let next = if current == "open" { "closed" } else { "open" };
        let _ = toc.set_attribute("data-state", next);
    }) as Box<dyn FnMut(_)>);

    btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "listener failed".into() })?;
    closure.forget();
    Ok(())
}

// ─── SCROLL SPY ──────────────────────────────────────────────────────────────

#[cfg(feature = "hydrate")]
fn setup_scroll_spy(container: &Element) -> BehaviorResult<()> {
    if container.get_attribute("data-scroll-spy-attached").as_deref() == Some("1") {
        return Ok(());
    }
    let _ = container.set_attribute("data-scroll-spy-attached", "1");

    let headings = container.query_selector_all("[data-md-heading]")
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "query failed".into() })?;

    if headings.length() == 0 {
        return Ok(());
    }

    let container_clone = container.clone();

    let callback = Closure::wrap(Box::new(move |entries: js_sys::Array, _: web_sys::IntersectionObserver| {
        for i in 0..entries.length() {
            let entry = entries.get(i);
            let Ok(entry) = entry.dyn_into::<web_sys::IntersectionObserverEntry>() else { continue };

            if !entry.is_intersecting() { continue }

            let target = entry.target();
            let Some(id): Option<String> = target.get_attribute("id") else { continue };
            if id.is_empty() { continue }

            // Remove active AND ancestor from all toc items
            if let Ok(all_links) = container_clone.query_selector_all("[data-toc-link]") {
                for j in 0..all_links.length() {
                    if let Some(link) = all_links.item(j) {
                        if let Ok(el) = link.dyn_into::<Element>() {
                            if let Some(li) = el.parent_element() {
                                let _ = li.set_attribute("data-state", "idle");
                            }
                        }
                    }
                }
            }

            // Set active on matching toc link
            let selector = format!("[data-toc-link][href='#{}']", id);
            if let Ok(Some(link)) = container_clone.query_selector(&selector) {
                if let Some(li) = link.parent_element() {
                    let _ = li.set_attribute("data-state", "active");

                    // Set ancestor state: mark last seen item at each parent level
                    if let Some(level_str) = li.get_attribute("data-level") {
                        if let Ok(current_level) = level_str.parse::<i32>() {
                            let mut last_at_level: [Option<Element>; 6] = Default::default();
                            
                            if let Ok(all_items) = container_clone.query_selector_all("[data-toc-item]") {
                                for j in 0..all_items.length() {
                                    if let Some(item) = all_items.item(j) {
                                        if let Ok(el) = item.dyn_into::<Element>() {
                                            if el.is_same_node(Some(&li)) {
                                                for parent_level in 1..current_level {
                                                    if let Some(ancestor) = &last_at_level[parent_level as usize] {
                                                        let _ = ancestor.set_attribute("data-state", "ancestor");
                                                    }
                                                }
                                                break;
                                            }
                                            
                                            if let Some(lvl_str) = el.get_attribute("data-level") {
                                                if let Ok(lvl) = lvl_str.parse::<i32>() {
                                                    if lvl > 0 && lvl < 6 {
                                                        last_at_level[lvl as usize] = Some(el.clone());
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                }
            }
        }
    }) as Box<dyn FnMut(js_sys::Array, web_sys::IntersectionObserver)>);

    let mut opts = web_sys::IntersectionObserverInit::new();
    opts.root_margin("0px 0px -60% 0px");
    let threshold = js_sys::Array::new();
    threshold.push(&JsValue::from_f64(0.0));
    opts.threshold(&threshold);

    let observer = web_sys::IntersectionObserver::new_with_options(
        callback.as_ref().unchecked_ref(),
        &opts,
    ).map_err(|_| canonrs_shared::BehaviorError::JsError { message: "observer failed".into() })?;

    callback.forget();

    for i in 0..headings.length() {
        if let Some(h) = headings.item(i) {
            if let Ok(el) = h.dyn_into::<web_sys::Element>() {
                observer.observe(&el);
            }
        }
    }

    Ok(())
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
