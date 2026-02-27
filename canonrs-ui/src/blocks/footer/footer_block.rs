//! # Footer Block
//! Canon Rule: Footer é BLOCK — slots explícitos, zero layout, zero lógica

use leptos::prelude::*;

#[component]
pub fn Footer(
    #[prop(optional)] left: Option<ChildrenFn>,
    #[prop(optional)] center: Option<ChildrenFn>,
    #[prop(optional)] right: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <footer
            class=format!("block-footer {}", class)
            data-block="footer"
            data-block-version="1"
        >
            <div data-block-region="left">{left.map(|l| l())}</div>
            <div data-block-region="center">{center.map(|c| c())}</div>
            <div data-block-region="right">{right.map(|r| r())}</div>
        </footer>
    }
}
