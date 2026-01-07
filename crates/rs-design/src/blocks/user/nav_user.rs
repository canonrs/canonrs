use crate::ui::{SidebarMenu, SidebarMenuItem, SidebarMenuButton};
use leptos::prelude::*;

#[derive(Clone)]
pub struct User {
    pub name: String,
    pub email: String,
    pub avatar: Option<String>,
}

#[component]
pub fn NavUser(
    user: User,
) -> impl IntoView {
    let user_stored = StoredValue::new(user);
    
    view! {
        <SidebarMenu>
            <SidebarMenuItem>
                <SidebarMenuButton>
                    <div class="flex items-center gap-2">
                        <div class="flex size-8 items-center justify-center rounded-full bg-sidebar-accent text-sidebar-accent-foreground">
                            {move || {
                                user_stored.with_value(|u| {
                                    u.name.chars().next().unwrap_or('U').to_uppercase().to_string()
                                })
                            }}
                        </div>
                        <div class="flex flex-col text-left text-sm">
                            <span class="font-semibold">
                                {move || user_stored.with_value(|u| u.name.clone())}
                            </span>
                            <span class="text-xs text-muted-foreground">
                                {move || user_stored.with_value(|u| u.email.clone())}
                            </span>
                        </div>
                    </div>
                </SidebarMenuButton>
            </SidebarMenuItem>
        </SidebarMenu>
    }
}
