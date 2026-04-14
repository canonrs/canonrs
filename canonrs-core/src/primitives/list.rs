//! @canon-level: strict
//! @canon-owner: primitives-team
//! List Primitives - Accessible list container + items

use leptos::prelude::*;
use crate::meta::{SelectionState, DisabledState, ActivityState};
use crate::infra::state_engine::{selection_attrs, disabled_attrs, activity_attrs};

#[component]
pub fn ListPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] aria_label: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-list=""
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
    let s = selection_attrs(selected);
    let f = activity_attrs(focused);
    let d = disabled_attrs(disabled);
    let tabindex = if f.data_rs_state == "active" { "0" } else { "-1" };
    view! {
        <div
            data-rs-list-item=""
            data-rs-state=s.data_rs_state
            data-rs-focused=f.data_rs_state
            data-rs-disabled=d.data_rs_disabled
            role="option"
            tabindex=tabindex
            aria-selected=s.aria_selected
            aria-disabled=d.aria_disabled
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
