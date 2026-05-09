//! Preview: Sidebar Interactive
//! Fiel ao SidebarInteractive do canonrs-client

use leptos::prelude::*;
use super::sidebar_boundary::{
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarMenuGroup,
    SidebarGroupLabel, SidebarSeparator, SidebarTrigger,
};
use canonrs_core::meta::{VisibilityState, ActivityState};
use canonrs_core::primitives::SidebarVariant;

#[component]
pub fn SidebarPreviewInteractive() -> impl IntoView {
    view! {
        <div style="position: relative; height: 480px; overflow: hidden;">
            <Sidebar state=VisibilityState::Open>
                <SidebarTrigger class="sidebar-trigger-abs">"⇔"</SidebarTrigger>

                <SidebarHeader>
                    <div style="display: flex; align-items: center; gap: 0.75rem; padding: 1rem;">
                        <div style="width: 2rem; height: 2rem; border-radius: 50%; background: var(--theme-primary); display: flex; align-items: center; justify-content: center; color: white; font-weight: 600; font-size: 0.75rem;">"JD"</div>
                        <div data-sidebar-label="" style="flex: 1; min-width: 0;">
                            <div style="font-weight: 600; font-size: 0.875rem;">"John Doe"</div>
                            <div style="font-size: 0.75rem; color: var(--theme-surface-fg-muted);">"john@canonrs.dev"</div>
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
                        <SidebarMenuItem href="/projects">
                            <span data-sidebar-icon="">"📁"</span>
                            <span data-sidebar-label="">"Projects"</span>
                        </SidebarMenuItem>
                        <SidebarMenuItem href="/tasks">
                            <span data-sidebar-icon="">"✓"</span>
                            <span data-sidebar-label="">"Tasks"</span>
                        </SidebarMenuItem>

                        <SidebarSeparator />

                        <SidebarGroupLabel>"Settings"</SidebarGroupLabel>
                        <SidebarMenuItem href="/profile">
                            <span data-sidebar-icon="">"👤"</span>
                            <span data-sidebar-label="">"Profile"</span>
                        </SidebarMenuItem>
                        <SidebarMenuItem href="/preferences">
                            <span data-sidebar-icon="">"⚙"</span>
                            <span data-sidebar-label="">"Preferences"</span>
                        </SidebarMenuItem>
                    </SidebarMenu>
                </SidebarContent>

                <SidebarFooter>
                    <div data-sidebar-label="" style="padding: 0.75rem; font-size: 0.75rem; color: var(--theme-surface-fg-muted);">
                        "© 2026 CanonRS"
                    </div>
                </SidebarFooter>
            </Sidebar>
        </div>
    }
}
