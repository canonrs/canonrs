use leptos::prelude::*;
use leptos::web_sys;
use canonrs_ui::ui::sidebar::{
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarGroupLabel, SidebarSeparator
};
use canonrs_ui::ui::avatar::{Avatar, AvatarImage, AvatarFallback, AvatarSize, AvatarStatus};
use canonrs_ui::ui::accordion::{Accordion, AccordionItem, AccordionTrigger, AccordionContent};
use canonrs_ui::ui::tooltip::{TooltipProvider, Tooltip, TooltipTrigger, TooltipContent};
use canonrs_ui::primitives::AccordionSelection;

#[component]
pub fn SidebarRailMode() -> impl IntoView {
    let collapsed = RwSignal::new(true); // Começa collapsed
    
    // Handlers de hover
    let handle_mouse_enter = move |_: web_sys::MouseEvent| {
        collapsed.set(false);
    };
    
    let handle_mouse_leave = move |_: web_sys::MouseEvent| {
        collapsed.set(true);
    };
    
    view! {
        <div 
            style="position: relative;"
            on:mouseenter=handle_mouse_enter
            on:mouseleave=handle_mouse_leave
        >
            <TooltipProvider>
                <Sidebar collapsed=collapsed>
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
                            
                            <Tooltip id="tooltip-dashboard-rail".to_string()>
                                <TooltipTrigger target_tooltip_id="tooltip-dashboard-rail".to_string()>
                                    <SidebarMenuItem href="/dashboard".to_string() active=true>
                                        <span data-sidebar-icon>"📊"</span>
                                        <span data-sidebar-label>"Dashboard"</span>
                                    </SidebarMenuItem>
                                </TooltipTrigger>
                                <TooltipContent id="tooltip-dashboard-rail-content".to_string()>
                                    "Dashboard"
                                </TooltipContent>
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
                            
                            <Tooltip id="tooltip-tasks-rail".to_string()>
                                <TooltipTrigger target_tooltip_id="tooltip-tasks-rail".to_string()>
                                    <SidebarMenuItem href="/tasks".to_string()>
                                        <span data-sidebar-icon>"✓"</span>
                                        <span data-sidebar-label>"Tasks"</span>
                                    </SidebarMenuItem>
                                </TooltipTrigger>
                                <TooltipContent id="tooltip-tasks-rail-content".to_string()>
                                    "Tasks"
                                </TooltipContent>
                            </Tooltip>
                            
                            <SidebarSeparator />
                            
                            <SidebarGroupLabel>"Settings"</SidebarGroupLabel>
                            
                            <Tooltip id="tooltip-profile-rail".to_string()>
                                <TooltipTrigger target_tooltip_id="tooltip-profile-rail".to_string()>
                                    <SidebarMenuItem href="/profile".to_string()>
                                        <span data-sidebar-icon>"👤"</span>
                                        <span data-sidebar-label>"Profile"</span>
                                    </SidebarMenuItem>
                                </TooltipTrigger>
                                <TooltipContent id="tooltip-profile-rail-content".to_string()>
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
            </TooltipProvider>
        </div>
    }
}
