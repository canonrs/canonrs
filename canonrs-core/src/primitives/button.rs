//! @canon-level: strict
//! @canon-owner: primitives-team
//! Button Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn ButtonPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] aria_label: Option<String>,
    #[prop(into, default = String::new())] variant: String,
    #[prop(into, default = String::new())] size: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-button=""
            data-rs-variant=variant
            data-rs-size=size
            disabled=disabled
            aria-disabled={if disabled { "true" } else { "false" }}
            aria-label=aria_label.unwrap_or_default()
            class=class
            id=id.unwrap_or_default()
        >
            {children()}
        </button>
    }
}
