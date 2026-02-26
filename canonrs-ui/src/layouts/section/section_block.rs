//! # Section — Regions: header, main, footer (always rendered)
use leptos::prelude::*;

#[component]
pub fn Section(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
    children: Children,
) -> impl IntoView {
    view! {
        <section
            id={if id.is_empty() { None } else { Some(id) }}
            class={format!("layout-section {}", class)}
            data-layout="section"
            data-layout-version="1"
        >
            <div class="layout-section-header"
                data-layout-region="header"
                data-region-hint="Drop section title"
                data-region-meta="Content · max 1">
                {header.map(|h| h())}
            </div>
            <div class="layout-section-main"
                data-layout-region="main"
                data-region-hint="Drop section content"
                data-region-meta="Content · ∞">
                {children()}
            </div>
            <div class="layout-section-footer"
                data-layout-region="footer"
                data-region-hint="Drop section footer"
                data-region-meta="Content · max 1">
                {footer.map(|f| f())}
            </div>
        </section>
    }
}
