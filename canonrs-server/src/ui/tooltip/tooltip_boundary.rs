//! @canon-level: strict
//! Tooltip Island — Canon Rule #340 (zero-logic boundary)
//! CR-342 v3.0.0: interaction delegated to canonrs-interactions-overlay

use leptos::prelude::*;
use super::tooltip_ui::{
    TooltipProvider as TooltipProviderUi,
    Tooltip as TooltipUi,
    TooltipTrigger as TooltipTriggerUi,
    TooltipContent as TooltipContentUi
};
pub use canonrs_core::primitives::TooltipSide;

#[component]
pub fn TooltipProvider(
    children: Children,
    #[prop(default = 400)] delay_open: u32,
    #[prop(default = 100)] delay_close: u32,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <TooltipProviderUi delay_open=delay_open delay_close=delay_close class=class>{children()}</TooltipProviderUi> }
}

#[component]
pub fn Tooltip(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <TooltipUi class=class>{children()}</TooltipUi> }
}

#[component]
pub fn TooltipTrigger(
    children: Children,
    #[prop(into, optional)] tooltip_id: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let tooltip_id = tooltip_id.unwrap_or_default();
    view! { <TooltipTriggerUi tooltip_id=tooltip_id class=class>{children()}</TooltipTriggerUi> }
}

#[component]
pub fn TooltipContent(
    children: Children,
    #[prop(default = TooltipSide::Top)] side: TooltipSide,
    #[prop(into, optional)] tooltip_id: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let tooltip_id = tooltip_id.unwrap_or_default();
    view! { <TooltipContentUi side=side tooltip_id=tooltip_id class=class>{children()}</TooltipContentUi> }
}
