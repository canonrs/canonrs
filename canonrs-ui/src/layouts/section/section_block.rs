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
            id={if id.is_empty() { None } else { Some(id) }}
            class={format!("layout-section {}", class)}
            data-layout="section" data-layout-version="1"
        >
            {header.map(|h| view! {
                <div class="layout-section-header" data-layout-region="header">{h()}</div>
            })}
            <div class="layout-section-main" data-layout-region="main">{children()}</div>
            {footer.map(|f| view! {
                <div class="layout-section-footer" data-layout-region="footer">{f()}</div>
            })}
        </section>
    }
}
