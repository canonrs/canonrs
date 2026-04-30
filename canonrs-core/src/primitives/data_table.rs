//! @canon-level: strict
//! @canon-owner: primitives-team
//! DataTable Primitive - HTML puro

use leptos::prelude::*;
use crate::primitives::table::SortDirection;
use crate::meta::SelectionState;
use crate::infra::state_engine::selection_attrs;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum DataTableDensity {
    Compact,
    #[default]
    Comfortable,
    Spacious,
}

impl DataTableDensity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Compact     => "compact",
            Self::Comfortable => "comfortable",
            Self::Spacious    => "spacious",
        }
    }
}










#[component]
pub fn DataTablePrimitive(
    children: Children,
    #[prop(default = DataTableDensity::Comfortable)] density: DataTableDensity,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-datatable=""
            data-rs-uid=crate::infra::uid::generate("dt")
            data-rs-interaction="data"
            data-rs-density=density.as_str()
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DataTableBulkBarPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-datatable-bulk-bar="" hidden class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn DataTableToolbarPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-datatable-toolbar="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn DataTableScrollPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-datatable-scroll="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn DataTableTablePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <table data-rs-datatable-table="" class=class>
            {children()}
        </table>
    }
}

#[component]
pub fn DataTableHeadPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <thead data-rs-datatable-head="" data-rs-resize-container="" class=class>
            {children()}
        </thead>
    }
}

#[component]
pub fn DataTableHeadRowPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <tr data-rs-datatable-head-row="" class=class>
            {children()}
        </tr>
    }
}

#[component]
pub fn DataTableHeadCellPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] sort_key: String,
    #[prop(default = SortDirection::None)] sort_direction: SortDirection,
    #[prop(into, default = String::new())] col_index: String,
    #[prop(into, default = String::new())] style: String,
) -> impl IntoView {
    view! {
        <th
            data-rs-datatable-head-cell=""
            scope="col"
            role="columnheader"
            aria-sort=sort_direction.aria_sort()
            data-rs-sort=sort_direction.as_str()
            data-rs-sort-key=sort_key
            data-rs-col-index=col_index
            style={(!style.is_empty()).then(|| style)}
            class=class
        >
            {children()}
        </th>
    }
}

#[component]
pub fn DataTableBodyPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <tbody data-rs-datatable-body="" class=class>
            {children()}
        </tbody>
    }
}

#[component]
pub fn DataTableRowPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] row_id: String,
    #[prop(into, default = String::new())] row_label: String,
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
    #[prop(optional)] row_index: Option<usize>,
) -> impl IntoView {
    let s = selection_attrs(selected);
    view! {
        <tr
            data-rs-datatable-row=""
            data-rs-state=s.data_rs_state
            data-rs-row-id={(!row_id.is_empty()).then_some(row_id)}
            data-rs-row-label={(!row_label.is_empty()).then_some(row_label)}
            aria-selected=s.aria_selected
            aria-rowindex={row_index.map(|i| (i + 1).to_string())}
            data-rs-row-index={row_index.map(|i| i.to_string())}
            class=class
        >
            {children()}
        </tr>
    }
}

#[component]
pub fn DataTableCellPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] col_index: String,
    #[prop(into, default = String::new())] style: String,
) -> impl IntoView {
    view! {
        <td
            data-rs-datatable-cell=""
            data-rs-col-index=col_index
            style={(!style.is_empty()).then(|| style)}
            class=class
        >
            {children()}
        </td>
    }
}

#[component]
pub fn DataTableFooterPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <tfoot data-rs-datatable-footer="" class=class>
            {children()}
        </tfoot>
    }
}

#[component]
pub fn DataTablePaginationPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-datatable-pagination="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn DataTableEmptyPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-datatable-empty=""
            data-rs-state="empty"
            role="status"
            aria-live="polite"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DataTableLoadingPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-datatable-loading=""
            data-rs-state="loading"
            role="status"
            aria-live="polite"
            class=class
        >
            {children()}
        </div>
    }
}
