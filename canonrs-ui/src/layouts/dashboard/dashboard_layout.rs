//! # DashboardLayout — Regions: header, sidebar, main
use leptos::prelude::*;

#[component]
pub fn DashboardLayout(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] sidebar: Option<ChildrenFn>,
    #[prop(default = Signal::derive(|| String::new()))] header_zone_id: Signal<String>,
    #[prop(default = Signal::derive(|| String::new()))] sidebar_zone_id: Signal<String>,
    #[prop(default = Signal::derive(|| String::new()))] main_zone_id: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div data-layout="dashboard" data-layout-version="1" class="layout-dashboard">
            <header class="layout-dashboard-header"
                data-layout-region="header"
                data-region-hint="Drop nav or toolbar"
                data-drop-zone=move || (!header_zone_id.get().is_empty()).then_some("").unwrap_or("")
                data-zone-id=move || header_zone_id.get()>
                {header.map(|h| h())}
            </header>
            <div class="layout-dashboard-body">
                <aside class="layout-dashboard-sidebar"
                    data-layout-region="sidebar"
                    data-region-hint="Drop navigation menu"
                    data-drop-zone=move || (!sidebar_zone_id.get().is_empty()).then_some("").unwrap_or("")
                    data-zone-id=move || sidebar_zone_id.get()>
                    {sidebar.map(|s| s())}
                </aside>
                <main class="layout-dashboard-main"
                    data-layout-region="main"
                    data-region-hint="Drop page content"
                    data-drop-zone=move || (!main_zone_id.get().is_empty()).then_some("").unwrap_or("")
                    data-zone-id=move || main_zone_id.get()>
                    {children()}
                </main>
            </div>
        </div>
    }
}
