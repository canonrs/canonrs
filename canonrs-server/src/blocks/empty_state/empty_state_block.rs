//! # EmptyState Block
use leptos::prelude::*;

#[component]
pub fn EmptyState(
    #[prop(optional)] icon: Option<ChildrenFn>,
    #[prop(optional)] title: Option<ChildrenFn>,
    #[prop(optional)] description: Option<ChildrenFn>,
    #[prop(optional)] action: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] _style: String,
) -> impl IntoView {
    view! {
        <div data-block="empty-state" data-block-version="1" class=class>
            {icon.map(|i| view! { <div data-block-region="icon">{i()}</div> })}
            {title.map(|t| view! { <div data-block-region="title">{t()}</div> })}
            {description.map(|d| view! { <div data-block-region="description">{d()}</div> })}
            {action.map(|a| view! { <div data-block-region="action">{a()}</div> })}
        </div>
    }
}
