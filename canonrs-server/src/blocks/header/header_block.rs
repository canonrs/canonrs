//! # Header Block
use leptos::prelude::*;

#[component]
pub fn Header(
    #[prop(optional)] logo: Option<ChildrenFn>,
    #[prop(optional)] nav: Option<ChildrenFn>,
    #[prop(optional)] center: Option<ChildrenFn>,
    #[prop(optional)] actions: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <header data-block="header" data-block-version="1" class=class>
            {logo.map(|l| view! { <div data-block-region="logo">{l()}</div> })}
            {nav.map(|n| view! { <nav data-block-region="nav">{n()}</nav> })}
            {center.map(|c| view! { <div data-block-region="center">{c()}</div> })}
            {actions.map(|a| view! { <div data-block-region="actions">{a()}</div> })}
        </header>
    }
}
