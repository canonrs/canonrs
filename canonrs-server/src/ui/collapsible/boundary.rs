//! @canon-level: strict
//! Collapsible Island — Canon Rule #340 (zero-logic boundary)
//! CR-342 v3.0.0: interaction delegated to canonrs-interactions-init

use leptos::prelude::*;
use super::collapsible_ui::{
    Collapsible as CollapsibleUi,
    CollapsibleTrigger as CollapsibleTriggerUi,
    CollapsibleContent as CollapsibleContentUi
};
use canonrs_core::meta::VisibilityState;

#[component]
pub fn Collapsible(
    children: Children,
    #[prop(default = false)] open: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let state = if open { VisibilityState::Open } else { VisibilityState::Closed };
    view! { <CollapsibleUi state=state class=class>{children()}</CollapsibleUi> }
}

#[component]
pub fn CollapsibleTrigger(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <CollapsibleTriggerUi class=class>{children()}</CollapsibleTriggerUi> }
}

#[component]
pub fn CollapsibleContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <CollapsibleContentUi class=class>{children()}</CollapsibleContentUi> }
}
