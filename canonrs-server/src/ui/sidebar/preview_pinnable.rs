//! Preview: Sidebar Pinnable
//! Demonstra sidebar com pin toggle — fixada ou recolhível

use leptos::prelude::*;
use super::sidebar_boundary::{
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarMenuGroup,
    SidebarGroupLabel, SidebarSeparator, SidebarTrigger,
};
use canonrs_core::meta::{VisibilityState, ActivityState};

#[component]
pub fn SidebarPreviewPinnable() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-anchor="" data-rs-showcase-preview-label="Pinnable">
            <div style="position: relative; height: 480px; overflow: hidden;">
                <Sidebar state=VisibilityState::Open>
                    <div class="sidebar-controls-abs">
                        <button
                            type="button"
                            data-rs-sidebar-pin-toggle=""
                            class="sidebar-pin-btn"
                            title="Pin sidebar"
                        >
                            "📍"
                        </button>
                        <SidebarTrigger class="sidebar-trigger-btn">"⇔"</SidebarTrigger>
                    </div>

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
                                <SidebarMenuItem href="/projects">"Projects"</SidebarMenuItem>
                                <SidebarMenuItem href="/tasks">"Tasks"</SidebarMenuItem>
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
    }
}
