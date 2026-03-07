use leptos::prelude::*;
use canonrs_core::primitives::{
    CollapsiblePrimitive,
    CollapsibleTriggerPrimitive,
    CollapsibleContentPrimitive
};

#[component]
pub fn Collapsible(
    children: Children,
    #[prop(default = false)] open: bool,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <CollapsiblePrimitive 
            open={open} 
            class={class.unwrap_or_default()} 
            id={id.unwrap_or_default()}
        >
            {children()}
        </CollapsiblePrimitive>
    }
}

#[component]
pub fn CollapsibleTrigger(
    children: Children,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <CollapsibleTriggerPrimitive 
            class={class.unwrap_or_default()} 
            id={id.unwrap_or_default()}
        >
            {children()}
        </CollapsibleTriggerPrimitive>
    }
}

#[component]
pub fn CollapsibleContent(
    children: Children,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <CollapsibleContentPrimitive 
            class={class.unwrap_or_default()} 
            id={id.unwrap_or_default()}
        >
            {children()}
        </CollapsibleContentPrimitive>
    }
}

#[component]
pub fn CollapsiblePreview() -> impl IntoView {
    view! {
        <Collapsible id="col-preview".to_string()>
            <CollapsibleTrigger>"Toggle"</CollapsibleTrigger>
            <CollapsibleContent>"Content"</CollapsibleContent>
        </Collapsible>
    }
}
