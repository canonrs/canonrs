#![allow(unreachable_pub, dead_code)]
//! @canon-level: strict
//! @canon-owner: primitives-team
//! CopyButton UI - SSR wrapper

use leptos::prelude::*;
use canonrs_core::primitives::CopyButtonPrimitive;

#[component]
pub fn CopyButton(
    #[prop(optional, into)] text: Option<String>,
    #[prop(optional, into)] target: Option<String>,
    #[prop(default = 2000)] reset_delay: u32,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional, into)] id: Option<String>,
    #[prop(into, default = "Copy to clipboard".to_string())] aria_label: String,
) -> impl IntoView {
    view! {
        <CopyButtonPrimitive
            class=class
            id=id.unwrap_or_default()
            text=text.unwrap_or_default()
            target=target.unwrap_or_default()
            reset_delay=reset_delay
            aria_label=aria_label
        />
    }
}

#[component]
pub fn CopyButtonPreview() -> impl IntoView {
    view! { <CopyButton id="copy-preview" text="Copy me" /> }
}
