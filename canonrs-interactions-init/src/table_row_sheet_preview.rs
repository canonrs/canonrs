//! TableRowSheetPreview Init
//! Intercepta click em rows com data-rs-action="open-sheet"
//! Injeta dados no Sheet global e abre

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::{state, query};

pub fn init(root: Element) {
    if root.get_attribute("data-rs-table-context-init").as_deref() == Some("true") { return; }
    let _ = root.set_attribute("data-rs-table-context-init", "true");

    // click — abre sheet
    let root_click = root.clone();
    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
        let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        if query::has_ancestor_attr(&target, "data-rs-copyable") { return; }
        let Some(row) = query::closest_action(&target, "open-sheet") else { return };

        let label = row.get_attribute("data-rs-label").unwrap_or_default();
        let meta  = row.get_attribute("data-rs-meta").unwrap_or_default();

        let Some(ctx) = query::closest_attr(&row, "data-rs-table-context") else { return };
        let parent = ctx.parent_element();
        let search_root = parent.as_ref().unwrap_or(&ctx);
        let Some(sheet) = query::first(search_root, "[data-rs-sheet]") else { return };

        if let Some(title) = query::first(&sheet, "[data-rs-sheet-title]") {
            let _ = title.set_text_content(Some(&label));
        }
        if let Some(desc) = query::first(&sheet, "[data-rs-sheet-description]") {
            let _ = desc.set_text_content(Some(&meta));
        }

        let current = sheet.get_attribute("data-rs-state").unwrap_or_default();
        let next: String = current.split_whitespace()
            .filter(|s| *s != "closed")
            .chain(std::iter::once("open"))
            .collect::<Vec<_>>()
            .join(" ");
        let _ = sheet.set_attribute("data-rs-state", &next);

        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            if let Some(body) = doc.body() {
                let _ = body.set_attribute("data-rs-scroll-lock", "true");
            }
        }

        // foca o botão close do sheet após transitionend (só quando open)
        let sheet_focus = sheet.clone();
        let focused = std::rc::Rc::new(std::cell::Cell::new(false));
        let focused2 = focused.clone();
        let cb = Closure::<dyn Fn(web_sys::Event)>::new(move |_: web_sys::Event| {
            if focused2.get() { return; }
            let state = sheet_focus.get_attribute("data-rs-state").unwrap_or_default();
            if !state.contains("open") { return; }
            focused2.set(true);
            if let Some(close_btn) = query::first(&sheet_focus, "[data-rs-sheet-close]") {
                if let Ok(el) = close_btn.dyn_into::<web_sys::HtmlElement>() {
                    let _ = el.focus();
                }
            }
        });
        let _ = sheet.add_event_listener_with_callback("transitionend", cb.as_ref().unchecked_ref());
        cb.forget();

        let rows = query::all(&ctx, "[data-rs-action='open-sheet']");
        for r in &rows {
            state::remove_state(r, "selected");
            let _ = r.remove_attribute("data-rs-row-selected");
        }
        state::add_state(&row, "selected");
        let _ = row.set_attribute("data-rs-row-selected", "true");
    });
    let _ = root_click.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
    cb.forget();

    // close button — fecha o sheet
    {
        let root_cls = root.clone();
        let parent = root_cls.parent_element();
        let search_root = parent.as_ref().unwrap_or(&root_cls);
        if let Some(sheet) = query::first(search_root, "[data-rs-sheet]") {
            if let Some(close_btn) = query::first(&sheet, "[data-rs-sheet-close]") {
                let sheet_cb = sheet.clone();
                let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
                    state::remove_state(&sheet_cb, "open");
                    state::add_state(&sheet_cb, "closed");
                    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                        if let Some(body) = doc.body() {
                            let _ = body.remove_attribute("data-rs-scroll-lock");
                        }
                    }
                });
                let _ = close_btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
                cb.forget();
            }
        }
    }

    // keydown — Enter/Escape fecha o sheet quando está aberto
    {
        let root_key = root.clone();
        let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
            match e.key().as_str() {
                "Escape" => {}
                "Enter" => {
                    // só fecha se o foco está dentro do sheet
                    let active = web_sys::window()
                        .and_then(|w| w.document())
                        .and_then(|d| d.active_element());
                    let focus_in_sheet = active
                        .as_ref()
                        .and_then(|el| el.closest("[data-rs-sheet]").ok().flatten())
                        .is_some();
                    if !focus_in_sheet { return; }
                }
                _ => return,
            }
            let parent = root_key.parent_element();
            let search_root = parent.as_ref().unwrap_or(&root_key);
            let Some(sheet) = query::first(search_root, "[data-rs-sheet]") else { return };
            let state = sheet.get_attribute("data-rs-state").unwrap_or_default();
            if !state.contains("open") { return; }
            e.prevent_default();
            state::remove_state(&sheet, "open");
            state::add_state(&sheet, "closed");
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                if let Some(body) = doc.body() {
                    let _ = body.remove_attribute("data-rs-scroll-lock");
                }
            }
            // devolve foco para row selected
            let rows = query::all(&root_key, "[data-rs-action='open-sheet']");
            for row in &rows {
                if row.get_attribute("data-rs-row-selected").as_deref() == Some("true") {
                    let row_clone = row.clone();
                    let sheet_t = sheet.clone();

                    let focused = std::rc::Rc::new(std::cell::Cell::new(false));
                    let focused2 = focused.clone();
                    let cb = Closure::<dyn Fn(web_sys::Event)>::new(move |_: web_sys::Event| {
                        if focused2.get() { return; }
                        focused2.set(true);
                        if let Ok(el) = row_clone.clone().dyn_into::<web_sys::HtmlElement>() {
                            let _ = el.focus();
                        }
                    });
                    let _ = sheet_t.add_event_listener_with_callback(
                        "transitionend", cb.as_ref().unchecked_ref(),
                    );
                    cb.forget();
                    break;
                }
            }
        });
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let _ = doc.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        }
        cb.forget();
    }

    // MutationObserver — quando sheet fecha, devolve foco para row selecionada
    let root_obs = root.clone();
    let on_mutation = Closure::<dyn Fn(js_sys::Array)>::new(move |_: js_sys::Array| {
        let parent = root_obs.parent_element();
        let search_root = parent.as_ref().unwrap_or(&root_obs);
        let Some(sheet) = query::first(search_root, "[data-rs-sheet]") else { return };
        let state = sheet.get_attribute("data-rs-state").unwrap_or_default();
        if !state.contains("closed") { return; }
        // devolve foco para a row selected
        let rows = query::all(&root_obs, "[data-rs-action='open-sheet']");
        for row in &rows {
            if row.get_attribute("data-rs-state").unwrap_or_default().contains("selected") {
                let row_clone = row.clone();
                let cb = Closure::once(move || {
                    if let Ok(el) = row_clone.dyn_into::<web_sys::HtmlElement>() {
                        let _ = el.focus();
                    }
                });
                if let Some(win) = web_sys::window() {
                    let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
                        cb.as_ref().unchecked_ref(), 0,
                    );
                }
                cb.forget();
                break;
            }
        }
    });

    let observer = web_sys::MutationObserver::new(on_mutation.as_ref().unchecked_ref())
        .expect("MutationObserver failed");
    on_mutation.forget();

    let parent = root.parent_element();
    let search_root = parent.as_ref().unwrap_or(&root);
    if let Some(sheet) = query::first(search_root, "[data-rs-sheet]") {
        let opts = web_sys::MutationObserverInit::new();
        opts.set_attributes(true);
        opts.set_attribute_filter(&js_sys::Array::of1(&"data-rs-state".into()));
        let _ = observer.observe_with_options(&sheet, &opts);
    }
    std::mem::forget(observer);
}
