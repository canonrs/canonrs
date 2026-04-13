use leptos::prelude::*;
use canonrs_core::infra::uid::generate;

#[component]
pub fn PageHeader(
    #[prop(optional)] breadcrumb: Option<ChildrenFn>,
    #[prop(optional)] title: Option<ChildrenFn>,
    #[prop(optional)] subtitle: Option<ChildrenFn>,
    #[prop(optional)] actions: Option<ChildrenFn>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid = generate("bl");
    let breadcrumb = StoredValue::new(breadcrumb);
    let title = StoredValue::new(title);
    let subtitle = StoredValue::new(subtitle);
    let actions = StoredValue::new(actions);
    view! {
        <div data-rs-page-header="" data-rs-uid=uid class=class>
            {move || breadcrumb.get_value().map(|b| view! { <div data-rs-region="breadcrumb">{b()}</div> })}
            {move || title.get_value().map(|t| view! { <div data-rs-region="title">{t()}</div> })}
            {move || subtitle.get_value().map(|s| view! { <div data-rs-region="subtitle">{s()}</div> })}
            {move || actions.get_value().map(|a| view! { <div data-rs-region="actions">{a()}</div> })}
        </div>
    }
}
