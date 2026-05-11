//! SidebarUnifiedBoundary — único boundary para todos os 10 tipos

use leptos::prelude::*;
use super::sidebar_boundary::{
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarGroupLabel, SidebarSeparator,
    SidebarTrigger, SidebarGroup, SidebarGroupTrigger, SidebarGroupContent,
    SidebarLabel, SidebarIcon, SidebarUser,
};
use super::sidebar_data::{SidebarConfig, NavGroup, NavItem};
use crate::ui::badge::badge_boundary::Badge;
use crate::ui::tooltip::tooltip_boundary::{Tooltip, TooltipTrigger, TooltipContent};
use crate::ui::scroll_area::scroll_area_boundary::ScrollArea;
use crate::ui::command::command_boundary::{Command, CommandItem};
use canonrs_core::meta::{VisibilityState, ActivityState};
use canonrs_core::primitives::SidebarVariant;

#[component]
pub fn SidebarUnifiedBoundary(config: SidebarConfig) -> impl IntoView {
    let tooltips = config.tooltips;
    let pinnable = config.pinnable;
    let responsive = config.responsive;
    let user_name = config.user_name;
    let user_email = config.user_email;
    let groups = config.groups.clone();
    let search = config.search;
    let state = config.state;
    let variant = config.variant;

    view! {
        <Sidebar state=state variant=variant>
            {if pinnable {
                view! {
                    <button type="button" data-rs-sidebar-pin-toggle="">"📍"</button>
                }.into_any()
            } else {
                view! { <SidebarTrigger>"⇔"</SidebarTrigger> }.into_any()
            }}

            <SidebarHeader>
                <SidebarUser>
                    <SidebarLabel>{user_name}</SidebarLabel>
                    <SidebarLabel>{user_email}</SidebarLabel>
                </SidebarUser>
            </SidebarHeader>

            <SidebarContent>
                {if search {
                    let items: Vec<(&'static str, &'static str, &'static str)> = groups.iter()
                        .flat_map(|g| g.items.iter())
                        .map(|item| (item.icon, item.label, item.href))
                        .collect();
                    view! {
                        <div data-rs-sidebar-search="">
                            <Command placeholder="Search...".to_string()>
                                {items.into_iter().map(|(icon, label, href)| view! {
                                    <CommandItem value=href.to_string()>
                                        <SidebarIcon>{icon}</SidebarIcon>
                                        <SidebarLabel>{label}</SidebarLabel>
                                    </CommandItem>
                                }).collect_view()}
                            </Command>
                        </div>
                    }.into_any()
                } else {
                    view! { <></> }.into_any()
                }}
                <SidebarMenu>
                    {groups.into_iter().enumerate().map(|(i, group)| {
                        let items = group.items.clone();
                        let collapsible = group.collapsible;
                        let label = group.label;
                        let icon = group.icon;
                        view! {
                            <div>
                                {if collapsible {
                                    view! {
                                        <SidebarGroup root=true>
                                            <SidebarGroupTrigger>
                                                <SidebarIcon>{icon}</SidebarIcon>
                                                <SidebarLabel>{label}</SidebarLabel>
                                                <span data-rs-sidebar-group-chevron="">"▼"</span>
                                            </SidebarGroupTrigger>
                                            <SidebarGroupContent>
                                                {render_items(items, tooltips)}
                                            </SidebarGroupContent>
                                        </SidebarGroup>
                                    }.into_any()
                                } else {
                                    view! {
                                        <SidebarGroupLabel>{label}</SidebarGroupLabel>
                                        {render_items(items, tooltips)}
                                    }.into_any()
                                }}
                                {if i < 1 { view! { <SidebarSeparator /> }.into_any() } else { view! { <></> }.into_any() }}
                            </div>
                        }
                    }).collect_view()}
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
        let active = if item.active { ActivityState::Active } else { ActivityState::Inactive };
        view! {
            <div>
                {if has_children {
                    let children = item.children.clone();
                    view! {
                        <SidebarGroup state=VisibilityState::Closed>
                            <SidebarGroupTrigger>
                                <SidebarIcon>{item.icon}</SidebarIcon>
                                <SidebarLabel>{item.label}</SidebarLabel>
                                <span data-rs-sidebar-group-chevron="">"▼"</span>
                            </SidebarGroupTrigger>
                            <SidebarGroupContent>
                                {render_items(children, tooltips)}
                            </SidebarGroupContent>
                        </SidebarGroup>
                    }.into_any()
                } else {
                    let badge = item.badge.clone();
                    let label = item.label;
                    if tooltips {
                        view! {
                            <Tooltip>
                                <TooltipTrigger>
                                    <SidebarMenuItem href=item.href active=active>
                                        <SidebarIcon>{item.icon}</SidebarIcon>
                                        <SidebarLabel>{label}</SidebarLabel>
                                        {badge.map(|b| view! { <Badge variant=b.variant>{b.label}</Badge> })}
                                    </SidebarMenuItem>
                                </TooltipTrigger>
                                <TooltipContent>{label}</TooltipContent>
                            </Tooltip>
                        }.into_any()
                    } else {
                        view! {
                            <SidebarMenuItem href=item.href active=active>
                                <SidebarIcon>{item.icon}</SidebarIcon>
                                <SidebarLabel>{label}</SidebarLabel>
                                {badge.map(|b| view! { <Badge variant=b.variant>{b.label}</Badge> })}
                            </SidebarMenuItem>
                        }.into_any()
                    }
                }}
            </div>
        }
    }).collect_view()
}
