//! @canon-id: collapsible
//! @canon-label: Collapsible
//! @canon-family: layout
//! @canon-category: Navigation
//! @canon-intent: Show and hide content sections
//! @canon-description: Collapsible section
//! @canon-composable: true
//! @canon-capabilities: OpenClose
//! @canon-required-parts: CollapsibleTrigger, CollapsibleContent
//! @canon-optional-parts:
//! @canon-tags: collapsible, collapse, expand, hide, toggle

use leptos::prelude::*;
use canonrs_core::primitives::{
    CollapsiblePrimitive, CollapsibleTriggerPrimitive, CollapsibleContentPrimitive,
};
use canonrs_core::meta::VisibilityState;

#[component]
pub fn Collapsible(
    children: Children,
    #[prop(default = false)] open: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let state = if open { VisibilityState::Open } else { VisibilityState::Closed };
    view! {
        <CollapsiblePrimitive state=state class=class>
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
        <CollapsibleTriggerPrimitive class=class>
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
        <CollapsibleContentPrimitive class=class>
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
