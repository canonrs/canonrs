//! @canon-level: strict
//! @canon-owner: primitives-team
//! Accordion Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::utils::id_gen::gen_accordion_id;

#[derive(Clone, Copy, PartialEq)]
pub enum AccordionSelection {
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
    #[prop(optional)] children: Option<Children>,
    #[prop(default = AccordionSelection::Single)] selection: AccordionSelection,
    #[prop(default = true)] collapsible: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(optional)] role: Option<String>,
) -> impl IntoView {
    let resolved_id = if id.is_empty() { gen_accordion_id() } else { id };

    view! {
        <div
            data-accordion
            data-selection={selection.as_str()}
            data-accordion-collapsible={if collapsible { "true" } else { "false" }}
            class={class}
            id={resolved_id}
            role={role.unwrap_or_else(|| "region".to_string())}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn AccordionItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-accordion-item
            data-state="closed"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn AccordionTriggerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            data-accordion-trigger
            type="button"
            aria-expanded="false"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn AccordionContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-accordion-content
            aria-hidden="true"
            hidden=true
            class={class}
        >
            {children.map(|c| c())}
        </div>
    }
}
