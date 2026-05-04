//! @canon-level: strict
//! @canon-owner: primitives-team
//! Accordion Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{VisibilityState, DisabledState};
use crate::infra::uid::generate;

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Default, Debug)]
pub enum AccordionSelection {
    #[default]
    Single,
    Multiple,
}
impl AccordionSelection {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Single => "single", Self::Multiple => "multiple" }
    }
}

#[component]
pub fn AccordionPrimitive(
    children: Children,
    #[prop(default = AccordionSelection::Single)] selection: AccordionSelection,
    #[prop(into, default = "true".to_string())] collapsible: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,
) -> impl IntoView {
    let uid = generate("ac");
    view! {
        <div
            data-rs-accordion=""
            data-rs-uid=uid
            data-rs-interaction="nav"
            data-rs-selection=selection.as_str()
            data-rs-collapsible=collapsible
            class=class
            node_ref=node_ref.unwrap_or_default()
        >
            {children()}
        </div>
    }
}

#[component]
pub fn AccordionItemPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid = generate("ac-item");
    view! {
        <div
            data-rs-accordion-item=""
            data-rs-uid=uid
            data-rs-visibility=state.as_str()
            data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
            aria-disabled=disabled.aria_disabled()
            role="group"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn AccordionTriggerPrimitive(
    children: Children,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <h3 data-rs-accordion-heading="">
            <button
                type="button"
                data-rs-accordion-trigger=""
                data-rs-uid=generate("ac-trigger")
                data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
                aria-expanded=state.aria_expanded()
                aria-disabled=disabled.aria_disabled()
                class=class
            >
                {children()}
                <svg data-rs-accordion-icon="" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
                    <path d="m6 9 6 6 6-6"/>
                </svg>
            </button>
        </h3>
    }
}

#[component]
pub fn AccordionContentPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid = generate("ac-content");
    view! {
        <div
            data-rs-accordion-content=""
            data-rs-uid=uid
            data-rs-visibility=state.as_str()
            class=class
        >
            {children()}
        </div>
    }
}
