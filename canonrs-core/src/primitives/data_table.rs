//! @canon-level: strict
//! @canon-owner: primitives-team
//! DataTable Primitive - HTML puro

use leptos::prelude::*;

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
    #[prop(optional)] children: Option<Children>,
    #[prop(default = DataTableDensity::Comfortable)] density: DataTableDensity,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-datatable=""
            data-rs-density=density.as_str()
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DataTableToolbarPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-datatable-toolbar="" class=class>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DataTableScrollPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-datatable-scroll="" class=class>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DataTableTablePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <table data-rs-datatable-table="" class=class>
            {children.map(|c| c())}
        </table>
    }
}

#[component]
pub fn DataTableHeadPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <thead data-rs-datatable-head="" data-rs-resize-container="" class=class id=id>
            {children.map(|c| c())}
        </thead>
    }
}

#[component]
pub fn DataTableHeadRowPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <tr data-rs-datatable-head-row="" class=class>
            {children.map(|c| c())}
        </tr>
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SortDirection {
    Ascending,
    Descending,
    #[default]
    None,
}

impl SortDirection {
    pub fn as_aria_str(&self) -> &'static str {
        match self {
            Self::Ascending => "ascending",
            Self::Descending => "descending",
            Self::None => "none",
        }
    }
}

#[component]
pub fn DataTableHeadCellPrimitive(
    #[prop(optional)] children: Option<Children>,
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
            aria-sort=sort_direction.as_aria_str()
            data-rs-sort-key=sort_key
            data-rs-col-index=col_index
            style={(!style.is_empty()).then(|| style)}
            class=class
        >
            {children.map(|c| c())}
        </th>
    }
}

#[component]
pub fn DataTableBodyPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <tbody data-rs-datatable-body="" class=class>
            {children.map(|c| c())}
        </tbody>
    }
}

#[component]
pub fn DataTableRowPrimitive(
    #[prop(optional)] children: Option<Children>,
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
            data-rs-selected={if selected { "true" } else { "false" }}
            aria-selected={if selected { "true" } else { "false" }}
            aria-rowindex={row_index.map(|i| (i + 1).to_string())}
            data-rs-row-index={row_index.map(|i| i.to_string())}
            id={(!dom_id.is_empty()).then_some(dom_id)}
            class=class
        >
            {children.map(|c| c())}
        </tr>
    }
}

#[component]
pub fn DataTableCellPrimitive(
    #[prop(optional)] children: Option<Children>,
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
            {children.map(|c| c())}
        </td>
    }
}

#[component]
pub fn DataTableFooterPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <tfoot data-rs-datatable-footer="" class=class>
            {children.map(|c| c())}
        </tfoot>
    }
}

#[component]
pub fn DataTablePaginationPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-datatable-pagination="" class=class>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DataTableEmptyPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-datatable-empty="" data-rs-empty="true" role="status" aria-live="polite" class=class>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DataTableLoadingPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-datatable-loading="" data-rs-loading="true" role="status" aria-live="polite" class=class>
            {children.map(|c| c())}
        </div>
    }
}
