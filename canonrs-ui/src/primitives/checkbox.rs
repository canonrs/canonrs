//! @canon-level: strict
//! Checkbox Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn CheckboxPrimitive(
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
            data-checkbox
            type="button"
            role="checkbox"
            tabindex="0"
            aria-checked={if checked { "true" } else { "false" }}
            aria-disabled={if disabled { "true" } else { "false" }}
            attr:data-state={if checked { "checked" } else { "unchecked" }}
            attr:data-disabled={if disabled { Some("true") } else { None }}
            attr:data-name={name}
            attr:data-value={value}
            id={id}
            class={class}
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn CheckboxIndicatorPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span
            data-checkbox-indicator
            class={class}
        >
            {children.map(|c| c())}
        </span>
    }
}
