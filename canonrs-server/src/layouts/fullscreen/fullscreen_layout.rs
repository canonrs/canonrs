//! # FullscreenLayout — Regions: header, main
use leptos::prelude::*;

#[component]
pub fn FullscreenLayout(
    #[prop(optional)] header: Option<ChildrenFn>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="layout-fullscreen" data-layout="fullscreen" data-layout-version="1">
            <header class="layout-fullscreen-header" data-layout-region="header">
                {header.map(|h| h())}
            </header>
            <main class="layout-fullscreen-main" data-layout-region="main">
                {children()}
            </main>
        </div>
    }
}
