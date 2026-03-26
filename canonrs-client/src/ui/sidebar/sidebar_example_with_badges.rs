use leptos::prelude::*;
use canonrs_core::{
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarGroupLabel, SidebarSeparator,
    SidebarTriggerPrimitive,
};
use crate::ui::avatar::{Avatar, AvatarImage, AvatarFallback, AvatarSize, AvatarStatus};
use canonrs_core::{Badge, BadgeVariant};

#[component]
pub fn SidebarWithBadges(
    #[prop(default = false)] default_collapsed: bool,
) -> impl IntoView {
    view! {
        <div style="position: relative;">
            <Sidebar state=if default_collapsed { canonrs_core::meta::VisibilityState::Closed } else { canonrs_core::meta::VisibilityState::Open }>
                <SidebarTriggerPrimitive style="position: absolute; top: 0.5rem; right: 0.5rem; z-index: 10; padding: 0.5rem; background: var(--theme-surface-bg); border: 1px solid var(--theme-surface-border); border-radius: var(--radius-sm); cursor: pointer; font-size: 1rem;">
                    "⇔"
                </SidebarTriggerPrimitive>

                <SidebarHeader>
                    <div style="display: flex; align-items: center; gap: 0.75rem; padding: 1rem;">
                        <Avatar size=AvatarSize::Md status=AvatarStatus::Online>
                            <AvatarImage
                                src="https://i.pravatar.cc/150?img=10".to_string()
                                alt="User".to_string()
                            />
                            <AvatarFallback>"JD"</AvatarFallback>
                        </Avatar>
                        <div data-sidebar-label style="flex: 1; min-width: 0;">
                            <div style="font-weight: 600; font-size: 0.875rem;">"John Doe"</div>
                            <div style="font-size: 0.75rem; color: var(--theme-surface-fg-muted);">"john@canonrs.dev"</div>
                        </div>
                    </div>
                </SidebarHeader>

                <SidebarContent>
                    <SidebarMenu>
                        <SidebarGroupLabel>"Navigation"</SidebarGroupLabel>

                        <SidebarMenuItem href="/dashboard".to_string() active=canonrs_core::meta::ActivityState::Active>
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

                        <SidebarMenuItem href="/messages".to_string()>
                            <span data-sidebar-icon>"💬"</span>
                            <span data-sidebar-label>"Messages"</span>
                            <Badge variant=BadgeVariant::Warning>"3"</Badge>
                        </SidebarMenuItem>

                        <SidebarMenuItem href="/notifications".to_string()>
                            <span data-sidebar-icon>"🔔"</span>
                            <span data-sidebar-label>"Notifications"</span>
                            <Badge variant=BadgeVariant::Success>"New"</Badge>
                        </SidebarMenuItem>

                        <SidebarSeparator />

                        <SidebarGroupLabel>"Settings"</SidebarGroupLabel>

                        <SidebarMenuItem href="/profile".to_string()>
                            <span data-sidebar-icon>"👤"</span>
                            <span data-sidebar-label>"Profile"</span>
                        </SidebarMenuItem>

                        <SidebarMenuItem href="/preferences".to_string()>
                            <span data-sidebar-icon>"⚙"</span>
                            <span data-sidebar-label>"Preferences"</span>
                        </SidebarMenuItem>
                    </SidebarMenu>
                </SidebarContent>

                <SidebarFooter>
                    <div data-sidebar-label style="padding: 0.75rem; font-size: 0.75rem; color: var(--theme-surface-fg-muted);">
                        "© 2026 CanonRS"
                    </div>
                </SidebarFooter>
            </Sidebar>
        </div>
    }
}
