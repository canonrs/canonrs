//! # Dialog Block
use leptos::prelude::*;

#[component]
pub fn DialogBlock(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(optional)] content: Option<ChildrenFn>,
) -> impl IntoView {
    view! {
        <div data-block="dialog" data-block-version="1" class=class>
            {header.map(|h| view! { <div data-block-region="header">{h()}</div> })}
            {content.map(|c| view! { <div data-block-region="content">{c()}</div> })}
            {footer.map(|f| view! { <div data-block-region="footer">{f()}</div> })}
        </div>
    }
}
