#![allow(unreachable_pub, dead_code)]
use leptos::prelude::*;
use canonrs_core::meta::{VisibilityState, DisabledState};
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
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AccordionPrimitive selection=selection collapsible=collapsible class=class node_ref=node_ref.unwrap_or_default()>
            {children()}
        </AccordionPrimitive>
    }
}

#[component]
pub fn AccordionItem(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AccordionItemPrimitive state=state disabled=disabled class=class>
            {children()}
        </AccordionItemPrimitive>
    }
}

#[component]
pub fn AccordionTrigger(
    children: Children,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AccordionTriggerPrimitive disabled=disabled class=class>
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
