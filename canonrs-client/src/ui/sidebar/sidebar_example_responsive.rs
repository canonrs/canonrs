use leptos::prelude::*;
use canonrs_core::{
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarGroupLabel, SidebarSeparator,
    SidebarTriggerPrimitive,
};
use crate::ui::avatar::{Avatar, AvatarImage, AvatarFallback, AvatarSize, AvatarStatus};
use canonrs_core::{Badge, BadgeVariant};

#[component]
pub fn SidebarResponsive() -> impl IntoView {
    view! {
        <div data-rs-sidebar-responsive="" class="responsive-sidebar-container">

            <button
                type="button"
                data-rs-sidebar-toggle="1"
                class="hamburger-btn"
            >
                "☰"
            </button>

            <div data-rs-sidebar-overlay="" class="sidebar-overlay" />

            <div class="sidebar-wrapper">
                <Sidebar collapsed=true>
                    <SidebarTriggerPrimitive class="close-btn">
                        "✕"
                    </SidebarTriggerPrimitive>

                    <SidebarHeader>
                        <div style="display: flex; align-items: center; gap: 0.75rem; padding: 1rem;">
                            <Avatar size=AvatarSize::Md status=AvatarStatus::Online>
                                <AvatarImage src="https://i.pravatar.cc/150?img=10".to_string() alt="User".to_string() />
                                <AvatarFallback>"JD"</AvatarFallback>
                            </Avatar>
                            <div data-sidebar-label="" style="flex: 1;">
                                <div style="font-weight: 600; font-size: 0.875rem;">"John Doe"</div>
                                <div style="font-size: 0.75rem; color: var(--theme-surface-fg-muted);">"john@canonrs.dev"</div>
                            </div>
                        </div>
                    </SidebarHeader>

                    <SidebarContent>
                        <SidebarMenu>
                            <SidebarGroupLabel>"Navigation"</SidebarGroupLabel>
                            <SidebarMenuItem href="/dashboard".to_string() active=true>
                                <span data-sidebar-icon>"📊"</span>
                                <span data-sidebar-label>"Dashboard"</span>
                            </SidebarMenuItem>
                            <SidebarMenuItem href="/projects".to_string()>
                                <span data-sidebar-icon>"📁"</span>
                                <span data-sidebar-label>"Projects"</span>
                                <Badge variant=BadgeVariant::Primary>"12"</Badge>
                            </SidebarMenuItem>
                            <SidebarMenuItem href="/tasks".to_string()>
                                <span data-sidebar-icon>"✓"</span>
                                <span data-sidebar-label>"Tasks"</span>
                                <Badge variant=BadgeVariant::Destructive>"5"</Badge>
                            </SidebarMenuItem>
                            <SidebarSeparator />
                            <SidebarGroupLabel>"Settings"</SidebarGroupLabel>
                            <SidebarMenuItem href="/profile".to_string()>
                                <span data-sidebar-icon>"👤"</span>
                                <span data-sidebar-label>"Profile"</span>
                            </SidebarMenuItem>
                        </SidebarMenu>
                    </SidebarContent>

                    <SidebarFooter>
                        <div data-sidebar-label="" style="padding: 0.75rem; font-size: 0.75rem;">
                            "© 2026 CanonRS"
                        </div>
                    </SidebarFooter>
                </Sidebar>
            </div>

        </div>
    }
}
