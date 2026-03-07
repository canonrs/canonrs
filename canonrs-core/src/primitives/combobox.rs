//! @canon-level: strict
//! @canon-owner: primitives-team
//! Combobox Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn ComboboxPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] expanded: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-combobox
            role="combobox"
            aria-expanded={if expanded { "true" } else { "false" }}
            aria-disabled={if disabled { "true" } else { "false" }}
            aria-haspopup="listbox"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ComboboxTriggerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            data-combobox-trigger
            type="button"
            role="button"
            aria-disabled={if disabled { "true" } else { "false" }}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn ComboboxListPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-combobox-list
            role="listbox"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ComboboxItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] selected: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-combobox-item
            role="option"
            aria-selected={if selected { "true" } else { "false" }}
            aria-disabled={if disabled { "true" } else { "false" }}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}
