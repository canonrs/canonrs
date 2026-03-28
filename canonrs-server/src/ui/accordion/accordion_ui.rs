//! @canon-id: accordion
//! @canon-label: Accordion
//! @canon-family: layout
//! @canon-category: Navigation
//! @canon-intent: Expand and collapse content sections
//! @canon-description: Expandable accordion sections
//! @canon-composable: true
//! @canon-capabilities: OpenClose, Multiple
//! @canon-required-parts: AccordionItem, AccordionTrigger, AccordionContent
//! @canon-optional-parts:
//! @canon-tags: accordion, collapsible, expand, sections, faq

use leptos::prelude::*;
use canonrs_core::primitives::{
    AccordionPrimitive, AccordionItemPrimitive,
    AccordionTriggerPrimitive, AccordionContentPrimitive,
    AccordionSelection,
};

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
) -> impl IntoView {
    view! {
        <AccordionItemPrimitive class=class>
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
) -> impl IntoView {
    view! {
        <AccordionContentPrimitive class=class>
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
