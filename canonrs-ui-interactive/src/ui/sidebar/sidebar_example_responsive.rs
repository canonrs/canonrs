use leptos::prelude::*;
use canonrs_ui::ui::sidebar::{
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarGroupLabel, SidebarSeparator
};
use canonrs_ui::ui::avatar::{Avatar, AvatarImage, AvatarFallback, AvatarSize, AvatarStatus};
use canonrs_ui::ui::badge::{Badge, BadgeVariant};

#[derive(Debug, Clone, Copy, PartialEq)]
enum ViewportMode {
    Desktop,
    Tablet,
    Mobile,
}

#[component]
pub fn SidebarResponsive() -> impl IntoView {
    let viewport_mode = RwSignal::new(ViewportMode::Desktop);
    let is_open = RwSignal::new(false);
    
    use leptos::web_sys::window;
    use leptos::wasm_bindgen::closure::Closure;
    use leptos::wasm_bindgen::JsCast;

    create_effect(move |_| {
        if let Some(win) = window() {
            let win_clone = win.clone();
            
            let update_viewport = move || {
                if let Ok(width) = win_clone.inner_width() {
                    if let Some(width) = width.as_f64() {
                        let mode = if width >= 1024.0 {
                            ViewportMode::Desktop
                        } else if width >= 768.0 {
                            ViewportMode::Tablet
                        } else {
                            ViewportMode::Mobile
                        };

                        viewport_mode.set(mode);

                        if mode == ViewportMode::Desktop {
                            is_open.set(true);
                        }
                    }
                }
            };

            update_viewport();

            let closure = Closure::wrap(Box::new(move || {
                update_viewport();
            }) as Box<dyn FnMut()>);

            let _ = win.add_event_listener_with_callback(
                "resize",
                closure.as_ref().unchecked_ref(),
            );

            closure.forget();
        }
    });

    let toggle_sidebar = move |_| is_open.update(|o| *o = !*o);
    let close_sidebar = move |_| is_open.set(false);

    let container_classes = move || {
        format!(
            "responsive-sidebar-container {} {}",
            match viewport_mode.get() {
                ViewportMode::Desktop => "mode-desktop",
                ViewportMode::Tablet => "mode-tablet",
                ViewportMode::Mobile => "mode-mobile",
            },
            if is_open.get() { "is-open" } else { "" }
        )
    };

    view! {
        <div class=container_classes>
            
            <button on:click=toggle_sidebar class="hamburger-btn">"☰"</button>

            <div on:click=close_sidebar class="sidebar-overlay" />

            <div class="sidebar-wrapper">
                <Sidebar collapsed=false>
                    
                    <button on:click=close_sidebar class="close-btn">"✕"</button>

                    <SidebarHeader>
                        <div style="display: flex; align-items: center; gap: 0.75rem; padding: 1rem;">
                            <Avatar size=AvatarSize::Md status=AvatarStatus::Online>
                                <AvatarImage src="https://i.pravatar.cc/150?img=10".to_string() alt="User".to_string() />
                                <AvatarFallback>"JD"</AvatarFallback>
                            </Avatar>
                            <div data-sidebar-label="" style="flex: 1;">
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
                            <SidebarSeparator />
                            <SidebarGroupLabel>"Settings"</SidebarGroupLabel>
                            <SidebarMenuItem href="/profile".to_string()>
                                <span data-sidebar-icon>"👤"</span>
                                <span data-sidebar-label>"Profile"</span>
                            </SidebarMenuItem>
                        </SidebarMenu>
                    </SidebarContent>

                    <SidebarFooter>
                        <div data-sidebar-label="" style="padding: 0.75rem; font-size: 0.75rem;">
                            <div>{move || format!("Mode: {:?}", viewport_mode.get())}</div>
                            <div>"© 2026 CanonRS"</div>
                        </div>
                    </SidebarFooter>
                </Sidebar>
            </div>

        </div>
    }
}
