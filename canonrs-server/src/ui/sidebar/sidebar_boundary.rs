//! Sidebar Boundary — componentes passthrough + unified boundary

use leptos::prelude::*;
use super::sidebar_ui::{
    SidebarGroup as SidebarGroupUi,
    SidebarGroupTrigger as SidebarGroupTriggerUi,
    SidebarGroupContent as SidebarGroupContentUi,
    Sidebar as SidebarUi,
    SidebarHeader as SidebarHeaderUi,
    SidebarContent as SidebarContentUi,
    SidebarFooter as SidebarFooterUi,
    SidebarMenu as SidebarMenuUi,
    SidebarMenuItem as SidebarMenuItemUi,
    SidebarMenuGroup as SidebarMenuGroupUi,
    SidebarSeparator as SidebarSeparatorUi,
    SidebarGroupLabel as SidebarGroupLabelUi,
    SidebarLabel as SidebarLabelUi,
    SidebarIcon as SidebarIconUi,
    SidebarUser as SidebarUserUi,
};
pub use canonrs_core::primitives::SidebarTriggerPrimitive as SidebarTrigger;
pub use canonrs_core::primitives::SidebarVariant;
pub use canonrs_core::primitives::BadgeVariant;
pub use canonrs_core::meta::{VisibilityState, ActivityState, DisabledState};
use crate::ui::badge::badge_boundary::Badge;
use crate::ui::tooltip::tooltip_boundary::{Tooltip, TooltipTrigger, TooltipContent};
use crate::ui::command::command_boundary::{Command, CommandItem};

pub use super::sidebar_data::{NavBadge, NavItem, NavGroup, SidebarConfig};

// ─── Boundary components ─────────────────────────────────────────────────────

#[component]
pub fn Sidebar(
    children: Children,
    #[prop(default = VisibilityState::Open)] state: VisibilityState,
    #[prop(default = SidebarVariant::Default)] variant: SidebarVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SidebarUi state=state variant=variant class=class>{children()}</SidebarUi>
    }
}

#[component]
pub fn SidebarHeader(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SidebarHeaderUi class=class>{children()}</SidebarHeaderUi> }
}

#[component]
pub fn SidebarContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SidebarContentUi class=class>{children()}</SidebarContentUi> }
}

#[component]
pub fn SidebarFooter(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SidebarFooterUi class=class>{children()}</SidebarFooterUi> }
}

#[component]
pub fn SidebarMenu(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SidebarMenuUi class=class>{children()}</SidebarMenuUi> }
}

#[component]
pub fn SidebarMenuItem(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = ActivityState::Inactive)] active: ActivityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
) -> impl IntoView {
    view! { <SidebarMenuItemUi class=class href=href active=active disabled=disabled>{children()}</SidebarMenuItemUi> }
}

#[component]
pub fn SidebarMenuGroup(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional, into)] label: Option<String>,
) -> impl IntoView {
    view! { <SidebarMenuGroupUi class=class label=label.unwrap_or_default()>{children()}</SidebarMenuGroupUi> }
}

#[component]
pub fn SidebarSeparator(
    #[prop(into, default = String::new())] class: String,
    #[prop(default = false)] hidden: bool,
) -> impl IntoView {
    view! { <SidebarSeparatorUi class=class attr:hidden=hidden.then(|| "") /> }
}

#[component]
pub fn SidebarGroupLabel(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SidebarGroupLabelUi class=class>{children()}</SidebarGroupLabelUi> }
}

#[component]
pub fn SidebarGroup(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = false)] root: bool,
    #[prop(default = false)] hidden: bool,
    #[prop(default = VisibilityState::Open)] state: VisibilityState,
) -> impl IntoView {
    view! { <SidebarGroupUi class=class root=root state=state hidden=hidden>{children()}</SidebarGroupUi> }
}

#[component]
pub fn SidebarGroupTrigger(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SidebarGroupTriggerUi class=class>{children()}</SidebarGroupTriggerUi> }
}

#[component]
pub fn SidebarGroupContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SidebarGroupContentUi class=class>{children()}</SidebarGroupContentUi> }
}

#[component]
pub fn SidebarLabel(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SidebarLabelUi class=class>{children()}</SidebarLabelUi> }
}

#[component]
pub fn SidebarIcon(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SidebarIconUi class=class>{children()}</SidebarIconUi> }
}

#[component]
pub fn SidebarUser(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SidebarUserUi class=class>{children()}</SidebarUserUi> }
}


// ─── Unified Boundary ────────────────────────────────────────────────────────

#[component]
pub fn SidebarUnifiedBoundary(config: SidebarConfig) -> impl IntoView {
    let tooltips  = config.tooltips;
    let pinnable  = config.pinnable;
    let user_name  = config.user_name;
    let user_email = config.user_email;
    let groups     = config.groups.clone();
    let search     = config.search;
    let state      = config.state;
    let variant    = config.variant;

    let search_items: Vec<(String, String, String)> = groups.iter()
        .flat_map(|g| g.items.iter())
        .map(|item| (item.icon.to_string(), item.label.to_string(), item.href.to_string()))
        .collect();

    let has_separator = groups.len() > 1;

    view! {
        <Sidebar state=state variant=variant>
            <SidebarHeader>
                <div style="display:flex; align-items:center; justify-content:space-between; width:100%;">
                    <SidebarUser>
                        <SidebarLabel>{user_name}</SidebarLabel>
                        <SidebarLabel>{user_email}</SidebarLabel>
                    </SidebarUser>
                    <div style="display:flex; align-items:center; gap:var(--space-xs);">
                        <SidebarTrigger>"⇔"</SidebarTrigger>
                        <button
                            type="button"
                            data-rs-sidebar-pin-toggle=""
                            hidden=(!pinnable)
                            style="padding: var(--space-xs); background: transparent; border: none; cursor: pointer; font-size: var(--font-size-md);"
                        >
                            "📍"
                        </button>
                    </div>
                </div>
            </SidebarHeader>

            <SidebarContent>
                <div data-rs-sidebar-search="" hidden=(!search)>
                    <Command placeholder="Search...".to_string()>
                        {search_items.into_iter().map(|(icon, label, href)| {
                            let label2 = label.clone();
                            view! {
                                <CommandItem value=href>
                                    <SidebarIcon>{icon}</SidebarIcon>
                                    <SidebarLabel>{label2}</SidebarLabel>
                                </CommandItem>
                            }
                        }).collect::<Vec<_>>()}
                    </Command>
                </div>
                <SidebarMenu>
                    {groups.into_iter().enumerate().map(|(i, group)| {
                        let items      = group.items.clone();
                        let items2     = items.clone();
                        let collapsible = group.collapsible;
                        let label      = group.label.to_string();
                        let label2     = label.clone();
                        let icon       = group.icon.to_string();
                        view! {
                            <div>
                                <SidebarGroup root=true hidden=(!collapsible)>
                                    <SidebarGroupTrigger>
                                        <SidebarIcon>{icon.clone()}</SidebarIcon>
                                        <SidebarLabel>{label}</SidebarLabel>
                                        <span data-rs-sidebar-group-chevron="">"▼"</span>
                                    </SidebarGroupTrigger>
                                    <SidebarGroupContent>
                                        {render_items(items, tooltips)}
                                    </SidebarGroupContent>
                                </SidebarGroup>
                                <div hidden=collapsible>
                                    <SidebarGroupLabel>{label2}</SidebarGroupLabel>
                                    {render_items(items2, tooltips)}
                                </div>
                                <SidebarSeparator hidden=(!has_separator || i >= 1) />
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </SidebarMenu>
            </SidebarContent>

            <SidebarFooter>
                <SidebarLabel>"© 2026 CanonRS"</SidebarLabel>
            </SidebarFooter>
        </Sidebar>
    }
}

fn render_items(items: Vec<NavItem>, tooltips: bool) -> impl IntoView {
    items.into_iter().map(|item| {
        let has_children = !item.children.is_empty();
        let active       = if item.active { ActivityState::Active } else { ActivityState::Inactive };
        let children     = item.children.clone();
        let badge        = item.badge.clone();
        let label        = item.label.to_string();
        let label2       = label.clone();
        let label3       = label.clone();
        let label4       = label.clone();
        let icon         = item.icon.to_string();
        let icon2        = icon.clone();
        let icon3        = icon.clone();
        let href         = item.href.to_string();
        let href2        = href.clone();
        let has_badge    = badge.is_some();
        let badge_variant  = badge.as_ref().map(|b| b.variant.clone()).unwrap_or_default();
        let badge_variant2 = badge_variant.clone();
        let badge_label    = badge.as_ref().map(|b| b.label.to_string()).unwrap_or_default();
        let badge_label2   = badge_label.clone();

        view! {
            <div>
                <SidebarGroup state=VisibilityState::Closed hidden=(!has_children)>
                    <SidebarGroupTrigger>
                        <SidebarIcon>{icon}</SidebarIcon>
                        <SidebarLabel>{label}</SidebarLabel>
                        <span data-rs-sidebar-group-chevron="">"▼"</span>
                    </SidebarGroupTrigger>
                    <SidebarGroupContent>
                        {render_items(children, tooltips)}
                    </SidebarGroupContent>
                </SidebarGroup>
                <div hidden=has_children>
                    <div hidden=tooltips>
                        <SidebarMenuItem href=href.clone() active=active>
                            <SidebarIcon>{icon2.clone()}</SidebarIcon>
                            <SidebarLabel>{label2.clone()}</SidebarLabel>
                            <Badge variant=badge_variant.clone() hidden=(!has_badge)>{badge_label.clone()}</Badge>
                        </SidebarMenuItem>
                    </div>
                    <div hidden=(!tooltips)>
                        <Tooltip>
                            <TooltipTrigger>
                                <SidebarMenuItem href=href2 active=active>
                                    <SidebarIcon>{icon3}</SidebarIcon>
                                    <SidebarLabel>{label3}</SidebarLabel>
                                    <Badge variant=badge_variant2 hidden=(!has_badge)>{badge_label2}</Badge>
                                </SidebarMenuItem>
                            </TooltipTrigger>
                            <TooltipContent>{label4}</TooltipContent>
                        </Tooltip>
                    </div>
                </div>
            </div>
        }
    }).collect::<Vec<_>>()
}

