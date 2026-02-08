//! # MarketingLayout
//! **Purpose:** Public-facing marketing pages

use leptos::prelude::*;
use crate::blocks::header::Header;

#[component]
pub fn MarketingLayout(
    header_logo: Children,
    header_nav: Children,
    header_actions: Children,
    #[prop(optional)]
    hero: Option<Children>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class="layout-marketing"
            data-layout="marketing"
        >
            <Header
                logo=header_logo
                primary_nav=header_nav
                actions=header_actions
            />

            {hero.map(|h| view! {
                <section class="layout-marketing-hero" data-layout-slot="hero">
                    {h()}
                </section>
            })}

            <main class="layout-marketing-main" data-layout-slot="main">
                {children()}
            </main>

            <footer class="layout-marketing-footer" data-layout-slot="footer">
                "Footer placeholder"
            </footer>
        </div>
    }
}
