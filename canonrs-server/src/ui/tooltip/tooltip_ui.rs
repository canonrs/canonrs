//! @canon-level: ui
use leptos::prelude::*;
use canonrs_core::primitives::{
    TooltipProviderPrimitive, TooltipPrimitive,
    TooltipTriggerPrimitive, TooltipContentPrimitive,
};

#[component]
pub fn TooltipProvider(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TooltipProviderPrimitive class=class>
            {children()}
        </TooltipProviderPrimitive>
    }
}

#[component]
pub fn Tooltip(
    children: Children,
    #[prop(default = false)] open: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TooltipPrimitive open=open class=class>
            {children()}
        </TooltipPrimitive>
    }
}

#[component]
pub fn TooltipTrigger(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TooltipTriggerPrimitive class=class>
            {children()}
        </TooltipTriggerPrimitive>
    }
}

#[component]
pub fn TooltipContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TooltipContentPrimitive class=class>
            {children()}
        </TooltipContentPrimitive>
    }
}

#[component]
pub fn TooltipPreview() -> impl IntoView {
    view! {
        <TooltipProvider>
            <Tooltip>
                <TooltipTrigger>"Hover me"</TooltipTrigger>
                <TooltipContent>"Tooltip"</TooltipContent>
            </Tooltip>
        </TooltipProvider>
    }
}
