use leptos::prelude::*;
use canonrs_core::{
    SidebarTriggerPrimitive,
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarGroupLabel
};
use crate::ui::avatar::{Avatar, AvatarImage, AvatarFallback, AvatarSize, AvatarStatus};
use canonrs_core::{Accordion, AccordionItem, AccordionTrigger, AccordionContent};
use canonrs_core::AccordionSelection;

#[component]
pub fn SidebarMultiLevel(
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

                        <Accordion selection=AccordionSelection::Single collapsible=true>
                            <AccordionItem>
                                <AccordionTrigger>
                                    <span data-sidebar-icon>"📁"</span>
                                    <span data-sidebar-label>"Projects"</span>
                                </AccordionTrigger>
                                <AccordionContent>
                                    <div style="padding-left: var(--sidebar-inset-padding-left);">
                                        <Accordion selection=AccordionSelection::Single collapsible=true>
                                            <AccordionItem>
                                                <AccordionTrigger>
                                                    <span data-sidebar-label>"Frontend"</span>
                                                </AccordionTrigger>
                                                <AccordionContent>
                                                    <div style="padding-left: var(--sidebar-inset-padding-left);">
                                                        <SidebarMenuItem href="/projects/frontend/components".to_string()>
                                                            <span data-sidebar-label>"Components"</span>
                                                        </SidebarMenuItem>
                                                        <SidebarMenuItem href="/projects/frontend/pages".to_string()>
                                                            <span data-sidebar-label>"Pages"</span>
                                                        </SidebarMenuItem>
                                                        <SidebarMenuItem href="/projects/frontend/hooks".to_string()>
                                                            <span data-sidebar-label>"Hooks"</span>
                                                        </SidebarMenuItem>
                                                    </div>
                                                </AccordionContent>
                                            </AccordionItem>
                                        </Accordion>

                                        <Accordion selection=AccordionSelection::Single collapsible=true>
                                            <AccordionItem>
                                                <AccordionTrigger>
                                                    <span data-sidebar-label>"Backend"</span>
                                                </AccordionTrigger>
                                                <AccordionContent>
                                                    <div style="padding-left: var(--sidebar-inset-padding-left);">
                                                        <SidebarMenuItem href="/projects/backend/api".to_string()>
                                                            <span data-sidebar-label>"API"</span>
                                                        </SidebarMenuItem>
                                                        <SidebarMenuItem href="/projects/backend/database".to_string()>
                                                            <span data-sidebar-label>"Database"</span>
                                                        </SidebarMenuItem>
                                                        <SidebarMenuItem href="/projects/backend/auth".to_string()>
                                                            <span data-sidebar-label>"Auth"</span>
                                                        </SidebarMenuItem>
                                                    </div>
                                                </AccordionContent>
                                            </AccordionItem>
                                        </Accordion>

                                        <SidebarMenuItem href="/projects/docs".to_string()>
                                            <span data-sidebar-label>"Documentation"</span>
                                        </SidebarMenuItem>
                                    </div>
                                </AccordionContent>
                            </AccordionItem>
                        </Accordion>

                        <SidebarMenuItem href="/tasks".to_string()>
                            <span data-sidebar-icon>"✓"</span>
                            <span data-sidebar-label>"Tasks"</span>
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
