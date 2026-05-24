//! Radio Interaction Engine — keyboard navigation + selection sync

use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::state;
use canonrs_interactions_core::runtime::listeners;
use crate::runtime::context;

fn get_items(root: &Element) -> Vec<Element> {
    let Ok(nodes) = root.query_selector_all("[data-rs-radio]") else { return vec![] };
    (0..nodes.length())
        .filter_map(|i| nodes.item(i))
        .filter_map(|n| n.dyn_into::<Element>().ok())
        .collect()
}

fn navigable_items(root: &Element) -> Vec<Element> {
    get_items(root).into_iter()
        .filter(|el| !state::has(el, canonrs_interactions_core::dom::state::State::Disabled.as_str()))
        .collect()
}

fn item_value(item: &Element) -> String {
    item.query_selector("[data-rs-radio-input]").ok().flatten()
        .and_then(|n| n.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|el| el.value())
        .unwrap_or_default()
}

fn set_input_checked(item: &Element, checked: bool) {
    if let Ok(Some(node)) = item.query_selector("[data-rs-radio-input]") {
        if let Ok(input) = node.dyn_into::<web_sys::HtmlInputElement>() {
            input.set_checked(checked);
        }
    }
}

fn set_tabindex(item: &Element, idx: &str) {
    if let Ok(el) = item.clone().dyn_into::<web_sys::HtmlElement>() {
        let _ = el.set_attribute("tabindex", idx);
    }
}

fn focus_item(item: &Element) {
    if let Ok(el) = item.clone().dyn_into::<web_sys::HtmlElement>() {
        let _ = el.focus();
    }
}

fn select_item(root: &Element, value: &str) {
    for item in get_items(root) {
        let matches = item_value(&item) == value;
        state::remove(&item, canonrs_interactions_core::dom::state::State::Selected.as_str());
        state::remove(&item, canonrs_interactions_core::dom::state::State::Unselected.as_str());
        if matches {
            state::add(&item, canonrs_interactions_core::dom::state::State::Selected.as_str());
            let _ = item.set_attribute("aria-checked", "true");
            set_input_checked(&item, true);
            set_tabindex(&item, "0");
        } else {
            state::add(&item, canonrs_interactions_core::dom::state::State::Unselected.as_str());
            let _ = item.set_attribute("aria-checked", "false");
            set_input_checked(&item, false);
            set_tabindex(&item, "-1");
        }
    }
    let _ = root.set_attribute("data-rs-value", value);
}

pub fn init(root: Element) {
    context::propagate_owner(&root);

    // SSR bootstrap — roving tabindex + garantir consistência
    {
        let items = get_items(&root);
        let has_selected = items.iter().any(|el| state::has(el, canonrs_interactions_core::dom::state::State::Selected.as_str()));
        for (i, item) in items.iter().enumerate() {
            let selected = state::has(item, canonrs_interactions_core::dom::state::State::Selected.as_str());
            if selected {
                set_tabindex(item, "0");
            } else if !has_selected && i == 0 {
                // sem seleção SSR → primeiro item focável
                set_tabindex(item, "0");
            } else {
                set_tabindex(item, "-1");
            }
        }
    }



    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

    listeners::listen(&uid, &root, "focusin", {
        let root_c = root.clone();
        move |e: web_sys::Event| {
            let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(item) = t.closest("[data-rs-radio]").ok().flatten() else { return };
            for el in get_items(&root_c) { state::remove(&el, "focus"); }
            state::add(&item, "focus");
        }
    });

    listeners::listen(&uid, &root, "focusout", {
        let root_c = root.clone();
        move |_: web_sys::Event| {
            for el in get_items(&root_c) { state::remove(&el, "focus"); }
        }
    });

    listeners::listen(&uid, &root, "click", move |e: web_sys::Event| {
        let e = e.dyn_into::<web_sys::MouseEvent>().unwrap();
        let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        let Some(rc) = context::find_root(&t, "[data-rs-radio-group]") else { return };
        let Some(item) = t.closest("[data-rs-radio]").ok().flatten() else { return };
        if state::has(&item, canonrs_interactions_core::dom::state::State::Disabled.as_str()) { return; }
        select_item(&rc, &item_value(&item));
    });

    listeners::listen(&uid, &root, "keydown", move |e: web_sys::Event| {
        let e = e.dyn_into::<web_sys::KeyboardEvent>().unwrap();
        let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        let Some(rc) = context::find_root(&t, "[data-rs-radio-group]") else { return };
        if t.closest("[data-rs-radio]").ok().flatten().is_none() { return; }
        let items = navigable_items(&rc);
        let len = items.len();
        if len == 0 { return; }
        let pos = items.iter().position(|el| el.contains(Some(&t)));
        let next_idx = match e.key().as_str() {
            "ArrowDown" | "ArrowRight" => { e.prevent_default(); pos.map(|p| (p + 1) % len) }
            "ArrowUp"   | "ArrowLeft"  => { e.prevent_default(); pos.map(|p| if p == 0 { len - 1 } else { p - 1 }) }
            _ => None,
        };
        if let Some(idx) = next_idx {
            if let Some(item) = items.get(idx) {
                select_item(&rc, &item_value(item));
                focus_item(item);
            }
        }
    });
}

pub fn init_all() {
    let win = match web_sys::window() { Some(w) => w, None => return };
    let doc = match win.document() { Some(d) => d, None => return };
    let nodes = match doc.query_selector_all("[data-rs-radio-group]") { Ok(n) => n, Err(_) => return };
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<Element>() { init(el); }
        }
    }
}
