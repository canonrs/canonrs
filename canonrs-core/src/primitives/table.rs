//! @canon-level: strict
//! @canon-owner: primitives-team
//! Table Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::SelectionState;
use crate::infra::state_engine::selection_attrs;


#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum TableState {
    #[default]
    Idle,
    Loading,
    Empty,
    Error,
}
impl TableState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Idle    => "idle",
            Self::Loading => "loading",
            Self::Empty   => "empty",
            Self::Error   => "error",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum SortDirection {
    #[default]
    None,
    Ascending,
    Descending,
}
impl SortDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None       => "none",
            Self::Ascending  => "ascending",
            Self::Descending => "descending",
        }
    }
    pub fn aria_sort(&self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::Ascending  => Some("ascending"),
            Self::Descending => Some("descending"),
        }
    }
}

#[component]
pub fn TableWrapperPrimitive(
    children: Children,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-table-wrapper=""
            role="region"
            aria-label=aria_label
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn TablePrimitive(
    children: Children,
    #[prop(default = TableState::Idle)] state: TableState,
    #[prop(default = false)] striped: bool,
    #[prop(default = false)] hoverable: bool,
    #[prop(default = false)] sheet_context: bool,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <table
            data-rs-table=""
            data-rs-uid=crate::infra::uid::generate("tbl")
            data-rs-interaction="init"
            data-rs-state=state.as_str()
            data-rs-striped={striped.then_some("")}
            data-rs-hoverable={hoverable.then_some("")}
            data-rs-table-context={sheet_context.then_some("")}
            aria-busy={if state == TableState::Loading { Some("true") } else { None }}
            aria-label=aria_label
            class=class
            tabindex="-1"
        >
            {children()}
        </table>
    }
}

#[component]
pub fn TableHeaderPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <thead data-rs-table-header="" class=class>
            {children()}
        </thead>
    }
}

#[component]
pub fn TableBodyPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <tbody data-rs-table-body="" class=class>
            {children()}
        </tbody>
    }
}

#[component]
pub fn TableFooterPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <tfoot data-rs-table-footer="" class=class>
            {children()}
        </tfoot>
    }
}

#[component]
pub fn TableRowPrimitive(
    children: Children,
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
    #[prop(into, default = TextProp::from(""))] class: TextProp,
    #[prop(into, default = String::new())] href: String,
    #[prop(into, default = String::new())] row_action: String,
    #[prop(into, default = String::new())] row_label: String,
    #[prop(into, default = String::new())] row_meta: String,
) -> impl IntoView {
    let s = selection_attrs(selected);
    let action_attr = if !row_action.is_empty() { Some(row_action.clone()) } else if !href.is_empty() { Some("navigate".to_string()) } else { None };
    let href_attr = if href.is_empty() { None } else { Some(href) };
    let row_label = if row_label.is_empty() { None } else { Some(row_label) };
    let row_meta = if row_meta.is_empty() { None } else { Some(row_meta) };
    view! {
        <tr
            data-rs-table-row=""
            data-rs-state=s.data_rs_state
            tabindex="0"
            data-rs-action=action_attr
            data-rs-href=href_attr
            data-rs-label=row_label
            data-rs-meta=row_meta
            role="row"
            aria-selected=s.aria_selected
            class=move || class.get().to_string()
        >
            {children()}
        </tr>
    }
}

#[component]
pub fn TableHeadPrimitive(
    children: Children,
    #[prop(default = SortDirection::None)] sort: SortDirection,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <th
            data-rs-table-head=""
            data-rs-sort=sort.as_str()
            scope="col"
            role="columnheader"
            aria-sort=sort.aria_sort()
            class=class
        >
            {children()}
        </th>
    }
}

#[component]
pub fn TableCellPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = false)] copyable: bool,
    #[prop(default = false)] truncate: bool,
) -> impl IntoView {
    view! {
        <td
            data-rs-table-cell=""
            data-rs-copyable={copyable.then(|| "")}
            data-rs-truncate={truncate.then(|| "")}
            class=class
        >
            {children()}
        </td>
    }
}

#[component]
pub fn TableCaptionPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <caption data-rs-table-caption="" class=class>
            {children()}
        </caption>
    }
}
