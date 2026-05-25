//! SidebarPreviewUnified — 10 demos via SidebarConfig

use leptos::prelude::*;
use super::sidebar_boundary::{SidebarConfig, NavGroup, NavItem, SidebarUnifiedBoundary, VisibilityState, SidebarVariant, BadgeVariant};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};
use canonrs_core::primitives::layout::grid::{GridPrimitive as Grid, GridCols, GridGap};

fn nav_items_basic() -> Vec<NavGroup> {
    vec![
        NavGroup::new("Navigation", vec![
            NavItem::new("📊", "Dashboard", "/dashboard").active(),
            NavItem::new("📁", "Projects", "/projects"),
            NavItem::new("✓", "Tasks", "/tasks"),
        ]),
        NavGroup::new("Settings", vec![
            NavItem::new("👤", "Profile", "/profile"),
            NavItem::new("⚙", "Preferences", "/preferences"),
        ]),
    ]
}

fn nav_items_with_badges() -> Vec<NavGroup> {
    vec![
        NavGroup::new("Navigation", vec![
            NavItem::new("📊", "Dashboard", "/dashboard").active(),
            NavItem::new("📁", "Projects", "/projects").badge("12", BadgeVariant::Primary),
            NavItem::new("✓", "Tasks", "/tasks").badge("5", BadgeVariant::Destructive),
            NavItem::new("💬", "Messages", "/messages").badge("3", BadgeVariant::Warning),
        ]),
        NavGroup::new("Settings", vec![
            NavItem::new("🔔", "Notifications", "/notifications").badge("New", BadgeVariant::Success),
            NavItem::new("👤", "Profile", "/profile"),
        ]),
    ]
}

fn nav_items_collapsible() -> Vec<NavGroup> {
    vec![
        NavGroup::new("Navigation", vec![
            NavItem::new("📊", "Dashboard", "/dashboard").active(),
            NavItem::new("📁", "Projects", "/projects"),
            NavItem::new("✓", "Tasks", "/tasks"),
        ]).icon("🧭").collapsible(),
        NavGroup::new("Settings", vec![
            NavItem::new("👤", "Profile", "/profile"),
            NavItem::new("⚙", "Preferences", "/preferences"),
        ]).icon("⚙").collapsible(),
    ]
}

fn nav_items_multi_level() -> Vec<NavGroup> {
    vec![
        NavGroup::new("Navigation", vec![
            NavItem::new("📊", "Dashboard", "/dashboard").active(),
            NavItem::new("📁", "Projects", "/projects").children(vec![
                NavItem::new("", "Frontend", "/projects/frontend").children(vec![
                    NavItem::new("", "Components", "/projects/frontend/components"),
                    NavItem::new("", "Pages", "/projects/frontend/pages"),
                ]),
                NavItem::new("", "Backend", "/projects/backend").children(vec![
                    NavItem::new("", "API", "/projects/backend/api"),
                    NavItem::new("", "Database", "/projects/backend/database"),
                ]),
                NavItem::new("", "Documentation", "/projects/docs"),
            ]),
            NavItem::new("✓", "Tasks", "/tasks"),
        ]),
    ]
}

pub fn config_for(demo: &str) -> SidebarConfig {
    match demo {
        "simple"     => SidebarConfig { groups: nav_items_basic(), ..Default::default() },
        "basic"      => SidebarConfig { groups: nav_items_collapsible(), ..Default::default() },
        "tooltips"   => SidebarConfig { groups: nav_items_basic(), tooltips: true, ..Default::default() },
        "multilevel" => SidebarConfig { groups: nav_items_multi_level(), ..Default::default() },
        "search"     => SidebarConfig { groups: nav_items_basic(), search: true, ..Default::default() },
        "rail"       => SidebarConfig {
            groups: nav_items_basic(),
            state: VisibilityState::Closed,
            variant: SidebarVariant::Rail,
            tooltips: false,
            ..Default::default()
        },
        "pinnable"   => SidebarConfig { groups: nav_items_basic(), pinnable: true, ..Default::default() },
        "badges"     => SidebarConfig { groups: nav_items_with_badges(), ..Default::default() },
        "groups"     => SidebarConfig { groups: nav_items_collapsible(), ..Default::default() },
        "responsive" => SidebarConfig { groups: nav_items_basic(), responsive: true, ..Default::default() },
        _            => SidebarConfig::default(),
    }
}

#[component]
pub fn SidebarPreviewUnified(#[prop(into)] demo: String) -> impl IntoView {
    let config = config_for(&demo);
    view! { <SidebarUnifiedBoundary config=config /> }
}

#[component]
pub fn SidebarShowcasePreview() -> impl IntoView {
    let demos: &[(&str, &str)] = &[
        ("simple",     "Simple"),
        ("basic",      "Collapsible Groups"),
        ("badges",     "With Badges"),
        ("search",     "With Search"),
        ("multilevel", "Multi-level"),
        ("tooltips",   "With Tooltips"),
        ("rail",       "Rail Mode"),
        ("pinnable",   "Pinnable"),
        ("groups",     "Groups"),
        ("responsive", "Responsive"),
    ];

    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Xl>
            <Grid cols=GridCols::Two gap=GridGap::Lg>
                {demos.iter().map(|(demo, label)| {
                    let config = config_for(demo);
                    view! {
                        <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                            <span data-rs-showcase-label="">{*label}</span>
                            <div data-rs-showcase-preview-stage="" style="height: 400px; overflow: hidden; border: 1px solid var(--theme-surface-border); border-radius: var(--radius-lg); position: relative;">
                                <SidebarUnifiedBoundary config=config />
                            </div>
                        </Stack>
                    }
                }).collect::<Vec<_>>()}
            </Grid>
            <p data-rs-showcase-preview-anchor="">
                "Sidebar — 10 variants: simple, collapsible, badges, search, multilevel, tooltips, rail, pinnable, groups, responsive."
            </p>
        </Stack>
    }
}
