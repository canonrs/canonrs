//! # FullscreenLayout — Regions: header, main (always rendered)
use leptos::prelude::*;

#[component]
pub fn FullscreenLayout(
    #[prop(optional)] header: Option<ChildrenFn>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="layout-fullscreen" data-layout="fullscreen" data-layout-version="1">
            <header class="layout-fullscreen-header"
                data-layout-region="header"
                data-region-hint="Drop toolbar"
                data-region-meta="Nav · max 1">
                {header.map(|h| h())}
            </header>
            <main class="layout-fullscreen-main"
                data-layout-region="main"
                data-region-hint="Drop editor content"
                data-region-meta="Content · ∞">
                {children()}
            </main>
        </div>
    }
}
