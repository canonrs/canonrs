//! SidebarShowcasePreview — legacy showcase
use leptos::prelude::*;
use super::sidebar_1_with_accordion_boundary::Sidebar1WithAccordionBoundary;

#[component]
pub fn SidebarShowcasePreview() -> impl IntoView {
    view! { <Sidebar1WithAccordionBoundary /> }
}
