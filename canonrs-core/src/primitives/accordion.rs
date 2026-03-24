//! @canon-level: strict
//! @canon-owner: primitives-team
//! Accordion Primitive - HTML puro + ARIA

use leptos::prelude::*;

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
    #[prop(optional)] children: Option<Children>,
    #[prop(default = AccordionSelection::Single)] selection: AccordionSelection,
    #[prop(default = true)] collapsible: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-accordion=""
            data-rs-selection=selection.as_str()
            data-rs-collapsible={if collapsible { "true" } else { "false" }}
            role="region"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn AccordionItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] open: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-accordion-item=""
            data-rs-state={if open { "open" } else { "closed" }}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn AccordionTriggerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] open: bool,
    #[prop(optional)] controls: Option<String>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-accordion-trigger=""
            aria-expanded={if open { "true" } else { "false" }}
            aria-controls=controls
            class=class
            id=id
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn AccordionContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] open: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-accordion-content=""
            data-rs-state={if open { "open" } else { "closed" }}
            aria-hidden={if open { "false" } else { "true" }}
            hidden=!open
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}
