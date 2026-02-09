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
    let state = if checked { "checked" } else { "unchecked" };
    let disabled_attr = if disabled { Some("true") } else { None };
    
    view! {
        <button
            data-checkbox=""
            data-state={state}
            data-disabled={disabled_attr}
            data-name={name}
            data-value={value}
            type="button"
            role="checkbox"
            tabindex="0"
            aria-checked={if checked { "true" } else { "false" }}
            aria-disabled={if disabled { "true" } else { "false" }}
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
        <span data-checkbox-indicator="" class={class}>
            {children.map(|c| c())}
        </span>
    }
}
