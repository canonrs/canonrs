//! # Header Block
//! Canon Rule: Header é BLOCK — slots explícitos, zero layout, zero lógica

use leptos::prelude::*;

#[component]
pub fn Header(
    #[prop(optional)] logo: Option<ChildrenFn>,
    #[prop(optional)] primary_nav: Option<ChildrenFn>,
    #[prop(optional)] actions: Option<ChildrenFn>,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <header
            class=format!("block-header {}", class)
            data-block="header"
            data-block-version="1"
        >
            <div data-block-region="logo">{logo.map(|l| l())}</div>
            <nav data-block-region="nav">{primary_nav.map(|n| n())}</nav>
            <div data-block-region="center">{children.map(|c| c())}</div>
            <div data-block-region="actions">{actions.map(|a| a())}</div>
        </header>
    }
}
