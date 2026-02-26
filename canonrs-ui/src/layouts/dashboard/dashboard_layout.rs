//! # DashboardLayout — Regions: header, sidebar, main (always rendered)
use leptos::prelude::*;

#[component]
pub fn DashboardLayout(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] sidebar: Option<ChildrenFn>,
    children: Children,
) -> impl IntoView {
    view! {
        <div data-layout="dashboard" data-layout-version="1" class="layout-dashboard">
            <header class="layout-dashboard-header"
                data-layout-region="header"
                data-region-hint="Drop nav or toolbar"
                data-region-meta="Nav · max 1">
                {header.map(|h| h())}
            </header>
            <div class="layout-dashboard-body">
                <aside class="layout-dashboard-sidebar"
                    data-layout-region="sidebar"
                    data-region-hint="Drop navigation menu"
                    data-region-meta="Nav · max 1">
                    {sidebar.map(|s| s())}
                </aside>
                <main class="layout-dashboard-main"
                    data-layout-region="main"
                    data-region-hint="Drop page content"
                    data-region-meta="Content · ∞">
                    {children()}
                </main>
            </div>
        </div>
    }
}
