//! # FullscreenLayout
//! **Purpose:** Focused experiences, builders, editors, tools
//! **Token Family:** Layout (layout.fullscreen.*)

use leptos::prelude::*;

#[component]
pub fn FullscreenLayout(
    #[prop(optional)]
    header: Option<ChildrenFn>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="layout-fullscreen" attr:data-layout="fullscreen">
            {header.map(|h| view! {
                <header class="layout-fullscreen-header" attr:data-layout-region="header">{h()}</header>
            })}
            <main class="layout-fullscreen-content" attr:data-layout-region="content">{children()}</main>
        </div>
    }
}
