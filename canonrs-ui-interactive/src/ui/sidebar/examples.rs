use leptos::prelude::*;
use super::{sidebar_with_accordion::SidebarWithAccordion, sidebar_with_tooltips::SidebarWithTooltips, sidebar_with_search::SidebarWithSearch};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <SidebarWithAccordion default_collapsed=false />
    }
}

#[component]
pub fn TooltipsExample() -> impl IntoView {
    view! {
        <SidebarWithTooltips default_collapsed=false />
    }
}

#[component]
pub fn SearchExample() -> impl IntoView {
    view! {
        <SidebarWithSearch default_collapsed=false />
    }
}

#[component]
pub fn MultiLevelExample() -> impl IntoView {
    view! {
        <super::sidebar_multi_level::SidebarMultiLevel default_collapsed=false />
    }
}

#[component]
pub fn RailModeExample() -> impl IntoView {
    view! {
        <super::sidebar_rail_mode::SidebarRailMode />
    }
}

#[component]
pub fn PinnableExample() -> impl IntoView {
    view! {
        <super::sidebar_pinnable::SidebarPinnable default_collapsed=false />
    }
}

#[component]
pub fn BadgesExample() -> impl IntoView {
    view! {
        <super::sidebar_with_badges::SidebarWithBadges default_collapsed=false />
    }
}

#[component]
pub fn GroupsCollapsibleExample() -> impl IntoView {
    view! {
        <super::sidebar_groups_collapsible::SidebarGroupsCollapsible default_collapsed=false />
    }
}

#[component]
pub fn ResponsiveExample() -> impl IntoView {
    view! {
        <super::sidebar_responsive::SidebarResponsive />
    }
}
