//! Preview: Sidebar Multi-Level
//! Demonstra navegação hierárquica com accordion aninhado (3 níveis)

use leptos::prelude::*;
use super::sidebar_boundary::{
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarMenuGroup,
    SidebarGroupLabel, SidebarTrigger,
};
use canonrs_core::meta::{VisibilityState, ActivityState};
use crate::ui::accordion::accordion_boundary::{
    Accordion, AccordionItem, AccordionTrigger, AccordionContent, AccordionSelection,
};

#[component]
pub fn SidebarPreviewMultiLevel() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-anchor="" data-rs-showcase-preview-label="Multi Level">
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
                                            <Accordion selection=AccordionSelection::Single collapsible="true".to_string()>
                                                <AccordionItem>
                                                    <AccordionTrigger>"Frontend"</AccordionTrigger>
                                                    <AccordionContent>
                                                        <SidebarMenuItem href="/projects/frontend/components">
                                                            "Components"
                                                        </SidebarMenuItem>
                                                        <SidebarMenuItem href="/projects/frontend/pages">
                                                            "Pages"
                                                        </SidebarMenuItem>
                                                        <SidebarMenuItem href="/projects/frontend/hooks">
                                                            "Hooks"
                                                        </SidebarMenuItem>
                                                    </AccordionContent>
                                                </AccordionItem>
                                            </Accordion>

                                            <Accordion selection=AccordionSelection::Single collapsible="true".to_string()>
                                                <AccordionItem>
                                                    <AccordionTrigger>"Backend"</AccordionTrigger>
                                                    <AccordionContent>
                                                        <SidebarMenuItem href="/projects/backend/api">
                                                            "API"
                                                        </SidebarMenuItem>
                                                        <SidebarMenuItem href="/projects/backend/database">
                                                            "Database"
                                                        </SidebarMenuItem>
                                                        <SidebarMenuItem href="/projects/backend/auth">
                                                            "Auth"
                                                        </SidebarMenuItem>
                                                    </AccordionContent>
                                                </AccordionItem>
                                            </Accordion>

                                            <SidebarMenuItem href="/projects/docs">
                                                "Documentation"
                                            </SidebarMenuItem>
                                        </AccordionContent>
                                    </AccordionItem>
                                </Accordion>

                                <SidebarMenuItem href="/tasks">"Tasks"</SidebarMenuItem>
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
