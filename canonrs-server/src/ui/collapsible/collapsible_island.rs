//! @canon-level: strict
//! Collapsible Island — apenas muta DOM via web_sys
//! SEM signals, SEM lógica de negócio, SEM estado reativo

use leptos::prelude::*;
use super::collapsible_ui::{Collapsible, CollapsibleTrigger, CollapsibleContent};
use canonrs_core::meta::VisibilityState;

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

            let state = root_cb.get_attribute("data-rs-state").unwrap_or_default();
            let is_open = state.contains("open");

            if is_open {
                let _ = root_cb.set_attribute("data-rs-state", "closed");
                if let Ok(Some(content)) = root_cb.query_selector("[data-rs-collapsible-content]") {
                    let _ = content.set_attribute("data-rs-state", "closed");
                    let _ = content.set_attribute("aria-hidden", "true");
                }
                if let Ok(Some(trigger)) = root_cb.query_selector("[data-rs-collapsible-trigger]") {
                    let _ = trigger.set_attribute("aria-expanded", "false");
                }
            } else {
                let _ = root_cb.set_attribute("data-rs-state", "open");
                if let Ok(Some(content)) = root_cb.query_selector("[data-rs-collapsible-content]") {
                    let _ = content.set_attribute("data-rs-state", "open");
                    let _ = content.set_attribute("aria-hidden", "false");
                }
                if let Ok(Some(trigger)) = root_cb.query_selector("[data-rs-collapsible-trigger]") {
                    let _ = trigger.set_attribute("aria-expanded", "true");
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
