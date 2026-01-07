//! Dashboard Layout - SSR-safe

use crate::blocks::dashboard::{
    ActivityList, DashboardConfig, DashboardState, DashboardStats, EmptyState, ErrorState,
};
use crate::blocks::navigation::NavItem;
use crate::layouts::Header;
use crate::tokens::animations::AnimationVariant;
use crate::ui::{
    Animate, Sidebar, SidebarContent, SidebarGroupLabel, SidebarHeader, SidebarInset, SidebarMenu,
    SidebarProvider,
};
use leptos::prelude::*;

#[component]
pub fn DashboardLayout(#[prop(optional)] config: Option<DashboardConfig>) -> impl IntoView {
    let _config = config.unwrap_or_default();
    let state = RwSignal::new(DashboardState::Loading);

    // Carregamento simulado APENAS no browser
    if leptos::prelude::is_browser() {
        #[cfg(target_arch = "wasm32")]
        {
            Effect::new(move || {
                set_timeout(
                    move || state.set(DashboardState::Ready),
                    std::time::Duration::from_secs(1),
                );
            });
        }
    }

    let chart_data = RwSignal::new(vec![
        crate::ui::ChartDataPoint {
            date: "Jan".to_string(),
            desktop: 186.0,
            mobile: 80.0,
        },
        crate::ui::ChartDataPoint {
            date: "Feb".to_string(),
            desktop: 305.0,
            mobile: 200.0,
        },
        crate::ui::ChartDataPoint {
            date: "Mar".to_string(),
            desktop: 237.0,
            mobile: 120.0,
        },
        crate::ui::ChartDataPoint {
            date: "Apr".to_string(),
            desktop: 273.0,
            mobile: 190.0,
        },
        crate::ui::ChartDataPoint {
            date: "May".to_string(),
            desktop: 209.0,
            mobile: 130.0,
        },
        crate::ui::ChartDataPoint {
            date: "Jun".to_string(),
            desktop: 214.0,
            mobile: 140.0,
        },
    ]);

    let handle_retry = Callback::new(move |_| {
        state.set(DashboardState::Loading);

        if leptos::prelude::is_browser() {
            #[cfg(target_arch = "wasm32")]
            {
                set_timeout(
                    move || state.set(DashboardState::Ready),
                    std::time::Duration::from_secs(1),
                );
            }
        }
    });

    view! {
        <SidebarProvider is_mobile=false>
            <Sidebar>
                <SidebarHeader>
                    <Animate variant=AnimationVariant::FadeIn>
                        <div class="p-4">
                            <h2 class="text-lg font-semibold">"Dashboard"</h2>
                        </div>
                    </Animate>
                </SidebarHeader>

                <SidebarContent>
                    <div class="px-3 py-2">
                        <SidebarGroupLabel>"Main"</SidebarGroupLabel>
                        <SidebarMenu>
                            <NavItem label="Dashboard" href="/dashboard" active=true />
                            <NavItem label="Analytics" href="/analytics" />
                            <NavItem label="Documents" href="/documents" />
                            <NavItem label="Settings" href="/settings" />
                        </SidebarMenu>
                    </div>
                </SidebarContent>
            </Sidebar>

            <SidebarInset>
                <Header show_sidebar_trigger=true>
                    <h1 class="text-lg font-semibold">"Dashboard Overview"</h1>
                </Header>

                <main class="flex-1 p-6">
                    {move || match state.get() {
                        DashboardState::Loading => view! {
                            <div class="flex items-center justify-center h-64">
                                <div class="text-muted-foreground">"Loading..."</div>
                            </div>
                        }.into_any(),

                        DashboardState::Ready => view! {
                            <Animate variant=AnimationVariant::FadeUp>
                                <div class="space-y-6">
                                    <DashboardStats />
                                    {/* <ChartAreaInteractive data=chart_data.get() /> */}
                                    <ActivityList />
                                </div>
                            </Animate>
                        }.into_any(),

                        DashboardState::Empty => view! {
                            <EmptyState
                                title="No data available".to_string()
                                description="Start by adding some data to your dashboard".to_string()
                                action_label="Add Data".to_string()
                            />
                        }.into_any(),

                        DashboardState::Error => view! {
                            <ErrorState
                                message="Failed to load dashboard".to_string()
                                on_retry=handle_retry
                            />
                        }.into_any(),
                    }}
                </main>
            </SidebarInset>
        </SidebarProvider>
    }
}
