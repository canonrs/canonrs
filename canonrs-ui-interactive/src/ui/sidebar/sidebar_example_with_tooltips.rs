use leptos::prelude::*;
use canonrs_ui::ui::sidebar::{
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarGroupLabel, SidebarSeparator
};
use canonrs_ui::ui::avatar::{Avatar, AvatarImage, AvatarFallback, AvatarSize, AvatarStatus};
use canonrs_ui::ui::accordion::{Accordion, AccordionItem, AccordionTrigger, AccordionContent};
use canonrs_ui::ui::tooltip::{TooltipProvider, Tooltip, TooltipTrigger, TooltipContent};
use canonrs_ui::primitives::AccordionSelection;

#[component]
pub fn SidebarWithTooltips(
    #[prop(default = false)] default_collapsed: bool,
) -> impl IntoView {
    let collapsed = RwSignal::new(default_collapsed);
    
    view! {
        <TooltipProvider>
            <div style="position: relative;">
                <Sidebar collapsed=collapsed>
                    <button 
                        on:click=move |_| collapsed.update(|c| *c = !*c)
                        style="position: absolute; top: 0.5rem; right: 0.5rem; z-index: 10; padding: 0.5rem; background: var(--theme-surface-bg); border: 1px solid var(--theme-surface-border); border-radius: var(--radius-sm); cursor: pointer; font-size: 1rem;"
                    >
                        {move || if collapsed.get() { "→" } else { "←" }}
                    </button>

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
                            
                            <Tooltip id="tooltip-dashboard".to_string()>
                                <TooltipTrigger target_tooltip_id="tooltip-dashboard".to_string()>
                                    <SidebarMenuItem href="/dashboard".to_string() active=true>
                                        <span data-sidebar-icon>"📊"</span>
                                        <span data-sidebar-label>"Dashboard"</span>
                                    </SidebarMenuItem>
                                </TooltipTrigger>
                                <TooltipContent id="tooltip-dashboard-content".to_string()>
                                    "Dashboard"
                                </TooltipContent>
                            </Tooltip>
                            
                            <Accordion selection=AccordionSelection::Single collapsible=true>
                                <AccordionItem>
                                <Tooltip id="tooltip-projects".to_string()>
                                    <TooltipTrigger target_tooltip_id="tooltip-projects".to_string()>
                                    <AccordionTrigger>
                                        <span data-sidebar-icon>"📁"</span>
                                        <span data-sidebar-label>"Projects"</span>
                                    </AccordionTrigger>
                                    </TooltipTrigger>
                                    <TooltipContent id="tooltip-projects-content".to_string()>
                                        "Projects"
                                    </TooltipContent>
                                </Tooltip>
                                    <AccordionContent>
                                        <Tooltip id="tooltip-web".to_string()>
                                            <TooltipTrigger target_tooltip_id="tooltip-web".to_string()>
                                                <SidebarMenuItem href="/projects/web".to_string()>
                                                    <span data-sidebar-label>"Web App"</span>
                                                </SidebarMenuItem>
                                            </TooltipTrigger>
                                            <TooltipContent id="tooltip-web-content".to_string()>
                                                "Web App"
                                            </TooltipContent>
                                        </Tooltip>
                                        
                                        <Tooltip id="tooltip-mobile".to_string()>
                                            <TooltipTrigger target_tooltip_id="tooltip-mobile".to_string()>
                                                <SidebarMenuItem href="/projects/mobile".to_string()>
                                                    <span data-sidebar-label>"Mobile App"</span>
                                                </SidebarMenuItem>
                                            </TooltipTrigger>
                                            <TooltipContent id="tooltip-mobile-content".to_string()>
                                                "Mobile App"
                                            </TooltipContent>
                                        </Tooltip>
                                        
                                        <Tooltip id="tooltip-api".to_string()>
                                            <TooltipTrigger target_tooltip_id="tooltip-api".to_string()>
                                                <SidebarMenuItem href="/projects/api".to_string()>
                                                    <span data-sidebar-label>"API"</span>
                                                </SidebarMenuItem>
                                            </TooltipTrigger>
                                            <TooltipContent id="tooltip-api-content".to_string()>
                                                "API"
                                            </TooltipContent>
                                        </Tooltip>
                                    </AccordionContent>
                                </AccordionItem>
                            </Accordion>
                            
                            <Tooltip id="tooltip-tasks".to_string()>
                                <TooltipTrigger target_tooltip_id="tooltip-tasks".to_string()>
                                    <SidebarMenuItem href="/tasks".to_string()>
                                        <span data-sidebar-icon>"✓"</span>
                                        <span data-sidebar-label>"Tasks"</span>
                                    </SidebarMenuItem>
                                </TooltipTrigger>
                                <TooltipContent id="tooltip-tasks-content".to_string()>
                                    "Tasks"
                                </TooltipContent>
                            </Tooltip>
                            
                            <SidebarSeparator />
                            
                            <SidebarGroupLabel>"Settings"</SidebarGroupLabel>
                            
                            <Tooltip id="tooltip-profile".to_string()>
                                <TooltipTrigger target_tooltip_id="tooltip-profile".to_string()>
                                    <SidebarMenuItem href="/profile".to_string()>
                                        <span data-sidebar-icon>"👤"</span>
                                        <span data-sidebar-label>"Profile"</span>
                                    </SidebarMenuItem>
                                </TooltipTrigger>
                                <TooltipContent id="tooltip-profile-content".to_string()>
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
