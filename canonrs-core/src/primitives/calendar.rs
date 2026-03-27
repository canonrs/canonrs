//! @canon-level: strict
//! @canon-owner: primitives-team
//! Calendar Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{SelectionState, DisabledState, ActivityState};
use crate::infra::state_engine::{selection_attrs, disabled_attrs, activity_attrs};

#[component]
pub fn CalendarPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-calendar=""
            data-rs-component="Calendar"
            data-rs-behavior="selection"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CalendarHeaderPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-calendar-header="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CalendarGridPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional, into)] aria_labelledby: Option<String>,
) -> impl IntoView {
    view! {
        <table
            data-rs-calendar-grid=""
            role="grid"
            aria-labelledby=aria_labelledby
            class=class
        >
            {children()}
        </table>
    }
}

#[component]
pub fn CalendarGridHeadPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <thead data-rs-calendar-grid-head="" class=class>
            {children()}
        </thead>
    }
}

#[component]
pub fn CalendarGridBodyPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <tbody data-rs-calendar-grid-body="" class=class>
            {children()}
        </tbody>
    }
}

#[component]
pub fn CalendarGridRowPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <tr data-rs-calendar-grid-row="" class=class>
            {children()}
        </tr>
    }
}

#[component]
pub fn CalendarHeadCellPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <th data-rs-calendar-head-cell="" scope="col" class=class>
            {children()}
        </th>
    }
}

#[component]
pub fn CalendarCellPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = ActivityState::Inactive)] activity: ActivityState,
) -> impl IntoView {
    let sel = selection_attrs(selected);
    let d   = disabled_attrs(disabled);
    let act = activity_attrs(activity);
    view! {
        <td
            data-rs-calendar-cell=""
            role="gridcell"
            aria-selected=sel.aria_selected
            aria-disabled=d.aria_disabled
            data-rs-selected=sel.data_rs_state
            data-rs-disabled=d.data_rs_disabled
            data-rs-activity=act.data_rs_state
            tabindex=if activity == ActivityState::Active { "0" } else { "-1" }
            class=class
        >
            {children()}
        </td>
    }
}
