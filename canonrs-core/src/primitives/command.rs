//! @canon-level: strict
//! @canon-owner: primitives-team
//! Command Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{SelectionState, DisabledState, ActivityState, VisibilityState};
use crate::infra::state_engine::{selection_attrs, disabled_attrs, activity_attrs, visibility_attrs};

#[component]
pub fn CommandPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-command=""
            data-rs-component="Command"
            data-rs-behavior="overlay"
            data-rs-state=s.data_rs_state
            role="listbox"
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
    view! {
        <div
            data-rs-command-list=""
            role="group"
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
    let sel = selection_attrs(selected);
    let d = disabled_attrs(disabled);
    let a = activity_attrs(highlighted);
    view! {
        <div
            data-rs-command-item=""
            data-rs-value=value
            data-rs-state=a.data_rs_state
            data-rs-selected=sel.data_rs_state
            data-rs-disabled=d.data_rs_disabled
            role="option"
            aria-selected=sel.aria_selected
            aria-disabled=d.aria_disabled
            tabindex=if d.disabled { "-1" } else { "0" }
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
