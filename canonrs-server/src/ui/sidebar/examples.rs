use leptos::prelude::*;
use super::{
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarMenuGroup, 
    SidebarSeparator, SidebarGroupLabel
};
use crate::ui::avatar::{Avatar, AvatarImage, AvatarFallback, AvatarSize, AvatarStatus};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Sidebar>
            <SidebarHeader>
                <div style="display: flex; align-items: center; gap: 0.75rem; padding: 1rem;">
                    <Avatar size=AvatarSize::Md status=AvatarStatus::Online>
                        <AvatarImage 
                            src="https://i.pravatar.cc/150?img=10".to_string()
                            alt="User avatar".to_string()
                        />
                        <AvatarFallback>"JD"</AvatarFallback>
                    </Avatar>
                    <div style="flex: 1; min-width: 0;">
                        <div style="font-weight: 600; font-size: 0.875rem; truncate: true;">"John Doe"</div>
                        <div style="font-size: 0.75rem; color: var(--theme-surface-fg-muted); truncate: true;">"john@canonrs.dev"</div>
                    </div>
                </div>
            </SidebarHeader>

            <SidebarContent>
                <SidebarMenu>
                    <SidebarGroupLabel>"Navigation"</SidebarGroupLabel>
                    <SidebarMenuItem href="/dashboard".to_string() active=true>"Dashboard"</SidebarMenuItem>
                    <SidebarMenuItem href="/projects".to_string()>"Projects"</SidebarMenuItem>
                    <SidebarMenuItem href="/tasks".to_string()>"Tasks"</SidebarMenuItem>
                    
                    <SidebarSeparator />
                    
                    <SidebarGroupLabel>"Settings"</SidebarGroupLabel>
                    <SidebarMenuItem href="/profile".to_string()>"Profile"</SidebarMenuItem>
                    <SidebarMenuItem href="/preferences".to_string()>"Preferences"</SidebarMenuItem>
                </SidebarMenu>
            </SidebarContent>

            <SidebarFooter>
                <div style="padding: 0.75rem; font-size: 0.75rem; color: var(--theme-surface-fg-muted);">
                    "© 2026 CanonRS"
                </div>
            </SidebarFooter>
        </Sidebar>
    }
}
