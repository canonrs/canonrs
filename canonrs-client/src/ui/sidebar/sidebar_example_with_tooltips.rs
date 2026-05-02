use leptos::prelude::*;
use canonrs_core::{
    SidebarTriggerPrimitive,
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarGroupLabel, SidebarSeparator
};
use crate::ui::avatar::{Avatar, AvatarImage, AvatarFallback, AvatarSize, AvatarStatus};
use canonrs_core::{Accordion, AccordionItem, AccordionTrigger, AccordionContent};
use canonrs_core::{TooltipProvider, Tooltip, TooltipTrigger, TooltipContent};
use canonrs_core::AccordionSelection;

#[component]
pub fn SidebarWithTooltips(
    #[prop(default = false)] default_collapsed: bool,
) -> impl IntoView {
    view! {
        <TooltipProvider>
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

                            <Tooltip>
                                <TooltipTrigger>
                                    <SidebarMenuItem href="/dashboard".to_string() active=canonrs_core::meta::ActivityState::Active>
                                        <span data-sidebar-icon>"📊"</span>
                                        <span data-sidebar-label>"Dashboard"</span>
                                    </SidebarMenuItem>
                                </TooltipTrigger>
                                <TooltipContent>
                                    "Dashboard"
                                </TooltipContent>
                            </Tooltip>

                            <Accordion selection=AccordionSelection::Single collapsible="true".to_string()>
                                <AccordionItem>
                                    <Tooltip>
                                        <TooltipTrigger>
                                            <AccordionTrigger>
                                                <span data-sidebar-icon>"📁"</span>
                                                <span data-sidebar-label>"Projects"</span>
                                            </AccordionTrigger>
                                        </TooltipTrigger>
                                        <TooltipContent>
                                            "Projects"
                                        </TooltipContent>
                                    </Tooltip>
                                    <AccordionContent>
                                        <Tooltip>
                                            <TooltipTrigger>
                                                <SidebarMenuItem href="/projects/web".to_string()>
                                                    <span data-sidebar-label>"Web App"</span>
                                                </SidebarMenuItem>
                                            </TooltipTrigger>
                                            <TooltipContent>
                                                "Web App"
                                            </TooltipContent>
                                        </Tooltip>
                                        <Tooltip>
                                            <TooltipTrigger>
                                                <SidebarMenuItem href="/projects/mobile".to_string()>
                                                    <span data-sidebar-label>"Mobile App"</span>
                                                </SidebarMenuItem>
                                            </TooltipTrigger>
                                            <TooltipContent>
                                                "Mobile App"
                                            </TooltipContent>
                                        </Tooltip>
                                        <Tooltip>
                                            <TooltipTrigger>
                                                <SidebarMenuItem href="/projects/api".to_string()>
                                                    <span data-sidebar-label>"API"</span>
                                                </SidebarMenuItem>
                                            </TooltipTrigger>
                                            <TooltipContent>
                                                "API"
                                            </TooltipContent>
                                        </Tooltip>
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
                                <TooltipContent>
                                    "Tasks"
                                </TooltipContent>
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
                                <TooltipContent>
                                    "Profile"
                                </TooltipContent>
                            </Tooltip>
                        </SidebarMenu>
                    </SidebarContent>

                    <SidebarFooter>
                        <div data-sidebar-label style="padding: 0.75rem; font-size: 0.75rem; color: var(--theme-surface-fg-muted);">
                            "© 2026 CanonRS"
                        </div>
                    </SidebarFooter>
                </Sidebar>
            </div>
        </TooltipProvider>
    }
}
