use leptos::prelude::*;
use super::sidebar_boundary::{
    Sidebar, SidebarHeader, SidebarContent,
    SidebarMenu, SidebarMenuItem, SidebarMenuGroup,
    SidebarGroupLabel,
};
use canonrs_core::meta::ActivityState;

#[component]
pub fn SidebarShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Sidebar>
                    <SidebarHeader>"CanonRS"</SidebarHeader>
                    <SidebarContent>
                        <SidebarMenuGroup label="Main">
                            <SidebarGroupLabel>"Main"</SidebarGroupLabel>
                            <SidebarMenu>
                                <SidebarMenuItem href="#" active=ActivityState::Active>"Dashboard"</SidebarMenuItem>
                                <SidebarMenuItem href="#">"Components"</SidebarMenuItem>
                                <SidebarMenuItem href="#">"Tokens"</SidebarMenuItem>
                            </SidebarMenu>
                        </SidebarMenuGroup>
                        <SidebarMenuGroup label="Settings">
                            <SidebarGroupLabel>"Settings"</SidebarGroupLabel>
                            <SidebarMenu>
                                <SidebarMenuItem href="#">"Preferences"</SidebarMenuItem>
                                <SidebarMenuItem href="#">"Team"</SidebarMenuItem>
                            </SidebarMenu>
                        </SidebarMenuGroup>
                    </SidebarContent>
                </Sidebar>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Navigation state and structure governed by DOM — SSR-safe, hydration-safe."
            </p>
        </div>
    }
}
