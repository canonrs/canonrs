//! @canon-level: strict
//! Collapsible Island — Canon Rule #340 (zero-logic boundary)
//! CR-342 v3.0.0: interaction delegated to canonrs-interactions-init

use leptos::prelude::*;
use super::collapsible_ui::{Collapsible, CollapsibleTrigger, CollapsibleContent};
use canonrs_core::meta::VisibilityState;

#[component]
pub fn CollapsibleIsland(
    children: Children,
    #[prop(default = false)] open: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let state = if open { VisibilityState::Open } else { VisibilityState::Closed };
    view! { <Collapsible state=state class=class>{children()}</Collapsible> }
}

#[component]
pub fn CollapsibleTriggerIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <CollapsibleTrigger class=class>{children()}</CollapsibleTrigger> }
}

#[component]
pub fn CollapsibleContentIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <CollapsibleContent class=class>{children()}</CollapsibleContent> }
}
