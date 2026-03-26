//! # MarketingLayout — Regions: header, hero, content, footer
use leptos::prelude::*;

#[component]
pub fn MarketingLayout(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] hero: Option<ChildrenFn>,
    #[prop(optional)] content: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div data-layout="marketing" data-layout-version="1" class=class>
            {header.map(|h| view! { <div data-layout-region="header">{h()}</div> })}
            {hero.map(|h| view! { <div data-layout-region="hero">{h()}</div> })}
            {content.map(|c| view! { <div data-layout-region="content">{c()}</div> })}
            {footer.map(|f| view! { <div data-layout-region="footer">{f()}</div> })}
        </div>
    }
}
