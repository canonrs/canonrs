//! # PageHeader Block
use leptos::prelude::*;

#[component]
pub fn PageHeader(
    #[prop(optional)] breadcrumb: Option<ChildrenFn>,
    #[prop(optional)] title: Option<ChildrenFn>,
    #[prop(optional)] subtitle: Option<ChildrenFn>,
    #[prop(optional)] actions: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div data-block="page-header" data-block-version="1" class=class>
            {breadcrumb.map(|b| view! { <div data-block-region="breadcrumb">{b()}</div> })}
            {title.map(|t| view! { <div data-block-region="title">{t()}</div> })}
            {subtitle.map(|s| view! { <div data-block-region="subtitle">{s()}</div> })}
            {actions.map(|a| view! { <div data-block-region="actions">{a()}</div> })}
        </div>
    }
}
