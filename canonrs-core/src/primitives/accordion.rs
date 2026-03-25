//! @canon-level: strict
//! @canon-owner: primitives-team
//! Accordion Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::VisibilityState;
use crate::state_engine::visibility_attrs;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum AccordionSelection {
    #[default]
    Single,
    Multiple,
}

impl AccordionSelection {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Multiple => "multiple",
        }
    }
}

#[component]
pub fn AccordionPrimitive(
    children: Children,
    #[prop(default = AccordionSelection::Single)] selection: AccordionSelection,
    #[prop(default = true)] collapsible: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-accordion=""
            data-rs-component="Accordion"
            data-rs-behavior="disclosure"
            data-rs-selection=selection.as_str()
            data-rs-collapsible={if collapsible { Some("true") } else { None }}
            role="region"
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn AccordionItemPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] open: VisibilityState,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-accordion-item=""
            data-rs-state={visibility_attrs(open).data_rs_state}
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn AccordionTriggerPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] open: VisibilityState,
    #[prop(optional)] controls: Option<String>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    // aria-controls só emitido se controls tiver valor real
    let aria_controls = controls.filter(|s| !s.is_empty());
    view! {
        <button
            type="button"
            data-rs-accordion-trigger=""
            aria-expanded={visibility_attrs(open).aria_expanded}
            aria-controls=aria_controls
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </button>
    }
}

#[component]
pub fn AccordionContentPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] open: VisibilityState,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-accordion-content=""
            data-rs-state={visibility_attrs(open).data_rs_state}
            aria-hidden={visibility_attrs(open).aria_hidden}
            hidden={visibility_attrs(open).hidden}
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </div>
    }
}
