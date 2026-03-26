//! # CodeBlock Block
use leptos::prelude::*;

#[component]
pub fn CodeBlockBlock(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] actions: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] style: String,
    #[prop(optional)] content: Option<ChildrenFn>,
) -> impl IntoView {
    view! {
        <div
            data-block="code-block"
            data-block-version="1"
            style=style
            class=class
        >
            {header.map(|h| view! { <div data-block-region="header">{h()}</div> })}
            {content.map(|c| view! { <div data-block-region="content">{c()}</div> })}
            {actions.map(|a| view! { <div data-block-region="actions">{a()}</div> })}
        </div>
    }
}
