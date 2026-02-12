use leptos::prelude::*;
use canonrs_ui::ui::sidebar::{
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarGroupLabel, SidebarSeparator
};
use canonrs_ui::ui::avatar::{Avatar, AvatarImage, AvatarFallback, AvatarSize, AvatarStatus};
use crate::ui::command::{CommandInteractive, CommandItemData};

#[component]
pub fn SidebarWithSearch(
    #[prop(default = false)] default_collapsed: bool,
) -> impl IntoView {
    let collapsed = RwSignal::new(default_collapsed);
    let selected_item = RwSignal::new(String::new());
    
    let menu_items = vec![
        CommandItemData {
            id: "dashboard".to_string(),
            label: "📊 Dashboard".to_string(),
            value: "/dashboard".to_string(),
            keywords: vec!["home".to_string(), "overview".to_string()],
            group: Some("Navigation".to_string()),
        },
        CommandItemData {
            id: "projects".to_string(),
            label: "📁 Projects".to_string(),
            value: "/projects".to_string(),
            keywords: vec!["work".to_string(), "portfolio".to_string()],
            group: Some("Navigation".to_string()),
        },
        CommandItemData {
            id: "tasks".to_string(),
            label: "✓ Tasks".to_string(),
            value: "/tasks".to_string(),
            keywords: vec!["todo".to_string(), "work".to_string()],
            group: Some("Navigation".to_string()),
        },
        CommandItemData {
            id: "profile".to_string(),
            label: "👤 Profile".to_string(),
            value: "/profile".to_string(),
            keywords: vec!["user".to_string(), "account".to_string()],
            group: Some("Settings".to_string()),
        },
        CommandItemData {
            id: "preferences".to_string(),
            label: "⚙ Preferences".to_string(),
            value: "/preferences".to_string(),
            keywords: vec!["settings".to_string(), "config".to_string()],
            group: Some("Settings".to_string()),
        },
    ];
    
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
                    <div style="display: flex; flex-direction: column; gap: 0.75rem; padding: 1rem;">
                        <div style="display: flex; align-items: center; gap: 0.75rem;">
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
                    </div>
                </SidebarHeader>

                <SidebarContent>
                    <Show when=move || !collapsed.get()>
                        <div style="padding: 0 1rem 1rem 1rem;">
                            <CommandInteractive
                                items=menu_items.clone()
                                placeholder="Search menu...".to_string()
                                on_select=Callback::new(move |value: String| {
                                    selected_item.set(value);
                                })
                            />
                        </div>
                    </Show>
                    
                    <SidebarMenu>
                        <SidebarGroupLabel>"Quick Access"</SidebarGroupLabel>
                        <SidebarMenuItem href="/dashboard".to_string() active=true>
                            <span data-sidebar-icon>"📊"</span>
                            <span data-sidebar-label>"Dashboard"</span>
                        </SidebarMenuItem>
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
