#![allow(unreachable_pub, dead_code)]

use leptos::prelude::*;
use canonrs_core::primitives::{
    TooltipProviderPrimitive, TooltipPrimitive,
    TooltipTriggerPrimitive, TooltipContentPrimitive, TooltipSide,
};
use canonrs_core::meta::VisibilityState;

#[component]
pub fn TooltipProvider(
    children: Children,
    #[prop(default = 400)] delay_open: u32,
    #[prop(default = 100)] delay_close: u32,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TooltipProviderPrimitive delay_open=delay_open delay_close=delay_close class=class>
            {children()}
        </TooltipProviderPrimitive>
    }
}

#[component]
pub fn Tooltip(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TooltipPrimitive state=state class=class>
            {children()}
        </TooltipPrimitive>
    }
}

#[component]
pub fn TooltipTrigger(
    children: Children,
    #[prop(into, optional)] tooltip_id: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TooltipTriggerPrimitive tooltip_id=tooltip_id.unwrap_or_default() class=class>
            {children()}
        </TooltipTriggerPrimitive>
    }
}

#[component]
pub fn TooltipContent(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(default = TooltipSide::Top)] side: TooltipSide,
    #[prop(into, optional)] tooltip_id: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TooltipContentPrimitive state=state side=side tooltip_id=tooltip_id.unwrap_or_default() class=class>
            {children()}
        </TooltipContentPrimitive>
    }
}

#[component]
pub fn TooltipPreview() -> impl IntoView {
    view! {
        <TooltipProvider>
            <Tooltip>
                <TooltipTrigger tooltip_id="preview-tooltip".to_string()>"Hover me"</TooltipTrigger>
                <TooltipContent tooltip_id="preview-tooltip".to_string()>"Tooltip"</TooltipContent>
            </Tooltip>
        </TooltipProvider>
    }
}
