//! @canon-level: strict
//! Accordion Island — apenas muta DOM via web_sys
//! SEM signals, SEM lógica de negócio, SEM estado reativo

use leptos::prelude::*;

use super::accordion_ui::{Accordion, AccordionItem, AccordionTrigger, AccordionContent};
use canonrs_core::meta::{VisibilityState, DisabledState};
use canonrs_core::primitives::AccordionSelection;

// ---------------------------------------------------------------------------
// Helpers de estado — multi-state safe
// ---------------------------------------------------------------------------

fn add_state(el: &leptos::web_sys::Element, token: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    if current.split_whitespace().any(|t| t == token) { return; }
    let next = format!("{} {}", current, token).trim().to_string();
    let _ = el.set_attribute("data-rs-state", &next);
}

fn remove_state(el: &leptos::web_sys::Element, token: &str) {
    let current = el.get_attribute("data-rs-state").unwrap_or_default();
    let next = current.split_whitespace()
        .filter(|t| *t != token)
        .collect::<Vec<_>>()
        .join(" ");
    let _ = el.set_attribute("data-rs-state", &next);
}

// ---------------------------------------------------------------------------
// Helpers DOM
// ---------------------------------------------------------------------------

fn get_items(root: &leptos::web_sys::Element) -> Vec<leptos::web_sys::Element> {
    let mut result = Vec::new();
    let mut i = 0u32;
    loop {
        let selector = format!("[data-rs-accordion-item]:nth-of-type({})", i + 1);
        match root.query_selector(&selector) {
            Ok(Some(el)) => { result.push(el); i += 1; }
            _ => break,
        }
    }
    result
}

fn get_trigger(item: &leptos::web_sys::Element) -> Option<leptos::web_sys::Element> {
    item.query_selector("[data-rs-accordion-trigger]").ok().flatten()
}

#[allow(dead_code)]
fn get_content(item: &leptos::web_sys::Element) -> Option<leptos::web_sys::Element> {
    item.query_selector("[data-rs-accordion-content]").ok().flatten()
}

fn is_open(item: &leptos::web_sys::Element) -> bool {
    item.get_attribute("data-rs-state")
        .map(|s: String| s.contains("open"))
        .unwrap_or(false)
}

fn is_item_disabled(item: &leptos::web_sys::Element) -> bool {
    item.get_attribute("data-rs-state")
        .map(|s: String| s.contains("disabled"))
        .unwrap_or(false)
}

fn open_item(item: &leptos::web_sys::Element) {
    remove_state(item, "closed");
    add_state(item, "open");
    if let Some(trigger) = get_trigger(item) {
        let _ = trigger.set_attribute("aria-expanded", "true");
    }
}

fn close_item(item: &leptos::web_sys::Element) {
    remove_state(item, "open");
    add_state(item, "closed");
    if let Some(trigger) = get_trigger(item) {
        let _ = trigger.set_attribute("aria-expanded", "false");
    }
}

fn toggle_item(root: &leptos::web_sys::Element, item: &leptos::web_sys::Element) {
    let collapsible = root.get_attribute("data-rs-collapsible")
        .map(|s: String| s == "true")
        .unwrap_or(true);
    let is_single = root.get_attribute("data-rs-selection")
        .map(|s: String| s == "single")
        .unwrap_or(true);

    let currently_open = is_open(item);

    if currently_open {
        if collapsible {
            close_item(item);
        }
    } else {
        if is_single {
            for other in get_items(root) {
                if is_open(&other) {
                    close_item(&other);
                }
            }
        }
        open_item(item);
    }
}

// ---------------------------------------------------------------------------
// Struct de item
// ---------------------------------------------------------------------------

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AccordionIslandItem {
    pub value:    String,
    pub trigger:  String,
    pub content:  String,
    pub disabled: bool,
}

#[derive(Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum AccordionSelectionMode {
    #[default]
    Single,
    Multiple,
}

// ---------------------------------------------------------------------------
// Island
// ---------------------------------------------------------------------------

static ACCORDION_ID: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);

#[island]
pub fn AccordionIsland(
    items: Vec<AccordionIslandItem>,
    #[prop(optional)] selection: Option<AccordionSelectionMode>,
    #[prop(optional)] collapsible: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let selection   = selection.unwrap_or_default();
    let collapsible = collapsible.unwrap_or(true);
    let class       = class.unwrap_or_default();
    let island_id   = ACCORDION_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed).to_string();

    let acc_selection = match selection {
        AccordionSelectionMode::Single   => AccordionSelection::Single,
        AccordionSelectionMode::Multiple => AccordionSelection::Multiple,
    };

    let node_ref = NodeRef::<leptos::html::Div>::new();

    let id_for_effect = island_id.clone();
    Effect::new(move |_| {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;

        let Some(root_html) = node_ref.get() else { return };
        let root: web_sys::Element = (*root_html).clone().unchecked_into();
        let _ = root.set_attribute("data-rs-accordion-id", &id_for_effect);

        // --- click trigger ---
        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
                let Some(target) = e.target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
                if target.closest("[data-rs-accordion-trigger]").ok().flatten().is_none() { return; }
                if let Ok(Some(item)) = target.closest("[data-rs-accordion-item]") {
                    if is_item_disabled(&item) { return; }
                    toggle_item(&root_cb, &item);
                }
            }));
            let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
            cb.forget();
        }

        // --- keydown ---
        {
            let root_cb = root.clone();
            let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                let Some(target) = e.target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
                if target.closest("[data-rs-accordion-trigger]").ok().flatten().is_none() { return; }

                let key = e.key();
                match key.as_str() {
                    "Enter" | " " => {
                        e.prevent_default();
                        if let Ok(Some(item)) = target.closest("[data-rs-accordion-item]") {
                            if !is_item_disabled(&item) {
                                toggle_item(&root_cb, &item);
                            }
                        }
                    }
                    "ArrowDown" => {
                        e.prevent_default();
                        let items = get_items(&root_cb);
                        if let Some(pos) = items.iter().position(|el| el.contains(Some(&target))) {
                            let next = (pos + 1).min(items.len() - 1);
                            if let Some(item) = items.get(next) {
                                if let Some(trigger) = get_trigger(item) {
                                    if let Ok(btn) = trigger.dyn_into::<web_sys::HtmlElement>() {
                                        let _ = btn.focus();
                                    }
                                }
                            }
                        }
                    }
                    "ArrowUp" => {
                        e.prevent_default();
                        let items = get_items(&root_cb);
                        if let Some(pos) = items.iter().position(|el| el.contains(Some(&target))) {
                            let prev = if pos == 0 { 0 } else { pos - 1 };
                            if let Some(item) = items.get(prev) {
                                if let Some(trigger) = get_trigger(item) {
                                    if let Ok(btn) = trigger.dyn_into::<web_sys::HtmlElement>() {
                                        let _ = btn.focus();
                                    }
                                }
                            }
                        }
                    }
                    "Home" => {
                        e.prevent_default();
                        let items = get_items(&root_cb);
                        if let Some(item) = items.first() {
                            if let Some(trigger) = get_trigger(item) {
                                if let Ok(btn) = trigger.dyn_into::<web_sys::HtmlElement>() {
                                    let _ = btn.focus();
                                }
                            }
                        }
                    }
                    "End" => {
                        e.prevent_default();
                        let items = get_items(&root_cb);
                        if let Some(item) = items.last() {
                            if let Some(trigger) = get_trigger(item) {
                                if let Ok(btn) = trigger.dyn_into::<web_sys::HtmlElement>() {
                                    let _ = btn.focus();
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }));
            let _ = root.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
            cb.forget();
        }
    });

    // SSR — estrutura pura, todos fechados
    let items_view = items.into_iter().map(|item| {
        let disabled = if item.disabled {
            DisabledState::Disabled
        } else {
            DisabledState::Enabled
        };
        view! {
            <AccordionItem state=VisibilityState::Closed disabled=disabled>
                <AccordionTrigger disabled=disabled>
                    {item.trigger}
                </AccordionTrigger>
                <AccordionContent>
                    {item.content}
                </AccordionContent>
            </AccordionItem>
        }
    }).collect::<Vec<_>>();

    view! {
        <Accordion selection=acc_selection collapsible=collapsible class=class node_ref=node_ref>
            {items_view}
        </Accordion>
    }
}
