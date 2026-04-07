use leptos::prelude::*;
use canonrs_core::primitives::{
    CollapsiblePrimitive, CollapsibleTriggerPrimitive, CollapsibleContentPrimitive,
};
use canonrs_core::meta::VisibilityState;

#[component]
pub fn Collapsible(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,
) -> impl IntoView {
    view! {
        <CollapsiblePrimitive state=state class=class node_ref=node_ref.unwrap_or_default()>
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
