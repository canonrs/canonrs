//! @canon-level: strict
//! @canon-owner: primitives-team
//! Collapsible Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{VisibilityState, DisabledState};


#[component]
pub fn CollapsiblePrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,
) -> impl IntoView {
    let uid_col = crate::infra::uid::generate("col");
    view! {
        <div
            data-rs-collapsible=""
            data-rs-uid=uid_col
            data-rs-interaction="init"
            data-rs-visibility=state.as_str()
            data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
            aria-disabled=disabled.aria_disabled()
            class=class
            node_ref=node_ref.unwrap_or_default()
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CollapsibleTriggerPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] controls: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-collapsible-trigger=""
            data-rs-visibility=state.as_str()
            data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
            aria-expanded=state.aria_expanded()
            aria-disabled=disabled.aria_disabled()
            aria-controls=if controls.is_empty() { None } else { Some(controls) }
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn CollapsibleContentPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
) -> impl IntoView {
    let content_id = if id.is_empty() { crate::infra::uid::generate("col-content") } else { id };
    view! {
        <div
            data-rs-collapsible-content=""
            data-rs-visibility=state.as_str()
            role="region"
            aria-hidden=state.aria_hidden()
            id=content_id
            class=class
        >
            {children()}
        </div>
    }
}
