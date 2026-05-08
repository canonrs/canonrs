//! Preview: Sidebar com Accordion
//! Demonstra navegação hierárquica com seções colapsáveis

use leptos::prelude::*;
use super::sidebar_boundary::{
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarMenuGroup,
    SidebarGroupLabel, SidebarSeparator, SidebarTrigger, SidebarVariant,
};
use canonrs_core::meta::{VisibilityState, ActivityState};
use crate::ui::accordion::accordion_boundary::{
    Accordion, AccordionItem, AccordionTrigger, AccordionContent, AccordionSelection,
};

#[component]
pub fn SidebarPreviewWithAccordion() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-anchor="" data-rs-showcase-preview-label="With Accordion">
            <div style="position: relative; height: 480px; overflow: hidden;">
                <Sidebar state=VisibilityState::Open>
                    <SidebarTrigger class="sidebar-trigger-abs">"☰"</SidebarTrigger>

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

                                <Accordion selection=AccordionSelection::Single collapsible="true".to_string()>
                                    <AccordionItem>
                                        <AccordionTrigger>"Projects"</AccordionTrigger>
                                        <AccordionContent>
                                            <SidebarMenuItem href="/projects/web">"Web App"</SidebarMenuItem>
                                            <SidebarMenuItem href="/projects/mobile">"Mobile App"</SidebarMenuItem>
                                            <SidebarMenuItem href="/projects/api">"API"</SidebarMenuItem>
                                        </AccordionContent>
                                    </AccordionItem>
                                </Accordion>

                                <SidebarMenuItem href="/tasks">"Tasks"</SidebarMenuItem>
                            </SidebarMenu>
                        </SidebarMenuGroup>

                        <SidebarSeparator />

                        <SidebarMenuGroup label="Settings">
                            <SidebarGroupLabel>"Settings"</SidebarGroupLabel>
                            <SidebarMenu>
                                <Accordion selection=AccordionSelection::Single collapsible="true".to_string()>
                                    <AccordionItem>
                                        <AccordionTrigger>"Preferences"</AccordionTrigger>
                                        <AccordionContent>
                                            <SidebarMenuItem href="/preferences/general">"General"</SidebarMenuItem>
                                            <SidebarMenuItem href="/preferences/theme">"Theme"</SidebarMenuItem>
                                            <SidebarMenuItem href="/preferences/notifications">"Notifications"</SidebarMenuItem>
                                        </AccordionContent>
                                    </AccordionItem>
                                </Accordion>
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
