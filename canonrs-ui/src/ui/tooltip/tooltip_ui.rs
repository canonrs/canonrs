use leptos::prelude::*;
use crate::primitives::{
    TooltipProviderPrimitive,
    TooltipPrimitive,
    TooltipTriggerPrimitive,
    TooltipContentPrimitive,
};

#[component]
pub fn TooltipProvider(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TooltipProviderPrimitive
            class=class
        >
            {children()}
        </TooltipProviderPrimitive>
    }
}

#[component]
pub fn Tooltip(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <TooltipPrimitive
            class=class
            id=id
        >
            {children()}
        </TooltipPrimitive>
    }
}

#[component]
pub fn TooltipTrigger(
    children: Children,
    #[prop(into)] target_tooltip_id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <TooltipTriggerPrimitive
            target_tooltip_id=target_tooltip_id
            class=class
            id=id
        >
            {children()}
        </TooltipTriggerPrimitive>
    }
}

#[component]
pub fn TooltipContent(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <TooltipContentPrimitive
            class=class
            id=id
        >
            {children()}
        </TooltipContentPrimitive>
    }
}
