//! TableRowSheetPreview Init

use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::{state, query};
use canonrs_interactions_core::runtime::{listeners, observer, timers};

fn closest_action(el: &Element, action: &str) -> Option<Element> {
    el.closest(&format!("[data-rs-action='{}']", action)).ok().flatten()
}

pub fn init(root: Element) {
    if root.get_attribute("data-rs-table-context-init").as_deref() == Some("true") { return; }
    let _ = root.set_attribute("data-rs-table-context-init", "true");

    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    let uid2 = uid.clone();
    let uid3 = uid.clone();

    // click — open sheet
    listeners::listen(&uid, &root, "click", {
        let root_c = root.clone();
        move |e: web_sys::Event| {
            let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if query::has_ancestor_attr(&target, "data-rs-copyable") { return; }
            let Some(row) = closest_action(&target, "open-sheet") else { return };
            let label = row.get_attribute("data-rs-label").unwrap_or_default();
            let meta  = row.get_attribute("data-rs-meta").unwrap_or_default();
            let Some(ctx) = query::closest_attr(&row, "data-rs-table-context") else { return };
            let parent = ctx.parent_element();
            let search_root = parent.as_ref().unwrap_or(&ctx);
            let Some(sheet) = query::first(search_root, "[data-rs-sheet]") else { return };
            if let Some(title) = query::first(&sheet, "[data-rs-sheet-title]") { let _ = title.set_text_content(Some(&label)); }
            if let Some(desc)  = query::first(&sheet, "[data-rs-sheet-description]") { let _ = desc.set_text_content(Some(&meta)); }
            let current = sheet.get_attribute("data-rs-state").unwrap_or_default();
            let next: String = current.split_whitespace()
                .filter(|s| *s != "closed")
                .chain(std::iter::once("open"))
                .collect::<Vec<_>>().join(" ");
            let _ = sheet.set_attribute("data-rs-state", &next);
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                if let Some(body) = doc.body() { let _ = body.set_attribute("data-rs-scroll-lock", "true"); }
            }
            // focus close button after transition
            let sheet_focus = sheet.clone();
            let focused = std::rc::Rc::new(std::cell::Cell::new(false));
            let focused2 = focused.clone();
            let uid_trans = format!("{}:trans", root_c.get_attribute("data-rs-uid").unwrap_or_default());
            listeners::listen(&uid_trans, &sheet, "transitionend", move |_: web_sys::Event| {
                if focused2.get() { return; }
                let s = sheet_focus.get_attribute("data-rs-state").unwrap_or_default();
                if !s.contains("open") { return; }
                focused2.set(true);
                if let Some(btn) = query::first(&sheet_focus, "[data-rs-sheet-close]") {
                    if let Ok(el) = btn.dyn_into::<web_sys::HtmlElement>() { let _ = el.focus(); }
                }
            });
            let rows = query::all(&ctx, "[data-rs-action='open-sheet']");
            for r in &rows {
                state::remove(r, canonrs_interactions_core::dom::state::State::Selected.as_str());
                let _ = r.remove_attribute("data-rs-row-selected");
            }
            state::add(&row, canonrs_interactions_core::dom::state::State::Selected.as_str());
            let _ = row.set_attribute("data-rs-row-selected", "true");
        }
    });

    // close button
    {
        let parent = root.parent_element();
        let search_root = parent.as_ref().unwrap_or(&root);
        if let Some(sheet) = query::first(search_root, "[data-rs-sheet]") {
            if let Some(close_btn) = query::first(&sheet, "[data-rs-sheet-close]") {
                let uid_close = format!("{}:close", uid);
                listeners::listen(&uid_close, &close_btn, "click", {
                    let sheet_c = sheet.clone();
                    move |_: web_sys::Event| {
                        state::remove(&sheet_c, canonrs_interactions_core::dom::state::State::Open.as_str());
                        state::add(&sheet_c, canonrs_interactions_core::dom::state::State::Closed.as_str());
                        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                            if let Some(body) = doc.body() { let _ = body.remove_attribute("data-rs-scroll-lock"); }
                        }
                    }
                });
            }
        }
    }

    // keydown — Escape/Enter closes sheet
    listeners::listen_document(&uid.clone(), "keydown", {
        let root_c = root.clone();
        move |e: web_sys::Event| {
            let e = e.dyn_into::<web_sys::KeyboardEvent>().unwrap();
            match e.key().as_str() {
                "Escape" => {}
                "Enter" => {
                    let in_sheet = web_sys::window()
                        .and_then(|w| w.document())
                        .and_then(|d| d.active_element())
                        .and_then(|el| el.closest("[data-rs-sheet]").ok().flatten())
                        .is_some();
                    if !in_sheet { return; }
                }
                _ => return,
            }
            let parent = root_c.parent_element();
            let search_root = parent.as_ref().unwrap_or(&root_c);
            let Some(sheet) = query::first(search_root, "[data-rs-sheet]") else { return };
            if !sheet.get_attribute("data-rs-state").unwrap_or_default().contains("open") { return; }
            e.prevent_default();
            state::remove(&sheet, canonrs_interactions_core::dom::state::State::Open.as_str());
            state::add(&sheet, canonrs_interactions_core::dom::state::State::Closed.as_str());
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                if let Some(body) = doc.body() { let _ = body.remove_attribute("data-rs-scroll-lock"); }
            }
            let rows = query::all(&root_c, "[data-rs-action='open-sheet']");
            for row in &rows {
                if row.get_attribute("data-rs-row-selected").as_deref() == Some("true") {
                    let row_c = row.clone();
                    let sheet_t = sheet.clone();
                    let uid_focus = format!("{}:focus-restore", uid2);
                    let focused = std::rc::Rc::new(std::cell::Cell::new(false));
                    let focused2 = focused.clone();
                    listeners::listen(&uid_focus, &sheet_t, "transitionend", move |_: web_sys::Event| {
                        if focused2.get() { return; }
                        focused2.set(true);
                        if let Ok(el) = row_c.clone().dyn_into::<web_sys::HtmlElement>() { let _ = el.focus(); }
                    });
                    break;
                }
            }
        }
    });

    // MutationObserver — when sheet closes, return focus to selected row
    {
        let parent = root.parent_element();
        let search_root = parent.as_ref().unwrap_or(&root);
        if let Some(sheet) = query::first(search_root, "[data-rs-sheet]") {
            let uid_obs = format!("{}:obs", uid3);
            let root_obs = root.clone();
            let opts = web_sys::MutationObserverInit::new();
            opts.set_attributes(true);
            opts.set_attribute_filter(&js_sys::Array::of1(&"data-rs-state".into()));
            observer::mutation_opts(&uid_obs, &sheet, &opts, move |_: js_sys::Array| {
                let parent = root_obs.parent_element();
                let search = parent.as_ref().unwrap_or(&root_obs);
                let Some(sheet) = query::first(search, "[data-rs-sheet]") else { return };
                if !sheet.get_attribute("data-rs-state").unwrap_or_default().contains("closed") { return; }
                let rows = query::all(&root_obs, "[data-rs-action='open-sheet']");
                for row in &rows {
                    if row.get_attribute("data-rs-state").unwrap_or_default().contains("selected") {
                        let row_c = row.clone();
                        timers::timeout(0, move || {
                            if let Ok(el) = row_c.dyn_into::<web_sys::HtmlElement>() { let _ = el.focus(); }
                        });
                        break;
                    }
                }
            });
        }
    }
}
