//! @canon-level: strict
//! @canon-owner: primitives-team
//! Switch Primitive - HTML puro + ARIA
//! Base: Checkbox com role="switch"

use leptos::prelude::*;

#[component]
pub fn SwitchPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] name: String,
    #[prop(default = String::new())] value: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            data-switch
            type="button"
            role="switch"
            tabindex="0"
            aria-checked={if checked { "true" } else { "false" }}
            aria-disabled={if disabled { "true" } else { "false" }}
            data-state={if checked { "checked" } else { "unchecked" }}
            data-disabled={if disabled { Some("true") } else { None }}
            data-name={name}
            data-value={value}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn SwitchThumbPrimitive(
    #[prop(default = String::new())] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <span data-switch-thumb class={class}>{children.map(|c| c())}</span>
    }
}
