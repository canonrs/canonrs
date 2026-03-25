//! @canon-level: strict
//! @canon-owner: primitives-team
//! Combobox Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn ComboboxPrimitive(
    children: Children,
    #[prop(default = false)] expanded: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
    #[prop(optional)] list_id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-combobox=""
            data-rs-state={if expanded { "open" } else { "closed" }}
            role="combobox"
            aria-expanded={if expanded { "true" } else { "false" }}
            aria-disabled={if disabled { Some("true") } else { None }}
            aria-haspopup="listbox"
            aria-controls=list_id.filter(|s| !s.is_empty())
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ComboboxTriggerPrimitive(
    children: Children,
    #[prop(default = false)] expanded: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
    #[prop(optional)] list_id: Option<String>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-combobox-trigger=""
            aria-haspopup="listbox"
            aria-expanded={if expanded { "true" } else { "false" }}
            aria-disabled={if disabled { Some("true") } else { None }}
            aria-controls=list_id.filter(|s| !s.is_empty())
            disabled=disabled
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </button>
    }
}

#[component]
pub fn ComboboxListPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-combobox-list=""
            role="listbox"
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ComboboxItemPrimitive(
    children: Children,
    #[prop(default = false)] selected: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-combobox-item=""
            data-rs-disabled={if disabled { Some("true") } else { None }}
            data-rs-selected={if selected { Some("true") } else { None }}
            role="option"
            aria-selected={if selected { Some("true") } else { None }}
            aria-disabled={if disabled { Some("true") } else { None }}
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </div>
    }
}
