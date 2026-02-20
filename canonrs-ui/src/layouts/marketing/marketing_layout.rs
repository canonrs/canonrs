//! # MarketingLayout
//! **Purpose:** Public-facing marketing pages

use leptos::prelude::*;
use crate::blocks::header::Header;

#[component]
pub fn MarketingLayout(
    header_logo: ChildrenFn,
    header_nav: ChildrenFn,
    header_actions: ChildrenFn,
    #[prop(optional)]
    hero: Option<ChildrenFn>,
    #[prop(optional)]
    footer: Option<ChildrenFn>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="layout-marketing" data-layout="marketing">
            <Header logo=header_logo primary_nav=header_nav actions=header_actions />
            {hero.map(|h| view! {
                <section class="layout-marketing-hero" data-layout-slot="hero">{h()}</section>
            })}
            <main class="layout-marketing-main" data-layout-slot="main">{children()}</main>
            {footer.map(|f| view! {
                <footer class="layout-marketing-footer" data-layout-slot="footer">{f()}</footer>
            })}
        </div>
    }
}
