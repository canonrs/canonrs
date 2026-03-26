//! @canon-level: strict
//! @canon-owner: primitives-team
//! Menu Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{SelectionState, DisabledState, ActivityState};
use crate::state_engine::{selection_attrs, disabled_attrs, activity_attrs};

#[component]
pub fn MenuPrimitive(
    children: Children,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <nav
            data-rs-menu=""
            data-rs-component="Menu"
            data-rs-behavior="navigation"
            aria-label=aria_label
            class=class
        >
            {children()}
        </nav>
    }
}

#[component]
pub fn MenuItemPrimitive(
    children: Children,
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = ActivityState::Inactive)] highlighted: ActivityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let sel = selection_attrs(selected);
    let d = disabled_attrs(disabled);
    let a = activity_attrs(highlighted);
    view! {
        <button
            type="button"
            data-rs-menu-item=""
            role="menuitem"
            data-rs-state=a.data_rs_state
            data-rs-selected=sel.data_rs_state
            data-rs-disabled=d.data_rs_disabled
            aria-selected=sel.aria_selected
            aria-disabled=d.aria_disabled
            tabindex=if d.disabled { "-1" } else { "0" }
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn MenuGroupPrimitive(
    children: Children,
    #[prop(into, optional)] label: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-menu-group=""
            role="group"
            aria-label=label
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn MenuLabelPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-menu-label=""
            role="presentation"
            aria-hidden="true"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn MenuSeparatorPrimitive(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-menu-separator="" role="separator" class=class />
    }
}
