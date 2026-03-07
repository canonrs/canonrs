use leptos::prelude::*;
use super::{
    sidebar_example_with_accordion::SidebarWithAccordion,
    sidebar_example_with_tooltips::SidebarWithTooltips,
    sidebar_example_with_search::SidebarWithSearch,
    sidebar_example_multi_level::SidebarMultiLevel,
    sidebar_example_rail_mode::SidebarRailMode,
    sidebar_example_pinnable::SidebarPinnable,
    sidebar_example_with_badges::SidebarWithBadges,
    sidebar_example_groups_collapsible::SidebarGroupsCollapsible,
    sidebar_example_responsive::SidebarResponsive,
};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! { <SidebarWithAccordion default_collapsed=false /> }
}

#[component]
pub fn TooltipsExample() -> impl IntoView {
    view! { <SidebarWithTooltips default_collapsed=false /> }
}

#[component]
pub fn SearchExample() -> impl IntoView {
    view! { <SidebarWithSearch default_collapsed=false /> }
}

#[component]
pub fn MultiLevelExample() -> impl IntoView {
    view! { <SidebarMultiLevel default_collapsed=false /> }
}

#[component]
pub fn RailModeExample() -> impl IntoView {
    view! { <SidebarRailMode /> }
}

#[component]
pub fn PinnableExample() -> impl IntoView {
    view! { <SidebarPinnable default_collapsed=false /> }
}

#[component]
pub fn BadgesExample() -> impl IntoView {
    view! { <SidebarWithBadges default_collapsed=false /> }
}

#[component]
pub fn GroupsCollapsibleExample() -> impl IntoView {
    view! { <SidebarGroupsCollapsible default_collapsed=false /> }
}

#[component]
pub fn ResponsiveExample() -> impl IntoView {
    view! { <SidebarResponsive /> }
}
