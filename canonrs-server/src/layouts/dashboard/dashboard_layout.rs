//! # DashboardLayout — Regions: header, sidebar, content
use leptos::prelude::*;

#[component]
pub fn DashboardLayout(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] sidebar: Option<ChildrenFn>,
    #[prop(optional)] content: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div data-layout="dashboard" data-layout-version="1" class=class>
            {header.map(|h| view! { <div data-layout-region="header">{h()}</div> })}
            {sidebar.map(|s| view! { <div data-layout-region="sidebar">{s()}</div> })}
            {content.map(|c| view! { <div data-layout-region="content">{c()}</div> })}
        </div>
    }
}
