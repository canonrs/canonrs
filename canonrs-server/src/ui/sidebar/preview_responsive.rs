//! Preview: Sidebar Responsiva
//! Demonstra sidebar com overlay e hamburger button para mobile

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
pub fn SidebarPreviewResponsive() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-anchor="" data-rs-showcase-preview-label="Responsive">
            <div data-rs-sidebar-responsive="" class="responsive-sidebar-container">

                <button
                    type="button"
                    data-rs-sidebar-toggle=""
                    class="hamburger-btn"
                >
                    "☰"
                </button>

                <div data-rs-sidebar-overlay="" class="sidebar-overlay" />

                <div class="sidebar-wrapper">
                    <Sidebar state=VisibilityState::Closed>
                        <SidebarTrigger class="close-btn">"✕"</SidebarTrigger>

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
                                </SidebarMenu>
                            </SidebarMenuGroup>

                            <SidebarSeparator />

                            <SidebarMenuGroup label="Settings">
                                <SidebarGroupLabel>"Settings"</SidebarGroupLabel>
                                <SidebarMenu>
                                    <SidebarMenuItem href="/profile">"Profile"</SidebarMenuItem>
                                </SidebarMenu>
                            </SidebarMenuGroup>
                        </SidebarContent>

                        <SidebarFooter>
                            <span class="sidebar-footer-label">"© 2026 CanonRS"</span>
                        </SidebarFooter>
                    </Sidebar>
                </div>

            </div>
        </div>
    }
}
