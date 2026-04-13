//! DropdownMenu Interaction Engine

use web_sys::Element;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::runtime::{lifecycle, state as rs};

fn add_tok(el: &Element, token: &str) {
    let cur = el.get_attribute("data-rs-state").unwrap_or_default();
    if cur.split_whitespace().any(|t| t == token) { return; }
    let _ = el.set_attribute("data-rs-state", &format!("{} {}", cur, token).trim().to_string());
}

fn rem_tok(el: &Element, token: &str) {
    let cur = el.get_attribute("data-rs-state").unwrap_or_default();
    let _ = el.set_attribute("data-rs-state", &cur.split_whitespace().filter(|t| *t != token).collect::<Vec<_>>().join(" "));
}

fn set_open(root: &Element, open: bool) {
    if open { rem_tok(root, "closed"); add_tok(root, "open"); } else { rem_tok(root, "open"); add_tok(root, "closed"); }
}
fn get_items(root: &Element) -> Vec<Element> {
    root.query_selector_all("[data-rs-dropdown-menu-item]").ok()
        .map(|l| (0..l.length()).filter_map(|i| l.item(i)).filter_map(|n| n.dyn_into::<Element>().ok()).collect())
        .unwrap_or_default()
}
fn navigable(root: &Element) -> Vec<Element> {
    get_items(root).into_iter().filter(|el| !el.get_attribute("data-rs-state").map(|s| s.contains("disabled")).unwrap_or(false)).collect()
}
fn focused_idx(items: &[Element]) -> Option<usize> {
    items.iter().position(|el| el.get_attribute("data-rs-state").map(|s| s.contains("focus")).unwrap_or(false))
}
fn clear_focus(root: &Element) { for el in get_items(root) { rem_tok(&el, "focus"); } }
fn focus_el(el: &Element) { add_tok(el, "focus"); }

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    {
        let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(cur) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let trig = t.closest("[data-rs-dropdown-menu-trigger]").ok().flatten();
            if trig.is_some() {
                e.stop_propagation();
                let open = rs::is_open(&cur);
                set_open(&cur, !open);
                if !open { clear_focus(&cur); }
                return;
            }
            if let Ok(Some(item)) = t.closest("[data-rs-dropdown-menu-item]") {
                if item.get_attribute("data-rs-state").map(|s| s.contains("disabled")).unwrap_or(false) { return; }
                e.stop_propagation();
                clear_focus(&cur);
                set_open(&cur, false);
            }
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    {
        let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(cur) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if let Ok(Some(item)) = t.closest("[data-rs-dropdown-menu-item]") {
                if item.get_attribute("data-rs-state").map(|s| s.contains("disabled")).unwrap_or(false) { return; }
                clear_focus(&cur);
                focus_el(&item);
            }
        }));
        let _ = root.add_event_listener_with_callback("mouseover", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    {
        let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            let Some(cur) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if !rs::is_open(&cur) { return; }
            match e.key().as_str() {
                "Escape" | "Tab" => { rs::close(&cur); clear_focus(&cur); }
                "Enter" | " " => { e.prevent_default(); if focused_idx(&navigable(&cur)).is_some() { rs::close(&cur); clear_focus(&cur); } }
                "ArrowDown" => { e.prevent_default(); let items = navigable(&cur); let len = items.len(); if len == 0 { return; } let next = match focused_idx(&items) { None => 0, Some(i) => (i+1).min(len-1) }; clear_focus(&cur); if let Some(el) = items.get(next) { focus_el(el); } }
                "ArrowUp"   => { e.prevent_default(); let items = navigable(&cur); let len = items.len(); if len == 0 { return; } let next = match focused_idx(&items) { None => len-1, Some(i) => if i==0 {0} else {i-1} }; clear_focus(&cur); if let Some(el) = items.get(next) { focus_el(el); } }
                _ => {}
            }
        }));
        let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // outside click via document — resolve dinamicamente
    {
        let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if t.closest("[data-rs-dropdown-menu-trigger]").ok().flatten().is_some() { return; }
            let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
            let Ok(nodes) = doc.query_selector_all("[data-rs-dropdown-menu][data-rs-initialized=\'true\']") else { return };
            for i in 0..nodes.length() {
                let Some(raw) = nodes.item(i) else { continue };
                let Ok(node) = raw.dyn_into::<Element>() else { continue };
                if !node.contains(Some(&t)) && rs::is_open(&node) { rs::close(&node); clear_focus(&node); }
            }
        }));
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let _ = doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        }
        cb.forget();
    }
}
