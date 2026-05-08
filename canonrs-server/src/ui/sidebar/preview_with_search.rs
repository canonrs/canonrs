//! Preview: Sidebar com Search
//! Demonstra busca integrada via Command no header da sidebar

use leptos::prelude::*;
use super::sidebar_boundary::{
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarMenuGroup,
    SidebarGroupLabel, SidebarTrigger,
};
use canonrs_core::meta::{VisibilityState, ActivityState};
use crate::ui::command::command_boundary::{Command, CommandItem};

#[component]
pub fn SidebarPreviewWithSearch() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-anchor="" data-rs-showcase-preview-label="With Search">
            <div style="position: relative; height: 480px; overflow: hidden;">
                <Sidebar state=VisibilityState::Open>
                    <SidebarTrigger class="sidebar-trigger-abs">"☰"</SidebarTrigger>

                    <SidebarHeader>
                        <div class="sidebar-user-block">
                            <span class="sidebar-user-name">"John Doe"</span>
                            <span class="sidebar-user-email">"john@canonrs.dev"</span>
                        </div>
                        <div class="sidebar-search-block">
                            <Command placeholder="Search menu...".to_string()>
                                <CommandItem value="dashboard">"Dashboard"</CommandItem>
                                <CommandItem value="tasks">"Tasks"</CommandItem>
                                <CommandItem value="projects">"Projects"</CommandItem>
                            </Command>
                        </div>
                    </SidebarHeader>

                    <SidebarContent>
                        <SidebarMenuGroup label="Quick Access">
                            <SidebarGroupLabel>"Quick Access"</SidebarGroupLabel>
                            <SidebarMenu>
                                <SidebarMenuItem href="/dashboard" active=ActivityState::Active>
                                    "Dashboard"
                                </SidebarMenuItem>
                                <SidebarMenuItem href="/tasks">
                                    "Tasks"
                                </SidebarMenuItem>
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
