//! # SidebarLayout Block

use leptos::prelude::*;

#[component]
pub fn SidebarLayout(
    #[prop(optional)] nav: Option<ChildrenFn>,
    #[prop(optional)] main: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div
            class=class
            data-block="sidebar-layout"
            data-block-version="1"
        >
            <div data-block-region="nav">{nav.map(|c| c())}</div>
            <div data-block-region="main">{main.map(|c| c())}</div>
        </div>
    }
}
