//! @canon-level: ui
//! Collapsible - attribute-driven
//! Relação trigger↔content via estrutura DOM

use leptos::prelude::*;
use canonrs_core::primitives::{
    CollapsiblePrimitive,
    CollapsibleTriggerPrimitive,
    CollapsibleContentPrimitive,
};

#[component]
pub fn Collapsible(
    children: Children,
    #[prop(default = false)] open: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <CollapsiblePrimitive open={open} class={class} id={id.unwrap_or_default()}>
            {children()}
        </CollapsiblePrimitive>
    }
}

#[component]
pub fn CollapsibleTrigger(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CollapsibleTriggerPrimitive class={class}>
            {children()}
        </CollapsibleTriggerPrimitive>
    }
}

#[component]
pub fn CollapsibleContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CollapsibleContentPrimitive class={class}>
            {children()}
        </CollapsibleContentPrimitive>
    }
}

#[component]
pub fn CollapsiblePreview() -> impl IntoView {
    view! {
        <Collapsible>
            <CollapsibleTrigger>"Toggle"</CollapsibleTrigger>
            <CollapsibleContent>"Content"</CollapsibleContent>
        </Collapsible>
    }
}
