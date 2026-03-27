//! @canon-id: toolbar
//! @canon-type: block
//! @canon-category: page
//! @canon-variant: page
//! @canon-container: true
//! @canon-regions: left, center, right
//! @canon-label: Toolbar
//! @canon-description: Action toolbar bar
//! @canon-tags: toolbar, bar, actions, buttons, tools
//! @canon-slot-accepts: left=Nav,center=Any,right=Action
use leptos::prelude::*;

#[component]
pub fn ToolbarBlock(
    #[prop(optional)] left: Option<ChildrenFn>,
    #[prop(optional)] center: Option<ChildrenFn>,
    #[prop(optional)] right: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] style: String,
) -> impl IntoView {
    view! {
        <div
            data-block="toolbar"
            data-block-version="1"
            style=style
            class=class
        >
            {left.map(|l| view! { <div data-block-region="left">{l()}</div> })}
            {center.map(|c| view! { <div data-block-region="center">{c()}</div> })}
            {right.map(|r| view! { <div data-block-region="right">{r()}</div> })}
        </div>
    }
}
