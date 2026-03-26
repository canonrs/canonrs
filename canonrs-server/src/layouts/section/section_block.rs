//! # Section — Regions: header, content, footer
use leptos::prelude::*;

#[component]
pub fn Section(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] content: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <section data-layout="section" data-layout-version="1" class=class>
            {header.map(|h| view! { <div data-layout-region="header">{h()}</div> })}
            {content.map(|c| view! { <div data-layout-region="content">{c()}</div> })}
            {footer.map(|f| view! { <div data-layout-region="footer">{f()}</div> })}
        </section>
    }
}
