#![allow(unreachable_pub, dead_code)]
use leptos::prelude::*;
use canonrs_core::primitives::{
    CollapsiblePrimitive, CollapsibleTriggerPrimitive, CollapsibleContentPrimitive,
};
use canonrs_core::meta::{VisibilityState, DisabledState};
use canonrs_core::infra::uid::generate;

#[component]
pub fn Collapsible(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,
) -> impl IntoView {
    view! {
        <CollapsiblePrimitive state=state disabled=disabled class=class node_ref=node_ref.unwrap_or_default()>
            {children()}
        </CollapsiblePrimitive>
    }
}

#[component]
pub fn CollapsibleTrigger(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] controls: String,
) -> impl IntoView {
    view! {
        <CollapsibleTriggerPrimitive class=class controls=controls>
            {children()}
        </CollapsibleTriggerPrimitive>
    }
}

#[component]
pub fn CollapsibleContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <CollapsibleContentPrimitive class=class id=id>
            {children()}
        </CollapsibleContentPrimitive>
    }
}

#[component]
pub fn CollapsiblePreview() -> impl IntoView {
    let content_id = generate("col-content");
    view! {
        <Collapsible>
            <CollapsibleTrigger controls=content_id.clone()>"Toggle"</CollapsibleTrigger>
            <CollapsibleContent id=content_id>"Content"</CollapsibleContent>
        </Collapsible>
    }
}
