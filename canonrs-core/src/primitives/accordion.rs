//! @canon-level: strict
//! @canon-owner: primitives-team
//! Accordion Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{VisibilityState, DisabledState};
use crate::state_engine::{visibility_attrs, trigger_attrs, disabled_attrs};

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum AccordionSelection {
    #[default]
    Single,
    Multiple,
}

impl AccordionSelection {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Single   => "single",
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
) -> impl IntoView {
    view! {
        <div
            data-rs-accordion=""
            data-rs-component="Accordion"
            data-rs-behavior="disclosure"
            data-rs-selection=selection.as_str()
            data-rs-collapsible=if collapsible { "true" } else { "false" }
            class=class
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
    let s = visibility_attrs(state);
    let d = disabled_attrs(disabled);
    view! {
        <div
            data-rs-accordion-item=""
            data-rs-state=s.data_rs_state
            data-rs-disabled=d.data_rs_disabled
            aria-disabled=d.aria_disabled
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
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let t = trigger_attrs(state);
    let d = disabled_attrs(disabled);
    view! {
        <h3 data-rs-accordion-heading="">
            <button
                type="button"
                data-rs-accordion-trigger=""
                data-rs-state=t.data_rs_state
                data-rs-disabled=d.data_rs_disabled
                aria-expanded=t.aria_expanded
                aria-disabled=d.aria_disabled
                class=class
            >
                {children()}
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
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-accordion-content=""
            data-rs-state=s.data_rs_state
            aria-hidden=s.aria_hidden
            hidden=s.hidden
            class=class
        >
            {children()}
        </div>
    }
}
