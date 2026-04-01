use leptos::prelude::*;
use super::sidebar_ui::{
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarMenuGroup,
    SidebarSeparator,
};
use canonrs_core::meta::{VisibilityState, ActivityState};

#[component]
pub fn SidebarShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Sidebar state=VisibilityState::Open>
                    <SidebarHeader>
                        <span style="font-weight:600;">"CanonRS"</span>
                    </SidebarHeader>
                    <SidebarContent>
                        <SidebarMenu>
                            <SidebarMenuGroup label="Main">
                                <SidebarMenuItem href="#".to_string() active=ActivityState::Active>"Dashboard"</SidebarMenuItem>
                                <SidebarMenuItem href="#".to_string()>"Components"</SidebarMenuItem>
                                <SidebarMenuItem href="#".to_string()>"Tokens"</SidebarMenuItem>
                            </SidebarMenuGroup>
                            <SidebarSeparator />
                            <SidebarMenuGroup label="Settings">
                                <SidebarMenuItem href="#".to_string()>"Preferences"</SidebarMenuItem>
                                <SidebarMenuItem href="#".to_string()>"Team"</SidebarMenuItem>
                            </SidebarMenuGroup>
                        </SidebarMenu>
                    </SidebarContent>
                    <SidebarFooter>
                        <span style="font-size:var(--font-size-xs);opacity:0.5;">"v0.1.0"</span>
                    </SidebarFooter>
                </Sidebar>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Navigation state and structure enforced at component level."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Collapsed"</span>
                <div data-rs-showcase-preview-row="">
                    <Sidebar state=VisibilityState::Closed>
                        <SidebarContent>
                            <SidebarMenu>
                                <SidebarMenuItem href="#".to_string()>"Home"</SidebarMenuItem>
                                <SidebarMenuItem href="#".to_string()>"About"</SidebarMenuItem>
                            </SidebarMenu>
                        </SidebarContent>
                    </Sidebar>
                </div>
            </div>
        </div>
    }
}
