use leptos::prelude::*;
use leptos::web_sys;
use canonrs_ui::ui::sidebar::{
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarGroupLabel, SidebarSeparator
};
use canonrs_ui::ui::avatar::{Avatar, AvatarImage, AvatarFallback, AvatarSize, AvatarStatus};
use canonrs_ui::ui::accordion::{Accordion, AccordionItem, AccordionTrigger, AccordionContent};
use canonrs_ui::primitives::AccordionSelection;

// LocalStorage helpers
fn get_pinned_state() -> bool {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(value)) = storage.get_item("sidebar-pinned") {
                return value == "true";
            }
        }
    }
    false
}

fn set_pinned_state(pinned: bool) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item("sidebar-pinned", if pinned { "true" } else { "false" });
        }
    }
}

#[component]
pub fn SidebarPinnable(
    #[prop(default = false)] default_collapsed: bool,
) -> impl IntoView {
    let pinned = RwSignal::new(get_pinned_state());
    let collapsed = RwSignal::new(if pinned.get() { false } else { default_collapsed });
    
    let toggle_pin = move |_: web_sys::MouseEvent| {
        pinned.update(|p| {
            *p = !*p;
            set_pinned_state(*p);
            if *p {
                collapsed.set(false); // Se pinned, sempre expandido
            }
        });
    };
    
    let toggle_collapsed = move |_: web_sys::MouseEvent| {
        if !pinned.get() {
            collapsed.update(|c| *c = !*c);
        }
    };
    
    view! {
        <div style="position: relative;">
            <Sidebar collapsed=collapsed>
                <div style="position: absolute; top: 0.5rem; right: 0.5rem; z-index: 10; display: flex; gap: 0.5rem;">
                    <button 
                        on:click=toggle_pin
                        style={move || format!(
                            "padding: 0.5rem; background: {}; border: 1px solid var(--theme-surface-border); border-radius: var(--radius-sm); cursor: pointer; font-size: 1rem;",
                            if pinned.get() { "var(--color-primary)" } else { "var(--theme-surface-bg)" }
                        )}
                        title={move || if pinned.get() { "Unpin sidebar" } else { "Pin sidebar" }}
                    >
                        {move || if pinned.get() { "📌" } else { "📍" }}
                    </button>
                    
                    <Show when=move || !pinned.get()>
                        <button 
                            on:click=toggle_collapsed
                            style="padding: 0.5rem; background: var(--theme-surface-bg); border: 1px solid var(--theme-surface-border); border-radius: var(--radius-sm); cursor: pointer; font-size: 1rem;"
                        >
                            {move || if collapsed.get() { "→" } else { "←" }}
                        </button>
                    </Show>
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
                        {move || if pinned.get() {
                            "📌 Pinned | © 2026 CanonRS"
                        } else {
                            "© 2026 CanonRS"
                        }}
                    </div>
                </SidebarFooter>
            </Sidebar>
        </div>
    }
}
