//! Preview: Sidebar com Badges
//! Demonstra itens de navegação com contadores e status

use leptos::prelude::*;
use super::sidebar_boundary::{
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarMenuGroup,
    SidebarGroupLabel, SidebarSeparator, SidebarTrigger,
};
use canonrs_core::meta::{VisibilityState, ActivityState};
use crate::ui::badge::badge_boundary::Badge;
use canonrs_core::primitives::BadgeVariant;

#[component]
pub fn SidebarPreviewWithBadges() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-anchor="" data-rs-showcase-preview-label="With Badges">
            <div style="position: relative; height: 480px; overflow: hidden;">
                <Sidebar state=VisibilityState::Open>
                    <SidebarTrigger class="sidebar-trigger-abs">"☰"</SidebarTrigger>

                    <SidebarHeader>
                        <div class="sidebar-user-block">
                            <span class="sidebar-user-name">"John Doe"</span>
                            <span class="sidebar-user-email">"john@canonrs.dev"</span>
                        </div>
                    </SidebarHeader>

                    <SidebarContent>
                        <SidebarMenuGroup label="Navigation">
                            <SidebarGroupLabel>"Navigation"</SidebarGroupLabel>
                            <SidebarMenu>
                                <SidebarMenuItem href="/dashboard" active=ActivityState::Active>
                                    "Dashboard"
                                </SidebarMenuItem>
                                <SidebarMenuItem href="/projects">
                                    "Projects"
                                    <Badge variant=BadgeVariant::Primary>"12"</Badge>
                                </SidebarMenuItem>
                                <SidebarMenuItem href="/tasks">
                                    "Tasks"
                                    <Badge variant=BadgeVariant::Destructive>"5"</Badge>
                                </SidebarMenuItem>
                                <SidebarMenuItem href="/messages">
                                    "Messages"
                                    <Badge variant=BadgeVariant::Warning>"3"</Badge>
                                </SidebarMenuItem>
                                <SidebarMenuItem href="/notifications">
                                    "Notifications"
                                    <Badge variant=BadgeVariant::Success>"New"</Badge>
                                </SidebarMenuItem>
                            </SidebarMenu>
                        </SidebarMenuGroup>

                        <SidebarSeparator />

                        <SidebarMenuGroup label="Settings">
                            <SidebarGroupLabel>"Settings"</SidebarGroupLabel>
                            <SidebarMenu>
                                <SidebarMenuItem href="/profile">"Profile"</SidebarMenuItem>
                                <SidebarMenuItem href="/preferences">"Preferences"</SidebarMenuItem>
                            </SidebarMenu>
                        </SidebarMenuGroup>
                    </SidebarContent>

                    <SidebarFooter>
                        <span class="sidebar-footer-label">"© 2026 CanonRS"</span>
                    </SidebarFooter>
                </Sidebar>
            </div>
        </div>
    }
}
