//! @canon-level: strict
//! Collapsible Island — apenas muta DOM via web_sys
//! SEM signals, SEM lógica de negócio, SEM estado reativo

use leptos::prelude::*;
use super::collapsible_ui::{Collapsible, CollapsibleTrigger, CollapsibleContent};
use canonrs_core::meta::VisibilityState;

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

#[island]
pub fn CollapsibleIsland(
    children: Children,
    #[prop(optional)] open: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let initial_open = open.unwrap_or(false);
    let class        = class.unwrap_or_default();
    let initial_state = if initial_open { VisibilityState::Open } else { VisibilityState::Closed };

    let node_ref = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;

        let Some(root_html) = node_ref.get() else { return };
        let root: web_sys::Element = (*root_html).clone().unchecked_into();

        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
            if target.closest("[data-rs-collapsible-trigger]").ok().flatten().is_none() { return; }

            let is_open = root_cb.get_attribute("data-rs-state")
                .map(|s| s.contains("open"))
                .unwrap_or(false);

            if is_open {
                remove_state(&root_cb, "open");
                add_state(&root_cb, "closed");
                if let Ok(Some(trigger)) = root_cb.query_selector("[data-rs-collapsible-trigger]") {
                    let _ = trigger.set_attribute("aria-expanded", "false");
                }
                if let Ok(Some(content)) = root_cb.query_selector("[data-rs-collapsible-content]") {
                    remove_state(&content, "open");
                    add_state(&content, "closed");
                    let _ = content.set_attribute("aria-hidden", "true");
                }
            } else {
                remove_state(&root_cb, "closed");
                add_state(&root_cb, "open");
                if let Ok(Some(trigger)) = root_cb.query_selector("[data-rs-collapsible-trigger]") {
                    let _ = trigger.set_attribute("aria-expanded", "true");
                }
                if let Ok(Some(content)) = root_cb.query_selector("[data-rs-collapsible-content]") {
                    remove_state(&content, "closed");
                    add_state(&content, "open");
                    let _ = content.set_attribute("aria-hidden", "false");
                }
            }
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    });

    view! {
        <Collapsible state=initial_state class=class node_ref=node_ref>
            {children()}
        </Collapsible>
    }
}

#[island]
pub fn CollapsibleTriggerIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <CollapsibleTrigger class=class.unwrap_or_default()>
            {children()}
        </CollapsibleTrigger>
    }
}

#[island]
pub fn CollapsibleContentIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <CollapsibleContent class=class.unwrap_or_default()>
            {children()}
        </CollapsibleContent>
    }
}
