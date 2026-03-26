//! # FullscreenLayout — Regions: header, content
use leptos::prelude::*;

#[component]
pub fn FullscreenLayout(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] content: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div data-layout="fullscreen" data-layout-version="1" class=class>
            {header.map(|h| view! { <div data-layout-region="header">{h()}</div> })}
            {content.map(|c| view! { <div data-layout-region="content">{c()}</div> })}
        </div>
    }
}
