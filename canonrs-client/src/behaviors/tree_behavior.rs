#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::{Element, HtmlElement};

#[cfg(feature = "hydrate")]
fn get_visible_items(root: &Element) -> Vec<Element> {
    let mut visible = Vec::new();
    if let Ok(all) = root.query_selector_all("[data-rs-tree-item]") {
        for i in 0..all.length() {
            if let Some(node) = all.item(i) {
                if let Ok(el) = node.dyn_into::<Element>() {
                    if is_visible(&el) { visible.push(el); }
                }
            }
        }
    }
    visible
}

#[cfg(feature = "hydrate")]
fn is_visible(item: &Element) -> bool {
    let mut current = item.parent_element();
    while let Some(parent) = current {
        if parent.has_attribute("data-rs-tree-group") {
            if let Some(prev) = parent.previous_element_sibling() {
                if prev.get_attribute("data-rs-expanded").as_deref() == Some("false") {
                    return false;
                }
            }
        }
        current = parent.parent_element();
    }
    true
}

#[cfg(feature = "hydrate")]
fn get_focused(root: &Element) -> Option<Element> {
    root.query_selector("[data-rs-tree-item][tabindex='0']").ok().flatten()
}

#[cfg(feature = "hydrate")]
fn set_focus(root: &Element, target: &Element) {
    if let Ok(all) = root.query_selector_all("[data-rs-tree-item]") {
        for i in 0..all.length() {
            if let Some(node) = all.item(i) {
                if let Ok(el) = node.dyn_into::<Element>() {
                    el.set_attribute("tabindex", "-1").ok();
                }
            }
        }
    }
    target.set_attribute("tabindex", "0").ok();
    if let Ok(el) = target.clone().dyn_into::<HtmlElement>() { el.focus().ok(); }
}

#[cfg(feature = "hydrate")]
fn select_item(root: &Element, item: &Element) {
    if let Ok(all) = root.query_selector_all("[data-rs-tree-item]") {
        for i in 0..all.length() {
            if let Some(node) = all.item(i) {
                if let Ok(el) = node.dyn_into::<Element>() {
                    el.set_attribute("data-rs-state", "unselected").ok();
                    el.set_attribute("aria-selected", "false").ok();
                }
            }
        }
    }
    item.set_attribute("data-rs-state", "selected").ok();
    item.set_attribute("aria-selected", "true").ok();
}

#[cfg(feature = "hydrate")]
fn toggle_expand(item: &Element) {
    let is_expanded = item.get_attribute("data-rs-expanded").as_deref() == Some("true");
    let new = if is_expanded { "false" } else { "true" };
    item.set_attribute("data-rs-expanded", new).ok();
    item.set_attribute("aria-expanded", new).ok();
}

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-tree", Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {

        if root.get_attribute("data-rs-tree-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-tree-attached", "1").ok();

        // click em items
        let items = root.query_selector_all("[data-rs-tree-item]")
            .map_err(|_| crate::BehaviorError::JsError { message: "query items".into() })?;

        for i in 0..items.length() {
            let Some(node) = items.item(i) else { continue };
            let Ok(item) = node.dyn_into::<Element>() else { continue };

            if item.get_attribute("data-rs-tree-item-initialized").is_some() { continue; }
            item.set_attribute("data-rs-tree-item-initialized", "1").ok();

            let root_click = root.clone();
            let item_click = item.clone();
            let cb = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
                e.stop_propagation();
                select_item(&root_click, &item_click);
                set_focus(&root_click, &item_click);
                if item_click.get_attribute("data-rs-expanded").is_some() {
                    toggle_expand(&item_click);
                }
            }) as Box<dyn FnMut(_)>);
            item.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        // keyboard navigation
        let root_key = root.clone();
        let cb_key = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            let visible = get_visible_items(&root_key);
            if visible.is_empty() { return; }
            let len = visible.len();

            match e.key().as_str() {
                "ArrowDown" | "ArrowUp" | "Home" | "End" => {
                    e.prevent_default();
                    let cur = get_focused(&root_key)
                        .and_then(|f| visible.iter().position(|el| el == &f));
                    let next = match e.key().as_str() {
                        "ArrowDown" => cur.map(|i| (i + 1).min(len - 1)).unwrap_or(0),
                        "ArrowUp"   => cur.map(|i| i.saturating_sub(1)).unwrap_or(0),
                        "Home"      => 0,
                        "End"       => len - 1,
                        _           => return,
                    };
                    set_focus(&root_key, &visible[next]);
                }
                "ArrowRight" => {
                    e.prevent_default();
                    if let Some(item) = get_focused(&root_key) {
                        if item.get_attribute("data-rs-expanded").as_deref() == Some("false") {
                            toggle_expand(&item);
                        }
                    }
                }
                "ArrowLeft" => {
                    e.prevent_default();
                    if let Some(item) = get_focused(&root_key) {
                        if item.get_attribute("data-rs-expanded").as_deref() == Some("true") {
                            toggle_expand(&item);
                        }
                    }
                }
                "Enter" | " " => {
                    e.prevent_default();
                    if let Some(item) = get_focused(&root_key) {
                        select_item(&root_key, &item);
                        if item.get_attribute("data-rs-expanded").is_some() {
                            toggle_expand(&item);
                        }
                    }
                }
                _ => {}
            }
        }) as Box<dyn FnMut(_)>);
        root.add_event_listener_with_callback("keydown", cb_key.as_ref().unchecked_ref()).ok();
        cb_key.forget();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
