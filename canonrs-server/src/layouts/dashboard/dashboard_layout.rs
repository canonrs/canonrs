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
    view! {
        <div data-rs-layout-dashboard="" data-rs-uid=uid class=class>
            {move || header.get_value().map(|h| view! { <div data-rs-region="header">{h()}</div> })}
            {move || sidebar.get_value().map(|s| view! { <div data-rs-region="sidebar">{s()}</div> })}
            {move || content.get_value().map(|c| view! { <div data-rs-region="content">{c()}</div> })}
        </div>
    }
}
