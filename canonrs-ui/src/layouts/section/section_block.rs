//! # Section — Regions: header, main, footer
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
            id=if id.is_empty() { None } else { Some(id) }
            class=format!("layout-section {}", class)
            data-layout="section"
            data-layout-version="1">
            <div class="layout-section-header" data-layout-region="header">{header.map(|h| h())}</div>
            <div class="layout-section-main" data-layout-region="main">{children()}</div>
            <div class="layout-section-footer" data-layout-region="footer">{footer.map(|f| f())}</div>
        </section>
    }
}
