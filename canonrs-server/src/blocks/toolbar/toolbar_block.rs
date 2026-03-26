//! # Toolbar Block
use leptos::prelude::*;

#[component]
pub fn ToolbarBlock(
    #[prop(optional)] left: Option<ChildrenFn>,
    #[prop(optional)] center: Option<ChildrenFn>,
    #[prop(optional)] right: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div
            data-block="toolbar"
            data-block-version="1"
            class=class
        >
            {left.map(|l| view! { <div data-block-region="left">{l()}</div> })}
            {center.map(|c| view! { <div data-block-region="center">{c()}</div> })}
            {right.map(|r| view! { <div data-block-region="right">{r()}</div> })}
        </div>
    }
}
