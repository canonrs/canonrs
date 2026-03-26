//! @canon-level: ui
//! Accordion - attribute-driven

use leptos::prelude::*;
use canonrs_core::primitives::{
    AccordionPrimitive, AccordionItemPrimitive,
    AccordionTriggerPrimitive, AccordionContentPrimitive,
    AccordionSelection,
};
use canonrs_core::meta::VisibilityState;

#[component]
pub fn Accordion(
    children: Children,
    #[prop(default = AccordionSelection::Single)] selection: AccordionSelection,
    #[prop(default = true)] collapsible: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AccordionPrimitive selection=selection collapsible=collapsible class=class>
            {children()}
        </AccordionPrimitive>
    }
}

#[component]
pub fn AccordionItem(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = false)] default_open: bool,
) -> impl IntoView {
    let state = if default_open { VisibilityState::Open } else { VisibilityState::Closed };
    view! {
        <AccordionItemPrimitive class=class state=state>
            {children()}
        </AccordionItemPrimitive>
    }
}

#[component]
pub fn AccordionTrigger(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AccordionTriggerPrimitive class=class>
            {children()}
        </AccordionTriggerPrimitive>
    }
}

#[component]
pub fn AccordionContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = false)] open: bool,
) -> impl IntoView {
    let state = if open { VisibilityState::Open } else { VisibilityState::Closed };
    view! {
        <AccordionContentPrimitive class=class state=state>
            {children()}
        </AccordionContentPrimitive>
    }
}

#[component]
pub fn AccordionPreview() -> impl IntoView {
    view! {
        <Accordion>
            <AccordionItem>
                <AccordionTrigger>"Item 1"</AccordionTrigger>
                <AccordionContent>"Content 1"</AccordionContent>
            </AccordionItem>
            <AccordionItem>
                <AccordionTrigger>"Item 2"</AccordionTrigger>
                <AccordionContent>"Content 2"</AccordionContent>
            </AccordionItem>
        </Accordion>
    }
}
