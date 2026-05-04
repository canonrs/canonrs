//! @canon-level: strict
//! @canon-owner: primitives-team
//! Command Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{SelectionState, DisabledState, ActivityState, VisibilityState};

#[component]
pub fn CommandPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_cmd = crate::infra::uid::generate("cmd");
    let listbox_id = crate::infra::uid::generate("cmd-list");

    view! {
        <div
            data-rs-command=""
            data-rs-uid=uid_cmd
            data-rs-interaction="init"
            data-rs-visibility=state.as_str()
            data-rs-listbox-id=listbox_id.clone()
            role="dialog"
            aria-label="Command palette"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CommandInputPrimitive(
    #[prop(into, default = String::new())] placeholder: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-command-input-wrapper="" class=class>
            <input
                data-rs-command-input=""
                type="text"
                role="combobox"
                aria-autocomplete="list"
                aria-expanded="false"
                aria-haspopup="listbox"
                placeholder=if placeholder.is_empty() { None } else { Some(placeholder) }
            />
        </div>
    }
}

#[component]
pub fn CommandListPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let list_id = crate::infra::uid::generate("cmd-list");
    view! {
        <div
            data-rs-command-list=""
            id=list_id
            role="listbox"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CommandEmptyPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-command-empty=""
            role="status"
            aria-live="polite"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CommandGroupPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-command-group=""
            role="group"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CommandGroupHeadingPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-command-group-heading=""
            role="presentation"
            aria-hidden="true"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CommandItemPrimitive(
    children: Children,
    #[prop(into, default = String::new())] value: String,
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = ActivityState::Inactive)] highlighted: ActivityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-command-item=""
            data-rs-value=value
            data-rs-activity=highlighted.as_str()
            data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
            id=crate::infra::uid::generate("cmd-item")
            role="option"
            aria-selected=if selected == SelectionState::Selected { Some("true") } else { None }
            aria-disabled=disabled.aria_disabled()
            tabindex=if disabled.disabled() { "-1" } else { "0" }
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CommandSeparatorPrimitive(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-command-separator=""
            role="separator"
            aria-orientation="horizontal"
            class=class
        />
    }
}
