//! # FullscreenLayout — Regions: header, main
use leptos::prelude::*;

#[component]
pub fn FullscreenLayout(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(default = Signal::derive(|| String::new()))] header_zone_id: Signal<String>,
    #[prop(default = Signal::derive(|| String::new()))] main_zone_id: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="layout-fullscreen" data-layout="fullscreen" data-layout-version="1">
            <header class="layout-fullscreen-header"
                data-layout-region="header"
                data-region-hint="Drop toolbar"
                data-drop-zone=move || (!header_zone_id.get().is_empty()).then_some("")
                data-zone-id=move || (!header_zone_id.get().is_empty()).then(|| header_zone_id.get())>
                {header.map(|h| h())}
            </header>
            <main class="layout-fullscreen-main"
                data-layout-region="main"
                data-region-hint="Drop editor content"
                data-drop-zone=move || (!main_zone_id.get().is_empty()).then_some("")
                data-zone-id=move || (!main_zone_id.get().is_empty()).then(|| main_zone_id.get())>
                {children()}
            </main>
        </div>
    }
}
