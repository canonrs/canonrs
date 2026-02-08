//! # AppShellLayout
//! **Purpose:** Standard SaaS applications without sidebar

use leptos::prelude::*;
use crate::blocks::header::Header;

#[component]
pub fn AppShellLayout(
    header_logo: Children,
    header_nav: Children,
    header_actions: Children,
    #[prop(default = false)]
    show_footer: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class="layout-app-shell"
            data-layout="app-shell"
        >
            <Header
                logo=header_logo
                primary_nav=header_nav
                actions=header_actions
            />

            <main
                class="layout-app-content"
                data-layout-slot="content"
            >
                {children()}
            </main>

            {show_footer.then(|| view! {
                <footer class="layout-app-footer" data-layout-slot="footer">
                    "Footer placeholder"
                </footer>
            })}
        </div>
    }
}
