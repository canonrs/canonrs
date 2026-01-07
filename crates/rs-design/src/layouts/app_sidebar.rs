use crate::ui::{
    Sidebar, SidebarContent, SidebarFooter, SidebarGroupLabel, SidebarHeader, SidebarInset,
    SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarProvider,
};
use leptos::prelude::*;

#[component]
pub fn AppSidebar() -> impl IntoView {
    view! {
        <SidebarProvider is_mobile=false>
            <Sidebar>
                <SidebarHeader>
                    <div class="px-3 py-2">
                        <h2 class="text-lg font-semibold">"My App"</h2>
                    </div>
                </SidebarHeader>

                <SidebarContent>
                    <div class="px-3 py-2">
                        <SidebarGroupLabel>"Navigation"</SidebarGroupLabel>
                        <SidebarMenu>
                            <SidebarMenuItem>
                                <SidebarMenuButton is_active=true>
                                    "Dashboard"
                                </SidebarMenuButton>
                            </SidebarMenuItem>
                            <SidebarMenuItem>
                                <SidebarMenuButton>
                                    "Projects"
                                </SidebarMenuButton>
                            </SidebarMenuItem>
                            <SidebarMenuItem>
                                <SidebarMenuButton>
                                    "Settings"
                                </SidebarMenuButton>
                            </SidebarMenuItem>
                        </SidebarMenu>
                    </div>
                </SidebarContent>

                <SidebarFooter>
                    <div class="px-3 py-2 text-sm text-muted-foreground">
                        "User Menu Here"
                    </div>
                </SidebarFooter>
            </Sidebar>

            <SidebarInset>
                <div class="p-6">
                    <h1 class="text-2xl font-bold">"Main Content"</h1>
                    <p>"Your content goes here"</p>
                </div>
            </SidebarInset>
        </SidebarProvider>
    }
}
