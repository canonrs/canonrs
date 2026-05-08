//! Preview: Sidebar com Groups Colapsáveis
//! Demonstra grupos de navegação com toggle via disclosure DOM-driven

use leptos::prelude::*;
use super::sidebar_boundary::{
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarSeparator, SidebarTrigger,
    SidebarGroup, SidebarGroupTrigger, SidebarGroupContent,
};
use canonrs_core::meta::{VisibilityState, ActivityState};
use crate::ui::badge::badge_boundary::Badge;
use canonrs_core::primitives::BadgeVariant;

#[component]
pub fn SidebarPreviewGroupsCollapsible() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-anchor="" data-rs-showcase-preview-label="Groups Collapsible">
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
                        <SidebarMenu>
                            <SidebarGroup>
                                <SidebarGroupTrigger>
                                    <span>"Navigation"</span>
                                    <span data-rs-sidebar-group-chevron="">"▼"</span>
                                </SidebarGroupTrigger>
                                <SidebarGroupContent>
                                    <SidebarMenuItem href="/dashboard" active=ActivityState::Active>
                                        "Dashboard"
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
                                </SidebarGroupContent>
                            </SidebarGroup>

                            <SidebarSeparator />

                            <SidebarGroup>
                                <SidebarGroupTrigger>
                                    <span>"Settings"</span>
                                    <span data-rs-sidebar-group-chevron="">"▼"</span>
                                </SidebarGroupTrigger>
                                <SidebarGroupContent>
                                    <SidebarMenuItem href="/profile">"Profile"</SidebarMenuItem>
                                    <SidebarMenuItem href="/preferences">"Preferences"</SidebarMenuItem>
                                    <SidebarMenuItem href="/notifications">
                                        "Notifications"
                                        <Badge variant=BadgeVariant::Success>"New"</Badge>
                                    </SidebarMenuItem>
                                </SidebarGroupContent>
                            </SidebarGroup>
                        </SidebarMenu>
                    </SidebarContent>

                    <SidebarFooter>
                        <span class="sidebar-footer-label">"© 2026 CanonRS"</span>
                    </SidebarFooter>
                </Sidebar>
            </div>
        </div>
    }
}
