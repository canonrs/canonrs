//! @canon-level: strict
//! CopyButton Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::copy_button_ui::CopyButton;

#[island]
pub fn CopyButtonInit() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
                use wasm_bindgen_futures::spawn_local;
        spawn_local(async move {
        });
    }
    view! { <></> }
}

#[component]
pub fn CopyButtonIsland(
    #[prop(optional, into)] text: Option<String>,
    #[prop(optional, into)] target: Option<String>,
    #[prop(default = 2000u32)] reset_delay: u32,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional, into)] id: Option<String>,
    #[prop(into, default = "Copy to clipboard".to_string())] aria_label: String,
) -> impl IntoView {
    view! {
        <CopyButtonInit />
        <CopyButton
            text=text.unwrap_or_default()
            target=target.unwrap_or_default()
            reset_delay=reset_delay
            class=class
            id=id.unwrap_or_default()
            aria_label=aria_label
        />
    }
}
