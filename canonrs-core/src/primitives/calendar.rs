//! @canon-level: strict
//! @canon-owner: primitives-team
//! Calendar Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{SelectionState, DisabledState, ActivityState};

#[component]
pub fn CalendarPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_cal = crate::infra::uid::generate("cal");
    view! {
        <div
            data-rs-calendar=""
            data-rs-uid=uid_cal
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
    view! {
        <td
            data-rs-calendar-cell=""
            role="gridcell"
            aria-selected=if selected == SelectionState::Selected { Some("true") } else { None }
            aria-disabled=disabled.aria_disabled()
            data-rs-selection=if selected == SelectionState::Selected { Some("selected") } else { None }
            data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
            data-rs-activity=activity.as_str()
            tabindex=if activity == ActivityState::Active { "0" } else { "-1" }
            class=class
        >
            {children()}
        </td>
    }
}
