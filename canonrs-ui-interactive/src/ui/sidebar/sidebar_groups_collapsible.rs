use leptos::prelude::*;
use canonrs_ui::ui::sidebar::{
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarGroupLabel, SidebarSeparator
};
use canonrs_ui::ui::avatar::{Avatar, AvatarImage, AvatarFallback, AvatarSize, AvatarStatus};
use canonrs_ui::ui::badge::{Badge, BadgeVariant};

#[component]
pub fn SidebarGroupsCollapsible(
    #[prop(default = false)] default_collapsed: bool,
) -> impl IntoView {
    let sidebar_collapsed = RwSignal::new(default_collapsed);
    
    // Group collapse states
    let navigation_expanded = RwSignal::new(true);
    let settings_expanded = RwSignal::new(true);

    // Load from localStorage on mount
    Effect::new(move |_| {
        #[cfg(feature = "hydrate")]
        {
            use leptos::leptos_dom::helpers::window;
            if let Ok(storage) = window().local_storage() {
                if let Some(storage) = storage {
                    if let Ok(Some(val)) = storage.get_item("sidebar-group-navigation") {
                        navigation_expanded.set(val == "true");
                    }
                    if let Ok(Some(val)) = storage.get_item("sidebar-group-settings") {
                        settings_expanded.set(val == "true");
                    }
                }
            }
        }
    });

    // Save to localStorage
    let save_group_state = move |group: &str, expanded: bool| {
        #[cfg(feature = "hydrate")]
        {
            use leptos::leptos_dom::helpers::window;
            if let Ok(storage) = window().local_storage() {
                if let Some(storage) = storage {
                    let _ = storage.set_item(
                        &format!("sidebar-group-{}", group),
                        if expanded { "true" } else { "false" }
                    );
                }
            }
        }
    };

    let toggle_navigation = move |_| {
        let new_state = !navigation_expanded.get();
        navigation_expanded.set(new_state);
        save_group_state("navigation", new_state);
    };

    let toggle_settings = move |_| {
        let new_state = !settings_expanded.get();
        settings_expanded.set(new_state);
        save_group_state("settings", new_state);
    };

    view! {
        <div style="position: relative;">
            <Sidebar collapsed=sidebar_collapsed>
                <button
                    on:click=move |_| sidebar_collapsed.update(|c| *c = !*c)
                    style="position: absolute; top: 0.5rem; right: 0.5rem; z-index: 10; padding: 0.5rem; background: var(--theme-surface-bg); border: 1px solid var(--theme-surface-border); border-radius: var(--radius-sm); cursor: pointer; font-size: 1rem;"
                >
                    {move || if sidebar_collapsed.get() { "→" } else { "←" }}
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
                        
                        // Group 1: Navigation (collapsible)
                        <div style="margin-bottom: var(--space-sm);">
                            <button
                                on:click=toggle_navigation
                                style="width: 100%; display: flex; align-items: center; justify-content: space-between; padding: var(--sidebar-group-label-padding-y) var(--sidebar-group-label-padding-x); background: transparent; border: none; cursor: pointer; font-size: var(--sidebar-group-label-font-size); font-weight: var(--sidebar-group-label-font-weight); color: var(--sidebar-group-label-fg); text-align: left;"
                            >
                                <span>"Navigation"</span>
                                <span style="transition: transform 0.2s ease;">
                                    {move || if navigation_expanded.get() { "▼" } else { "▶" }}
                                </span>
                            </button>

                            <div style=move || if navigation_expanded.get() { 
                                "display: block; animation: slideDown 0.2s ease;" 
                            } else { 
                                "display: none;" 
                            }>
                                <SidebarMenuItem href="/dashboard".to_string() active=true>
                                    <span data-sidebar-icon>"📊"</span>
                                    <span data-sidebar-label>"Dashboard"</span>
                                </SidebarMenuItem>

                                <SidebarMenuItem href="/projects".to_string()>
                                    <span data-sidebar-icon>"📁"</span>
                                    <span data-sidebar-label>"Projects"</span>
                                    <Badge variant=BadgeVariant::Primary>"12"</Badge>
                                </SidebarMenuItem>

                                <SidebarMenuItem href="/tasks".to_string()>
                                    <span data-sidebar-icon>"✓"</span>
                                    <span data-sidebar-label>"Tasks"</span>
                                    <Badge variant=BadgeVariant::Error>"5"</Badge>
                                </SidebarMenuItem>

                                <SidebarMenuItem href="/messages".to_string()>
                                    <span data-sidebar-icon>"💬"</span>
                                    <span data-sidebar-label>"Messages"</span>
                                    <Badge variant=BadgeVariant::Warning>"3"</Badge>
                                </SidebarMenuItem>
                            </div>
                        </div>

                        <SidebarSeparator />

                        // Group 2: Settings (collapsible)
                        <div style="margin-bottom: var(--space-sm);">
                            <button
                                on:click=toggle_settings
                                style="width: 100%; display: flex; align-items: center; justify-content: space-between; padding: var(--sidebar-group-label-padding-y) var(--sidebar-group-label-padding-x); background: transparent; border: none; cursor: pointer; font-size: var(--sidebar-group-label-font-size); font-weight: var(--sidebar-group-label-font-weight); color: var(--sidebar-group-label-fg); text-align: left;"
                            >
                                <span>"Settings"</span>
                                <span style="transition: transform 0.2s ease;">
                                    {move || if settings_expanded.get() { "▼" } else { "▶" }}
                                </span>
                            </button>

                            <div style=move || if settings_expanded.get() { 
                                "display: block; animation: slideDown 0.2s ease;" 
                            } else { 
                                "display: none;" 
                            }>
                                <SidebarMenuItem href="/profile".to_string()>
                                    <span data-sidebar-icon>"👤"</span>
                                    <span data-sidebar-label>"Profile"</span>
                                </SidebarMenuItem>

                                <SidebarMenuItem href="/preferences".to_string()>
                                    <span data-sidebar-icon>"⚙"</span>
                                    <span data-sidebar-label>"Preferences"</span>
                                </SidebarMenuItem>

                                <SidebarMenuItem href="/notifications".to_string()>
                                    <span data-sidebar-icon>"🔔"</span>
                                    <span data-sidebar-label>"Notifications"</span>
                                    <Badge variant=BadgeVariant::Success>"New"</Badge>
                                </SidebarMenuItem>
                            </div>
                        </div>

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
