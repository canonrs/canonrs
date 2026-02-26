//! # MarketingLayout — Regions: header, hero, main, footer
use leptos::prelude::*;

#[component]
pub fn MarketingLayout(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] hero: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="layout-marketing" data-layout="marketing" data-layout-version="1">
            {header.map(|h| view! {
                <header class="layout-marketing-header" data-layout-region="header">{h()}</header>
            })}
            {hero.map(|h| view! {
                <section class="layout-marketing-hero" data-layout-region="hero">{h()}</section>
            })}
            <main class="layout-marketing-main" data-layout-region="main">{children()}</main>
            {footer.map(|f| view! {
                <footer class="layout-marketing-footer" data-layout-region="footer">{f()}</footer>
            })}
        </div>
    }
}
