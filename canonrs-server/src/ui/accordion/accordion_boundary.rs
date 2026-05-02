//! @canon-level: strict
//! Accordion Island — Canon Rule #340 (zero-logic boundary)
//! CR-342 v4.0.0: interaction delegated to canonrs-interactions-nav

use leptos::prelude::*;
use super::accordion_ui::{
    Accordion as AccordionUi,
    AccordionItem as AccordionItemUi,
    AccordionTrigger as AccordionTriggerUi,
    AccordionContent as AccordionContentUi
};
use canonrs_core::meta::{VisibilityState, DisabledState};
pub use canonrs_core::primitives::AccordionSelection;

#[component]
pub fn Accordion(
    children: Children,
    #[prop(default = AccordionSelection::Single)] selection: AccordionSelection,
    #[prop(into, default = "true".to_string())] collapsible: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <AccordionUi selection=selection collapsible=collapsible class=class>{children()}</AccordionUi> }
}

#[component]
pub fn AccordionItem(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <AccordionItemUi state=state disabled=disabled class=class>{children()}</AccordionItemUi> }
}

#[component]
pub fn AccordionTrigger(
    children: Children,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <AccordionTriggerUi disabled=disabled class=class>{children()}</AccordionTriggerUi> }
}

#[component]
pub fn AccordionContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <AccordionContentUi class=class>{children()}</AccordionContentUi> }
}
