//! @canon-level: strict
//! Accordion Island — Canon Rule #340 (zero-logic boundary)
//! CR-342 v3.0.0: interaction delegated to canonrs-interactions-init

use leptos::prelude::*;
use super::accordion_ui::{Accordion, AccordionItem, AccordionTrigger, AccordionContent};
use canonrs_core::meta::{VisibilityState, DisabledState};
use canonrs_core::primitives::AccordionSelection;

#[component]
pub fn AccordionIsland(
    children: Children,
    #[prop(default = AccordionSelection::Single)] selection: AccordionSelection,
    #[prop(default = true)] collapsible: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <Accordion selection=selection collapsible=collapsible class=class>{children()}</Accordion> }
}

#[component]
pub fn AccordionItemIsland(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <AccordionItem state=state disabled=disabled class=class>{children()}</AccordionItem> }
}

#[component]
pub fn AccordionTriggerIsland(
    children: Children,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <AccordionTrigger disabled=disabled class=class>{children()}</AccordionTrigger> }
}

#[component]
pub fn AccordionContentIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <AccordionContent class=class>{children()}</AccordionContent> }
}
