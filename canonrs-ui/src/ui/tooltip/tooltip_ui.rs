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
    #[prop(default = 0)] delay_duration: u32,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <TooltipProviderPrimitive
            delay_duration=delay_duration
            class=class
            id=id
        >
            {children()}
        </TooltipProviderPrimitive>
    }
}

#[component]
pub fn Tooltip(
    children: Children,
    open: Signal<bool>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <TooltipPrimitive
            open=open
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
    #[prop(default = String::new())] describedby_id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <TooltipTriggerPrimitive
            describedby_id=describedby_id
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
    #[prop(default = String::new())] content_id: String,
    #[prop(default = 0)] side_offset: i32,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TooltipContentPrimitive
            content_id=content_id
            side_offset=side_offset
            class=class
        >
            {children()}
        </TooltipContentPrimitive>
    }
}
