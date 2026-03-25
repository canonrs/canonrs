//! @canon-level: ui
//! Accordion - attribute-driven
//! Relação trigger↔content via estrutura DOM (closest + query_selector)

use leptos::prelude::*;
use canonrs_core::primitives::{
    AccordionPrimitive,
    AccordionItemPrimitive,
    AccordionTriggerPrimitive,
    AccordionContentPrimitive,
    AccordionSelection,
};

#[component]
pub fn Accordion(
    children: Children,
    #[prop(default = AccordionSelection::Single)] selection: AccordionSelection,
    #[prop(default = true)] collapsible: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <AccordionPrimitive
            selection={selection}
            collapsible={collapsible}
            class={class}
            id={id.unwrap_or_default()}
        >
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
    view! {
        <AccordionItemPrimitive class={class} open={default_open.into()}>
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
        <AccordionTriggerPrimitive class={class}>
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
    view! {
        <AccordionContentPrimitive class={class} open={open.into()}>
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
