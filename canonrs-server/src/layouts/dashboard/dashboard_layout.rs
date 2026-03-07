//! # DashboardLayout — Regions: header, sidebar, main
use leptos::prelude::*;

#[component]
pub fn DashboardLayout(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] sidebar: Option<ChildrenFn>,
    children: Children,
) -> impl IntoView {
    view! {
        <div data-layout="dashboard" data-layout-version="1" class="layout-dashboard">
            <header class="layout-dashboard-header" data-layout-region="header">
                {header.map(|h| h())}
            </header>
            <div class="layout-dashboard-body">
                <aside class="layout-dashboard-sidebar" data-layout-region="sidebar">
                    {sidebar.map(|s| s())}
                </aside>
                <main class="layout-dashboard-main" data-layout-region="main">
                    {children()}
                </main>
            </div>
        </div>
    }
}
