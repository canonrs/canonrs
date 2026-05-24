//! ScrollSpy — centralized scroll-based active heading detection
//!
//! Responsibilities:
//! - Viewport topology resolution (finds scroll container)
//! - Heading element caching (no DOM queries in scroll loop)
//! - Active heading detection
//! - Lifecycle management via uid namespace

use web_sys::Element;

/// Resolve scroll viewport for a given root element
/// Priority: [data-rs-md-layout] > [data-rs-scroll-viewport] > window
pub fn resolve_viewport(root: &Element) -> Option<Element> {
    // 1. Layout-aware: find scroll viewport in md-layout ancestor
    root.closest("[data-rs-md-layout]").ok().flatten()
        .and_then(|layout| layout.query_selector("[data-rs-markdown-content]").ok().flatten())
        .and_then(|content_el| find_scroll_ancestor(content_el))
        .or_else(|| {
            // 2. Fallback: find scroll viewport containing markdown content
            let doc = web_sys::window()?.document()?;
            let content_el = doc.query_selector("[data-rs-markdown-content]").ok()??;
            find_scroll_ancestor(content_el)
        })
}

fn find_scroll_ancestor(el: Element) -> Option<Element> {
    let mut current: Option<Element> = Some(el);
    while let Some(parent) = current {
        if parent.has_attribute("data-rs-scroll-viewport") {
            return Some(parent);
        }
        current = parent.parent_element();
    }
    None
}

/// Cache heading elements by id — call once at init, not in scroll loop
pub fn cache_headings(ids: &[String]) -> Vec<(String, Element)> {
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return vec![] };
    ids.iter().filter_map(|id| {
        doc.query_selector(&format!("#{}", id)).ok().flatten().map(|el| (id.clone(), el))
    }).collect()
}

/// Find active heading from cached elements — O(n) with no DOM queries
pub fn active_heading(
    cached: &[(String, Element)],
    viewport: Option<&Element>,
    threshold_offset: f64,
) -> Option<String> {
    let vp_top = viewport.map(|v| v.get_bounding_client_rect().top()).unwrap_or(0.0);
    let threshold = vp_top + threshold_offset;
    let mut closest: Option<(f64, String)> = None;
    for (id, el) in cached {
        let top = el.get_bounding_client_rect().top();
        if top <= threshold {
            match &closest {
                None => { closest = Some((top, id.clone())); }
                Some((ct, _)) => { if top > *ct { closest = Some((top, id.clone())); } }
            }
        }
    }
    closest.map(|(_, id)| id).or_else(|| cached.first().map(|(id, _)| id.clone()))
}

/// Navigate to anchor — scroll + history update
/// Note: history.replace_state belongs to navigation runtime, exposed here as utility
pub fn scroll_to_anchor(id: &str, offset: f64) {
    let Some(win) = web_sys::window() else { return };
    let Some(doc) = win.document() else { return };
    if let Ok(Some(target)) = doc.query_selector(&format!("#{}", id)) {
        let rect = target.get_bounding_client_rect();
        let scroll_y = win.scroll_y().unwrap_or(0.0);
        let top = rect.top() + scroll_y - offset;
        let opts = web_sys::ScrollToOptions::new();
        opts.set_top(top);
        opts.set_behavior(web_sys::ScrollBehavior::Smooth);
        win.scroll_to_with_scroll_to_options(&opts);
        // history update — owned by nav runtime, delegated here for now
        if let Ok(history) = win.history() {
            history.replace_state_with_url(
                &wasm_bindgen::JsValue::NULL, "", Some(&format!("#{}", id))
            ).ok();
        }
    }
}
