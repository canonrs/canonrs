//! @canon-level: strict
//! @canon-owner: primitives-team
//! Calendar Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn CalendarPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-calendar=""
            class=class
            id=id.filter(|s| !s.is_empty())
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
    #[prop(optional)] id: Option<String>,
    #[prop(optional)] aria_labelledby: Option<String>,
) -> impl IntoView {
    let aria_labelledby = aria_labelledby.filter(|s| !s.is_empty());
    view! {
        <table
            data-rs-calendar-grid=""
            role="grid"
            aria-labelledby=aria_labelledby
            class=class
            id=id.filter(|s| !s.is_empty())
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
    #[prop(optional)] id: Option<String>,
    #[prop(default = false)] selected: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] active: bool,
) -> impl IntoView {
    view! {
        <td
            data-rs-calendar-cell=""
            role="gridcell"
            aria-selected={if selected { Some("true") } else { None }}
            aria-disabled={if disabled { Some("true") } else { None }}
            data-rs-selected={if selected { Some("true") } else { None }}
            data-rs-disabled={if disabled { Some("true") } else { None }}
            tabindex={if active { "0" } else { "-1" }}
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </td>
    }
}
