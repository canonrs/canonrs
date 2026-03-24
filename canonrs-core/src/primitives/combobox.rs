//! @canon-level: strict
//! @canon-owner: primitives-team
//! Combobox Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn ComboboxPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] expanded: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-combobox=""
            data-rs-state={if expanded { "open" } else { "closed" }}
            role="combobox"
            aria-expanded={if expanded { "true" } else { "false" }}
            aria-disabled={if disabled { "true" } else { "false" }}
            aria-haspopup="listbox"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ComboboxTriggerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] expanded: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-combobox-trigger=""
            aria-haspopup="listbox"
            aria-expanded={if expanded { "true" } else { "false" }}
            aria-disabled={if disabled { "true" } else { "false" }}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn ComboboxListPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-combobox-list=""
            role="listbox"
            class=class
            id=id
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
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-combobox-item=""
            data-rs-state={if disabled { "disabled" } else { "closed" }}
            data-rs-selected={if selected { "true" } else { "false" }}
            role="option"
            aria-selected={if selected { "true" } else { "false" }}
            aria-disabled={if disabled { "true" } else { "false" }}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}
