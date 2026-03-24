//! @canon-level: strict
//! @canon-owner: primitives-team
//! Calendar Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn CalendarPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-calendar=""
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CalendarHeaderPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-calendar-header="" class=class>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CalendarGridPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
    #[prop(optional)] aria_labelledby: Option<String>,
) -> impl IntoView {
    view! {
        <table
            data-rs-calendar-grid=""
            role="grid"
            aria-labelledby=aria_labelledby
            class=class
            id=id
        >
            {children.map(|c| c())}
        </table>
    }
}

#[component]
pub fn CalendarGridHeadPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <thead data-rs-calendar-grid-head="" class=class>
            {children.map(|c| c())}
        </thead>
    }
}

#[component]
pub fn CalendarGridBodyPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <tbody data-rs-calendar-grid-body="" class=class>
            {children.map(|c| c())}
        </tbody>
    }
}

#[component]
pub fn CalendarGridRowPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <tr data-rs-calendar-grid-row="" class=class>
            {children.map(|c| c())}
        </tr>
    }
}

#[component]
pub fn CalendarHeadCellPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <th data-rs-calendar-head-cell="" scope="col" class=class>
            {children.map(|c| c())}
        </th>
    }
}

#[component]
pub fn CalendarCellPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
    #[prop(default = false)] selected: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = "-1".to_string())] tabindex: String,
) -> impl IntoView {
    view! {
        <td
            data-rs-calendar-cell=""
            role="gridcell"
            aria-selected={if selected { "true" } else { "false" }}
            aria-disabled={if disabled { "true" } else { "false" }}
            data-rs-selected={if selected { "true" } else { "false" }}
            data-rs-state={if disabled { "disabled" } else { "closed" }}
            tabindex=tabindex
            class=class
            id=id
        >
            {children.map(|c| c())}
        </td>
    }
}
