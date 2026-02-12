use leptos::prelude::*;
use canonrs_ui::ui::sidebar::{
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarGroupLabel, SidebarSeparator
};
use canonrs_ui::ui::avatar::{Avatar, AvatarImage, AvatarFallback, AvatarSize, AvatarStatus};
use canonrs_ui::ui::accordion::{Accordion, AccordionItem, AccordionTrigger, AccordionContent};
use canonrs_ui::primitives::AccordionSelection;

#[component]
pub fn SidebarWithAccordion(
    #[prop(default = false)] default_collapsed: bool,
) -> impl IntoView {
    let collapsed = RwSignal::new(default_collapsed);
    
    view! {
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
                        <SidebarMenuItem href="/dashboard".to_string() active=true>
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
                                    <SidebarMenuItem href="/projects/web".to_string()>
                                        <span data-sidebar-label>"Web App"</span>
                                    </SidebarMenuItem>
                                    <SidebarMenuItem href="/projects/mobile".to_string()>
                                        <span data-sidebar-label>"Mobile App"</span>
                                    </SidebarMenuItem>
                                    <SidebarMenuItem href="/projects/api".to_string()>
                                        <span data-sidebar-label>"API"</span>
                                    </SidebarMenuItem>
                                </AccordionContent>
                            </AccordionItem>
                        </Accordion>
                        
                        <SidebarMenuItem href="/tasks".to_string()>
                            <span data-sidebar-icon>"✓"</span>
                            <span data-sidebar-label>"Tasks"</span>
                        </SidebarMenuItem>
                        
                        <SidebarSeparator />
                        
                        <SidebarGroupLabel>"Settings"</SidebarGroupLabel>
                        
                        <Accordion selection=AccordionSelection::Single collapsible=true>
                            <AccordionItem>
                                <AccordionTrigger>
                                    <span data-sidebar-icon>"⚙"</span>
                                    <span data-sidebar-label>"Preferences"</span>
                                </AccordionTrigger>
                                <AccordionContent>
                                    <SidebarMenuItem href="/preferences/general".to_string()>
                                        <span data-sidebar-label>"General"</span>
                                    </SidebarMenuItem>
                                    <SidebarMenuItem href="/preferences/theme".to_string()>
                                        <span data-sidebar-label>"Theme"</span>
                                    </SidebarMenuItem>
                                    <SidebarMenuItem href="/preferences/notifications".to_string()>
                                        <span data-sidebar-label>"Notifications"</span>
                                    </SidebarMenuItem>
                                </AccordionContent>
                            </AccordionItem>
                        </Accordion>
                        
                        <SidebarMenuItem href="/profile".to_string()>
                            <span data-sidebar-icon>"👤"</span>
                            <span data-sidebar-label>"Profile"</span>
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
