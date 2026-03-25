//! @canon-level: strict
//! @canon-owner: primitives-team
//! Command Primitive - HTML puro para command palette

use leptos::prelude::*;

#[component]
pub fn CommandPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-command=""
            role="listbox"
            aria-label="Command palette"
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
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
                placeholder=placeholder.filter(|s| !s.is_empty())
                id=id.filter(|s| !s.is_empty())
            />
        </div>
    }
}

#[component]
pub fn CommandListPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-command-list=""
            role="group"
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CommandEmptyPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-command-empty=""
            role="status"
            aria-live="polite"
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CommandGroupPrimitive(
    children: Children,
    #[prop(optional)] heading: Option<String>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-command-group=""
            role="group"
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {heading.map(|h| view! {
                <div data-rs-command-group-heading="" role="presentation">{h}</div>
            })}
            {children()}
        </div>
    }
}

#[component]
pub fn CommandItemPrimitive(
    children: Children,
    #[prop(optional)] value: Option<String>,
    #[prop(default = false)] selected: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-command-item=""
            data-rs-value=value
            data-rs-selected={if selected { Some("true") } else { None }}
            role="option"
            aria-selected={if selected { Some("true") } else { None }}
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
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
            id=id.filter(|s| !s.is_empty())
        />
    }
}
