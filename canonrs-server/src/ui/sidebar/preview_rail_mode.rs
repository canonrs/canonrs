//! Preview: Sidebar Rail Mode
//! Sidebar colapsada por padrão — expande ao hover via engine nav

use leptos::prelude::*;
use super::sidebar_boundary::{
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarMenuGroup,
    SidebarGroupLabel, SidebarSeparator, SidebarVariant,
};
use canonrs_core::meta::{VisibilityState, ActivityState};

#[component]
pub fn SidebarPreviewRailMode() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-anchor="" data-rs-showcase-preview-label="Rail Mode">
            <div style="position: relative; height: 480px; overflow: hidden;">
                <Sidebar state=VisibilityState::Closed variant=SidebarVariant::Rail>

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
                                    "📊 Dashboard"
                                </SidebarMenuItem>
                                <SidebarMenuItem href="/projects">"📁 Projects"</SidebarMenuItem>
                                <SidebarMenuItem href="/tasks">"✓ Tasks"</SidebarMenuItem>
                            </SidebarMenu>
                        </SidebarMenuGroup>

                        <SidebarSeparator />

                        <SidebarMenuGroup label="Settings">
                            <SidebarGroupLabel>"Settings"</SidebarGroupLabel>
                            <SidebarMenu>
                                <SidebarMenuItem href="/profile">"👤 Profile"</SidebarMenuItem>
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
