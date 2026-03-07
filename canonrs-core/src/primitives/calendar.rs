//! @canon-level: strict
//! @canon-owner: primitives-team
//! Calendar Primitive - HTML puro + ARIA

use leptos::prelude::*;
use leptos::html;

#[component]
pub fn CalendarPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] node_ref: NodeRef<html::Div>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            data-calendar=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CalendarHeaderPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-calendar-header=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CalendarGridPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(optional)] aria_labelledby: Option<String>,
) -> impl IntoView {
    view! {
        <table
            data-calendar-grid=""
            role="grid"
            class={class}
            id={id}
            attr:aria-labelledby={aria_labelledby}
        >
            {children.map(|c| c())}
        </table>
    }
}

#[component]
pub fn CalendarGridHeadPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <thead
            data-calendar-grid-head=""
            class={class}
        >
            {children.map(|c| c())}
        </thead>
    }
}

#[component]
pub fn CalendarGridBodyPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <tbody
            data-calendar-grid-body=""
            class={class}
        >
            {children.map(|c| c())}
        </tbody>
    }
}

#[component]
pub fn CalendarGridRowPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <tr
            data-calendar-grid-row=""
            class={class}
        >
            {children.map(|c| c())}
        </tr>
    }
}

#[component]
pub fn CalendarHeadCellPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <th
            data-calendar-head-cell=""
            scope="col"
            class={class}
        >
            {children.map(|c| c())}
        </th>
    }
}

#[component]
pub fn CalendarCellPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(default = false)] selected: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(default = "-1".to_string())] tabindex: String,
) -> impl IntoView {
    view! {
        <td
            data-calendar-cell=""
            role="gridcell"
            attr:aria-selected={if selected { "true" } else { "false" }}
            attr:aria-disabled={if disabled { "true" } else { "false" }}
            attr:data-selected={if selected { Some("true") } else { None }}
            attr:data-disabled={if disabled { Some("true") } else { None }}
            tabindex={tabindex}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </td>
    }
}
