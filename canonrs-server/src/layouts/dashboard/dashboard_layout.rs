//! @canon-id: dashboard
//! @canon-type: layout
//! @canon-category: layout
//! @canon-variant: page
//! @canon-container: true
//! @canon-regions: header, sidebar, content
//! @canon-label: Dashboard
//! @canon-icon: ⬛
//! @canon-description: App shell with header, sidebar and main content area
//! @canon-tags: dashboard, app, admin, header, sidebar, main
//! @canon-slot-accepts: header=Nav,sidebar=Nav,content=Any
//! @canon-slot-descriptions: header:Top navigation bar,sidebar:Left navigation panel,content:Primary content area
use leptos::prelude::*;

#[component]
pub fn DashboardLayout(
    #[prop(optional)] header: Option<AnyView>,
    #[prop(optional)] sidebar: Option<AnyView>,
    #[prop(optional)] content: Option<AnyView>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-layout="" data-rs-component="Dashboard" class=class>
            {header.map(|h| view! { <div data-rs-region="header">{h}</div> })}
            {sidebar.map(|s| view! { <div data-rs-region="sidebar">{s}</div> })}
            {content.map(|c| view! { <div data-rs-region="content">{c}</div> })}
        </div>
    }
}
