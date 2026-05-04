//! @canon-level: strict
//! @canon-owner: primitives-team
//! List Primitives - Accessible list container + items

use leptos::prelude::*;
use crate::meta::{SelectionState, DisabledState, ActivityState};

#[component]
pub fn ListPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] aria_label: Option<String>,
) -> impl IntoView {
    let uid_lst = crate::infra::uid::generate("lst");
    view! {
        <div
            data-rs-list=""
            data-rs-uid=uid_lst
            data-rs-interaction="data"
            role="listbox"
            aria-label=aria_label
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ListItemPrimitive(
    children: Children,
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
    #[prop(default = ActivityState::Inactive)] focused: ActivityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let tabindex = if focused.as_str() == "active" { "0" } else { "-1" };
    view! {
        <div
            data-rs-list-item=""
            data-rs-selection=if selected == SelectionState::Selected { Some("selected") } else { None }
            data-rs-focused=focused.as_str()
            data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
            role="option"
            tabindex=tabindex
            aria-selected=if selected == SelectionState::Selected { Some("true") } else { None }
            aria-disabled=disabled.aria_disabled()
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ListItemTitlePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-list-item-title="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn ListItemDescriptionPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-list-item-description="" class=class>
            {children()}
        </div>
    }
}
