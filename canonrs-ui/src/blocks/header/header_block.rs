//! # Header Block
//!
//! Canon Rule:
//! - Header é BLOCK (composição semântica)
//! - Slots explícitos
//! - ZERO layout, ZERO lógica

use leptos::prelude::*;

#[component]
pub fn Header(
    #[prop(optional)] logo: Option<ChildrenFn>,
    #[prop(optional)] primary_nav: Option<ChildrenFn>,
    #[prop(optional)] actions: Option<ChildrenFn>,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <header
            class={format!("block-header {}", class)}
            data-block="header"
        >
            {logo.map(|l| view! {
                <div data-slot="logo">{l()}</div>
            })}

            {primary_nav.map(|n| view! {
                <nav data-slot="primary-nav">{n()}</nav>
            })}

            {children.map(|c| view! {
                <div data-slot="center">{c()}</div>
            })}

            {actions.map(|a| view! {
                <div data-slot="actions">{a()}</div>
            })}
        </header>
    }
}
