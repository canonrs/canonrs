//! @canon-level: strict
//! @canon-owner: primitives-team
//! Accordion Primitive - HTML puro + ARIA

use leptos::prelude::*;

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
    view! {
        <div
            data-rs-accordion=""
            data-selection={selection.as_str()}
            data-accordion-collapsible={if collapsible { "true" } else { "false" }}
            class={class}
            id={id}
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
    #[prop(default = false)] default_open: bool,
) -> impl IntoView {
    view! {
        <div data-rs-accordion-item="" data-state={if default_open { "open" } else { "closed" }} class={class} id={id}>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn AccordionTriggerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] controls: String,
    #[prop(default = false)] default_open: bool,
) -> impl IntoView {
    view! {
        <button
            data-rs-accordion-trigger=""
            type="button"
            aria-expanded={if default_open { "true" } else { "false" }}
            aria-controls={(!controls.is_empty()).then(|| controls)}
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
    #[prop(default = String::new())] id: String,
    #[prop(default = false)] default_open: bool,
) -> impl IntoView {
    view! {
        <div
            data-rs-accordion-content=""
            aria-hidden={if default_open { "false" } else { "true" }}
            hidden=(!default_open)
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}
