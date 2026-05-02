use leptos::prelude::*;
use canonrs_core::{
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarGroupLabel, SidebarSeparator,
    SidebarTriggerPrimitive,
};
use crate::ui::avatar::{Avatar, AvatarImage, AvatarFallback, AvatarSize, AvatarStatus};
use canonrs_core::{Accordion, AccordionItem, AccordionTrigger, AccordionContent};
use canonrs_core::AccordionSelection;

#[component]
pub fn SidebarPinnable(
    #[prop(default = false)] default_collapsed: bool,
) -> impl IntoView {
    view! {
        <div style="position: relative;">
            <Sidebar state=if default_collapsed { canonrs_core::meta::VisibilityState::Closed } else { canonrs_core::meta::VisibilityState::Open }>
                <div style="position: absolute; top: 0.5rem; right: 0.5rem; z-index: 10; display: flex; gap: 0.5rem;">
                    <button
                        type="button"
                        data-rs-sidebar-pin-toggle=""
                        style="padding: 0.5rem; background: var(--theme-surface-bg); border: 1px solid var(--theme-surface-border); border-radius: var(--radius-sm); cursor: pointer; font-size: 1rem;"
                        title="Pin sidebar"
                    >
                        "📍"
                    </button>

                    <SidebarTriggerPrimitive style="padding: 0.5rem; background: var(--theme-surface-bg); border: 1px solid var(--theme-surface-border); border-radius: var(--radius-sm); cursor: pointer; font-size: 1rem;">
                        "⇔"
                    </SidebarTriggerPrimitive>
                </div>

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

                        <Accordion selection=AccordionSelection::Single collapsible="true".to_string()>
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

                        <SidebarMenuItem href="/tasks".to_string()>
                            <span data-sidebar-icon>"✓"</span>
                            <span data-sidebar-label>"Tasks"</span>
                        </SidebarMenuItem>

                        <SidebarSeparator />

                        <SidebarGroupLabel>"Settings"</SidebarGroupLabel>
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
