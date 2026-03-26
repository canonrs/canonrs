use leptos::prelude::*;
use canonrs_core::{
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarGroupLabel, SidebarSeparator,
};
use crate::ui::avatar::{Avatar, AvatarImage, AvatarFallback, AvatarSize, AvatarStatus};
use canonrs_core::{Accordion, AccordionItem, AccordionTrigger, AccordionContent};
use canonrs_core::{TooltipProvider, Tooltip, TooltipTrigger, TooltipContent};
use canonrs_core::AccordionSelection;

#[component]
pub fn SidebarRailMode() -> impl IntoView {
    view! {
        <div style="position: relative;">
            <TooltipProvider>
                <Sidebar state=canonrs_core::meta::VisibilityState::Closed variant=canonrs_core::primitives::SidebarVariant::Rail>
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

                            <Tooltip>
                                <TooltipTrigger>
                                    <SidebarMenuItem href="/dashboard".to_string() active=canonrs_core::meta::ActivityState::Active>
                                        <span data-sidebar-icon>"📊"</span>
                                        <span data-sidebar-label>"Dashboard"</span>
                                    </SidebarMenuItem>
                                </TooltipTrigger>
                                <TooltipContent>"Dashboard"</TooltipContent>
                            </Tooltip>

                            <Accordion selection=AccordionSelection::Single collapsible=true>
                                <AccordionItem>
                                    <AccordionTrigger>
                                        <span data-sidebar-icon>"📁"</span>
                                        <span data-sidebar-label>"Projects"</span>
                                    </AccordionTrigger>
                                    <AccordionContent>
                                        <SidebarMenuItem href="/projects/web".to_string()>
                                            <span data-sidebar-label>"Web App"</span>
                                        </SidebarMenuItem>
                                        <SidebarMenuItem href="/projects/mobile".to_string()>
                                            <span data-sidebar-label>"Mobile App"</span>
                                        </SidebarMenuItem>
                                    </AccordionContent>
                                </AccordionItem>
                            </Accordion>

                            <Tooltip>
                                <TooltipTrigger>
                                    <SidebarMenuItem href="/tasks".to_string()>
                                        <span data-sidebar-icon>"✓"</span>
                                        <span data-sidebar-label>"Tasks"</span>
                                    </SidebarMenuItem>
                                </TooltipTrigger>
                                <TooltipContent>"Tasks"</TooltipContent>
                            </Tooltip>

                            <SidebarSeparator />

                            <SidebarGroupLabel>"Settings"</SidebarGroupLabel>

                            <Tooltip>
                                <TooltipTrigger>
                                    <SidebarMenuItem href="/profile".to_string()>
                                        <span data-sidebar-icon>"👤"</span>
                                        <span data-sidebar-label>"Profile"</span>
                                    </SidebarMenuItem>
                                </TooltipTrigger>
                                <TooltipContent>"Profile"</TooltipContent>
                            </Tooltip>
                        </SidebarMenu>
                    </SidebarContent>

                    <SidebarFooter>
                        <div data-sidebar-label style="padding: 0.75rem; font-size: 0.75rem; color: var(--theme-surface-fg-muted);">
                            "© 2026 CanonRS"
                        </div>
                    </SidebarFooter>
                </Sidebar>
            </TooltipProvider>
        </div>
    }
}
