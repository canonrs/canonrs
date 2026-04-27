use leptos::prelude::*;
use canonrs_core::infra::uid::generate;

#[component]
pub fn DashboardLayout(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] sidebar: Option<ChildrenFn>,
    #[prop(optional)] content: Option<ChildrenFn>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid     = generate("ly");
    let header  = StoredValue::new(header);
    let sidebar = StoredValue::new(sidebar);
    let content = StoredValue::new(content);
    // Chamar os slots UMA vez — evita recriação do DOM. CR-404.
    let header_rendered  = header.get_value().map(|h| h());
    let sidebar_rendered = sidebar.get_value().map(|s| s());
    let content_rendered = content.get_value().map(|c| c());

    view! {
        <div data-rs-layout="dashboard" data-rs-uid=uid class=class>
            <header data-rs-region="header">{header_rendered}</header>
            <nav data-rs-region="sidebar" aria-label="Sidebar">{sidebar_rendered}</nav>
            <main data-rs-region="content">{content_rendered}</main>
        </div>
    }
}
