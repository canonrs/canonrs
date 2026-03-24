//! @canon-level: strict
//! @canon-owner: primitives-team
//! Command Primitive - HTML puro para command palette

use leptos::prelude::*;

#[component]
pub fn CommandPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-command=""
            role="listbox"
            aria-label="Command palette"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CommandInputPrimitive(
    #[prop(optional)] placeholder: Option<String>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div data-rs-command-input-wrapper="" class=class>
            <input
                data-rs-command-input=""
                type="text"
                role="combobox"
                aria-autocomplete="list"
                aria-expanded="true"
                placeholder=placeholder
                id=id
            />
        </div>
    }
}

#[component]
pub fn CommandListPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-command-list=""
            role="group"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CommandEmptyPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-command-empty=""
            role="status"
            aria-live="polite"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CommandGroupPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] heading: Option<String>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-command-group=""
            role="group"
            class=class
            id=id
        >
            {heading.map(|h| view! {
                <div data-rs-command-group-heading="" role="presentation">{h}</div>
            })}
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CommandItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] value: Option<String>,
    #[prop(default = false)] selected: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-command-item=""
            data-rs-value=value.unwrap_or_default()
            data-rs-selected={if selected { "true" } else { "false" }}
            role="option"
            aria-selected={if selected { "true" } else { "false" }}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CommandSeparatorPrimitive(
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-command-separator=""
            role="separator"
            aria-orientation="horizontal"
            class=class
            id=id
        />
    }
}
