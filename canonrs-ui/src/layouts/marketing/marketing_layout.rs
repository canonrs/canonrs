//! # MarketingLayout — Regions: header, hero, main, footer (always rendered)
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
            <header class="layout-marketing-header"
                data-layout-region="header"
                data-region-hint="Drop logo, nav or actions"
                data-region-meta="Nav · max 1">
                {header.map(|h| h())}
            </header>
            <section class="layout-marketing-hero"
                data-layout-region="hero"
                data-region-hint="Drop hero banner"
                data-region-meta="Any · max 1">
                {hero.map(|h| h())}
            </section>
            <main class="layout-marketing-main"
                data-layout-region="main"
                data-region-hint="Drop page sections"
                data-region-meta="Content · ∞">
                {children()}
            </main>
            <footer class="layout-marketing-footer"
                data-layout-region="footer"
                data-region-hint="Drop footer links"
                data-region-meta="Nav · max 1">
                {footer.map(|f| f())}
            </footer>
        </div>
    }
}
