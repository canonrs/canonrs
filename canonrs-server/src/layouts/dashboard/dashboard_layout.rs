use leptos::prelude::*;

#[component]
pub fn DashboardLayout(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] sidebar: Option<ChildrenFn>,
    #[prop(optional)] content: Option<ChildrenFn>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let header  = StoredValue::new(header);
    let sidebar = StoredValue::new(sidebar);
    let content = StoredValue::new(content);
    view! {
        <div data-rs-layout="dashboard" class=class>
            {move || header.get_value().map(|h| view! { <header data-rs-region="header">{h()}</header> })}
            {move || sidebar.get_value().map(|s| view! { <nav data-rs-region="sidebar" aria-label="Sidebar">{s()}</nav> })}
            {move || content.get_value().map(|c| view! { <main data-rs-region="content">{c()}</main> })}
        </div>
    }
}
