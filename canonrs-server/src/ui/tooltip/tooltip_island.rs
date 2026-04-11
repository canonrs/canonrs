//! @canon-level: strict
//! Tooltip Island — Canon Rule #340 (zero-logic boundary)
//! CR-342 v3.0.0: interaction delegated to canonrs-interactions-overlay

use leptos::prelude::*;
use super::tooltip_ui::{TooltipProvider, Tooltip, TooltipTrigger, TooltipContent};
use canonrs_core::primitives::TooltipSide;

#[component]
pub fn TooltipProviderIsland(
    children: Children,
    #[prop(default = 400)] delay_open: u32,
    #[prop(default = 100)] delay_close: u32,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <TooltipProvider delay_open=delay_open delay_close=delay_close class=class>{children()}</TooltipProvider> }
}

#[component]
pub fn TooltipIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <Tooltip class=class>{children()}</Tooltip> }
}

#[component]
pub fn TooltipTriggerIsland(
    children: Children,
    #[prop(into, optional)] tooltip_id: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let tooltip_id = tooltip_id.unwrap_or_default();
    view! { <TooltipTrigger tooltip_id=tooltip_id class=class>{children()}</TooltipTrigger> }
}

#[component]
pub fn TooltipContentIsland(
    children: Children,
    #[prop(default = TooltipSide::Top)] side: TooltipSide,
    #[prop(into, optional)] tooltip_id: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let tooltip_id = tooltip_id.unwrap_or_default();
    view! { <TooltipContent side=side tooltip_id=tooltip_id class=class>{children()}</TooltipContent> }
}
