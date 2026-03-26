//! @canon-level: strict
//! @canon-owner: primitives-team
//! DataTable Primitive - HTML puro

use leptos::prelude::*;
use crate::primitives::table::SortDirection;

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
            Self::Compact => "compact",
            Self::Comfortable => "comfortable",
            Self::Spacious => "spacious",
        }
    }
}

#[component]
pub fn DataTablePrimitive(
    children: Children,
    #[prop(default = DataTableDensity::Comfortable)] density: DataTableDensity,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-datatable=""
            data-rs-density=density.as_str()
            class=class
            id=id.filter(|s| !s.is_empty())
        >
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
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <thead data-rs-datatable-head="" data-rs-resize-container="" class=class id=id.filter(|s| !s.is_empty())>
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
            aria-sort=sort_direction.aria_sort()
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
    #[prop(default = false)] selected: bool,
    #[prop(optional)] row_index: Option<usize>,
) -> impl IntoView {
    let dom_id = if !row_id.is_empty() && !row_id.starts_with("row-") {
        format!("row-{}", row_id)
    } else {
        row_id.clone()
    };

    view! {
        <tr
            data-rs-datatable-row=""
            data-rs-row-id={(!row_id.is_empty()).then_some(row_id)}
            data-rs-selected={if selected { Some("true") } else { None }}
            aria-selected={if selected { Some("true") } else { None }}
            aria-rowindex={row_index.map(|i| (i + 1).to_string())}
            data-rs-row-index={row_index.map(|i| i.to_string())}
            id={(!dom_id.is_empty()).then_some(dom_id)}
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
        <div data-rs-datatable-empty="" data-rs-empty="true" role="status" aria-live="polite" class=class>
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
        <div data-rs-datatable-loading="" data-rs-loading="true" role="status" aria-live="polite" class=class>
            {children()}
        </div>
    }
}
