//! # Toolbar Block
//! Container for actions and tools

use leptos::prelude::*;

#[component]
pub fn ToolbarBlock(
    #[prop(optional)] left: Option<ChildrenFn>,
    #[prop(optional)] center: Option<ChildrenFn>,
    #[prop(optional)] right: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=format!("canon-toolbar {}", class)
            data-block="toolbar"
            data-block-version="1"
            role="toolbar"
        >
            <div data-block-region="left">{left.map(|l| l())}</div>
            <div data-block-region="center">{center.map(|c| c())}</div>
            <div data-block-region="main">{children()}</div>
            <div data-block-region="right">{right.map(|r| r())}</div>
        </div>
    }
}
