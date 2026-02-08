//! # Footer Block
//!
//! Canon Rule:
//! - Footer é BLOCK
//! - Slots explícitos
//! - ZERO layout
//! - ZERO lógica

use leptos::prelude::*;

#[component]
pub fn Footer(
    #[prop(optional)] left: Option<Children>,
    #[prop(optional)] center: Option<Children>,
    #[prop(optional)] right: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <footer
            class={format!("block-footer {}", class)}
            data-block="footer"
        >
            {left.map(|l| view! {
                <div data-slot="left">{l()}</div>
            })}

            {center.map(|c| view! {
                <div data-slot="center">{c()}</div>
            })}

            {right.map(|r| view! {
                <div data-slot="right">{r()}</div>
            })}
        </footer>
    }
}
