//! Preview: Sidebar With Accordion
//! Chama apenas o boundary — zero lógica

use leptos::prelude::*;
use super::sidebar_with_accordion_boundary::SidebarWithAccordionBoundary;

#[component]
pub fn SidebarPreviewWithAccordion() -> impl IntoView {
    view! {
        <SidebarWithAccordionBoundary />
    }
}
