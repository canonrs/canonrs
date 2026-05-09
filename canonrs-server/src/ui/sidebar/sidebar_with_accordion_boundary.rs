//! SidebarWithAccordionBoundary — Boundary Tipo 3: Interaction
//! Composição: Sidebar + SidebarGroup collapsible (nav tree disclosure)
//! SEM Accordion — disclosure via engine do sidebar

use leptos::prelude::*;
use super::sidebar_boundary::{
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarGroupLabel,
    SidebarSeparator, SidebarTrigger,
    SidebarGroup, SidebarGroupTrigger, SidebarGroupContent,
};
use canonrs_core::meta::{VisibilityState, ActivityState};

#[component]
pub fn SidebarWithAccordionBoundary() -> impl IntoView {
    view! {
        <Sidebar state=VisibilityState::Open>
            <SidebarTrigger>"⇔"</SidebarTrigger>

            <SidebarHeader>
                <div data-sidebar-user="">
                    <div data-sidebar-user-avatar="">"JD"</div>
                    <div data-sidebar-label="">
                        <div data-sidebar-user-name="">"John Doe"</div>
                        <div data-sidebar-user-email="">"john@canonrs.dev"</div>
                    </div>
                </div>
            </SidebarHeader>

            <SidebarContent>
                <SidebarMenu>
                    <SidebarGroupLabel>"Navigation"</SidebarGroupLabel>
                    <SidebarMenuItem href="/dashboard" active=ActivityState::Active>
                        <span data-sidebar-icon="">"📊"</span>
                        <span data-sidebar-label="">"Dashboard"</span>
                    </SidebarMenuItem>

                    <SidebarGroup>
                        <SidebarGroupTrigger>
                            <span data-sidebar-icon="">"📁"</span>
                            <span data-sidebar-label="">"Projects"</span>
                            <span data-rs-sidebar-group-chevron="">"▼"</span>
                        </SidebarGroupTrigger>
                        <SidebarGroupContent>
                            <SidebarMenuItem href="/projects/web">
                                <span data-sidebar-label="">"Web App"</span>
                            </SidebarMenuItem>
                            <SidebarMenuItem href="/projects/mobile">
                                <span data-sidebar-label="">"Mobile App"</span>
                            </SidebarMenuItem>
                            <SidebarMenuItem href="/projects/api">
                                <span data-sidebar-label="">"API"</span>
                            </SidebarMenuItem>
                        </SidebarGroupContent>
                    </SidebarGroup>

                    <SidebarMenuItem href="/tasks">
                        <span data-sidebar-icon="">"✓"</span>
                        <span data-sidebar-label="">"Tasks"</span>
                    </SidebarMenuItem>

                    <SidebarSeparator />

                    <SidebarGroupLabel>"Settings"</SidebarGroupLabel>

                    <SidebarGroup>
                        <SidebarGroupTrigger>
                            <span data-sidebar-icon="">"⚙"</span>
                            <span data-sidebar-label="">"Preferences"</span>
                            <span data-rs-sidebar-group-chevron="">"▼"</span>
                        </SidebarGroupTrigger>
                        <SidebarGroupContent>
                            <SidebarMenuItem href="/preferences/general">
                                <span data-sidebar-label="">"General"</span>
                            </SidebarMenuItem>
                            <SidebarMenuItem href="/preferences/theme">
                                <span data-sidebar-label="">"Theme"</span>
                            </SidebarMenuItem>
                            <SidebarMenuItem href="/preferences/notifications">
                                <span data-sidebar-label="">"Notifications"</span>
                            </SidebarMenuItem>
                        </SidebarGroupContent>
                    </SidebarGroup>

                    <SidebarMenuItem href="/profile">
                        <span data-sidebar-icon="">"👤"</span>
                        <span data-sidebar-label="">"Profile"</span>
                    </SidebarMenuItem>
                </SidebarMenu>
            </SidebarContent>

            <SidebarFooter>
                <span data-sidebar-label="">"© 2026 CanonRS"</span>
            </SidebarFooter>
        </Sidebar>
    }
}
